use crate::manifest::{self, Locations};
use anyhow::{Result, bail};
use chrono::DateTime;
use clap::Parser;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};
#[derive(Parser)]
#[command(
    name = "pack",
    about = "Validate all personas, then build deterministic gzip archives for the selection."
)]
struct Options {
    out: PathBuf,
    #[arg(long)]
    persona: Vec<String>,
    #[command(flatten)]
    locations: Locations,
}
fn octal(field: &mut [u8], value: u64) {
    let s = format!("{:0width$o}\0", value, width = field.len() - 1);
    field.copy_from_slice(s.as_bytes());
}
fn header(name: &str, size: u64, mtime: u64, mode: u64, kind: u8) -> [u8; 512] {
    let mut h = [0u8; 512];
    let ascii = name
        .chars()
        .map(|c| if c.is_ascii() { c as u8 } else { b'?' })
        .collect::<Vec<_>>();
    let n = ascii.len().min(100);
    h[..n].copy_from_slice(&ascii[..n]);
    octal(&mut h[100..108], mode);
    octal(&mut h[108..116], 0);
    octal(&mut h[116..124], 0);
    octal(&mut h[124..136], size);
    octal(&mut h[136..148], mtime);
    h[148..156].fill(b' ');
    h[156] = kind;
    h[257..263].copy_from_slice(b"ustar\0");
    h[263..265].copy_from_slice(b"00");
    // Python TarInfo leaves the device-number fields empty for regular/PAX members.
    let sum: u64 = h.iter().map(|v| *v as u64).sum();
    let check = format!("{sum:06o}\0 ");
    h[148..156].copy_from_slice(check.as_bytes());
    h
}
fn padded<W: Write>(out: &mut W, data: &[u8]) -> Result<usize> {
    out.write_all(data)?;
    let padding = (512 - data.len() % 512) % 512;
    out.write_all(&vec![0; padding])?;
    Ok(data.len() + padding)
}
fn pax_record(key: &str, value: &str) -> Vec<u8> {
    let body = format!(" {key}={value}\n");
    let mut n = body.len() + 1;
    loop {
        let next = body.len() + n.to_string().len();
        if next == n {
            break;
        }
        n = next;
    }
    format!("{n}{body}").into_bytes()
}
fn archive(
    folder: &Path,
    name: &str,
    rows: &[manifest::Row],
    destination: &Path,
) -> Result<BTreeSet<String>> {
    let file = fs::File::create(destination)?;
    let mut gzip = flate2::GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(file, flate2::Compression::new(9));
    let mut count = 0;
    let mut extensions = BTreeSet::new();
    for [rel, _, size, stamp] in rows {
        let full = format!("{name}/{rel}");
        let size: u64 = size.parse()?;
        let mtime = DateTime::parse_from_rfc3339(stamp)?.timestamp();
        let mut records = Vec::new();
        if !full.is_ascii() || full.len() > 100 {
            records.extend(pax_record("path", &full));
        }
        let large = size >= 8u64.pow(11);
        if large {
            records.extend(pax_record("size", &size.to_string()));
        }
        let large_time = mtime < 0 || mtime as u64 >= 8u64.pow(11);
        if large_time {
            records.extend(pax_record("mtime", &mtime.to_string()));
        }
        if !records.is_empty() {
            gzip.write_all(&header("././@PaxHeader", records.len() as u64, 0, 0, b'x'))?;
            count += 512;
            count += padded(&mut gzip, &records)?;
        }
        gzip.write_all(&header(
            &full,
            if large { 0 } else { size },
            if large_time { 0 } else { mtime as u64 },
            0o644,
            b'0',
        ))?;
        count += 512;
        let path = folder.join(rel);
        let mut file = fs::File::open(&path)?;
        let bytes = std::io::copy(&mut file, &mut gzip)?;
        if bytes != size {
            bail!("file size changed while packing: {}", path.display())
        }
        let padding = (512 - size % 512) % 512;
        gzip.write_all(&vec![0; padding as usize])?;
        count += size as usize + padding as usize;
        extensions.insert(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase(),
        );
    }
    gzip.write_all(&[0; 1024])?;
    count += 1024;
    let padding = (10240 - count % 10240) % 10240;
    gzip.write_all(&vec![0; padding])?;
    gzip.finish()?;
    Ok(extensions)
}
fn execute(a: Options) -> Result<()> {
    let mut accepted = BTreeMap::new();
    let names = manifest::accepted(&a.locations.root)?
        .into_iter()
        .map(|s| s.split('/').next().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    if names.is_empty() {
        bail!("no accepted personas")
    }
    let selected = if a.persona.is_empty() {
        names.clone()
    } else {
        a.persona
            .iter()
            .map(|n| manifest::resolve(&a.locations.root, n))
            .collect::<Result<BTreeSet<_>>>()?
    };
    if !selected.is_subset(&names) {
        bail!("selected persona has no accepted files")
    }
    let out = manifest::resolve_path(&manifest::absolute(&a.out)?)?;
    let base = crate::base().canonicalize()?;
    if out.starts_with(&base) && !out.starts_with(base.join("scratch")) {
        bail!("output must be outside repository or under scratch/")
    }
    let root = manifest::resolve_path(&a.locations.root)?;
    if out.starts_with(&root) || root.starts_with(&out) {
        bail!("output must not overlap corpus root")
    }
    if out.exists() && fs::read_dir(&out)?.next().is_some() {
        bail!("output directory must be empty")
    }
    for n in &names {
        accepted.insert(n.clone(), manifest::accepted(&a.locations.root.join(n))?);
    }
    let mut args = a.locations.check_args();
    args.push("--all".to_string());
    if crate::check::run_selected(&args, Some(&accepted)) != 0 {
        bail!("check --all failed or OCR incomplete")
    }
    let mut inventories = BTreeMap::new();
    for n in &names {
        inventories.insert(
            n.clone(),
            manifest::verify(
                &a.locations.root.join(n),
                &a.locations.manifests.join(format!("{n}.tsv")),
                &accepted[n],
            )?,
        );
    }
    let staging = tempfile::Builder::new().prefix("corpus-pack-").tempdir()?;
    let mut sums = String::new();
    let mut summary = String::new();
    for name in &selected {
        let rows = &inventories[name];
        let dest = staging.path().join(format!("{name}.tar.gz"));
        let extensions = archive(&a.locations.root.join(name), name, rows, &dest)?;
        sums.push_str(&format!("{}  {name}.tar.gz\n", manifest::digest(&dest)?));
        summary.push_str(&format!(
            "- {name}: {} files; formats: {}\n",
            rows.len(),
            if extensions.is_empty() {
                "none".to_string()
            } else {
                extensions.into_iter().collect::<Vec<_>>().join(", ")
            }
        ));
    }
    fs::write(staging.path().join("SHA256SUMS"), sums)?;
    fs::write(
        staging.path().join("README.md"),
        format!(
            "# Persona corpus\n\nThis distribution consists entirely of synthetic data representing personal and work folders. It does not describe real people or customer records.\n\nEach .tar.gz archive contains one persona folder. Files retain the UTC modification times recorded in the verified manifests. Verify archive hashes using SHA256SUMS.\n\n{summary}"
        ),
    )?;
    for name in &names {
        manifest::verify(
            &a.locations.root.join(name),
            &a.locations.manifests.join(format!("{name}.tsv")),
            &accepted[name],
        )?;
    }
    fs::create_dir_all(&out)?;
    let mut outputs = fs::read_dir(staging.path())?.collect::<std::io::Result<Vec<_>>>()?;
    outputs.sort_by_key(|e| e.file_name());
    for file in outputs {
        fs::copy(file.path(), out.join(file.file_name()))?;
    }
    Ok(())
}
pub fn run(args: &[String]) -> i32 {
    let a = match Options::try_parse_from(
        std::iter::once("pack".to_string()).chain(args.iter().cloned()),
    ) {
        Ok(a) => a,
        Err(e) => {
            let code = e.exit_code();
            let _ = e.print();
            return code;
        }
    };
    match execute(a) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("pack: {e}");
            1
        }
    }
}
