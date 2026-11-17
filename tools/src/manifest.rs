use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use clap::{Args, Parser};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Args, Clone, Debug)]
pub struct Locations {
    #[arg(long, default_value_os_t=crate::base().join("people"))]
    pub root: PathBuf,
    #[arg(long, default_value_os_t=crate::base().join("personas"))]
    pub personas: PathBuf,
    #[arg(long, default_value_os_t=crate::base().join("progress"))]
    pub progress: PathBuf,
    #[arg(long, default_value_os_t=crate::base().join("manifests"))]
    pub manifests: PathBuf,
    #[arg(long, default_value_os_t=crate::base().join("sources"))]
    pub sources: PathBuf,
}
impl Locations {
    pub fn check_args(&self) -> Vec<String> {
        [
            ("root", &self.root),
            ("personas", &self.personas),
            ("progress", &self.progress),
            ("manifests", &self.manifests),
            ("sources", &self.sources),
        ]
        .into_iter()
        .flat_map(|(n, p)| [format!("--{n}"), p.to_string_lossy().into_owned()])
        .collect()
    }
}
#[derive(Parser)]
#[command(
    name = "manifest",
    about = "Inventory accepted HEAD files plus explicitly selected batch files."
)]
struct Options {
    persona: String,
    #[arg(long)]
    verify: bool,
    #[arg(long)]
    restore: bool,
    #[arg(long)]
    include: Vec<String>,
    #[command(flatten)]
    locations: Locations,
}
pub fn absolute(path: &Path) -> Result<PathBuf> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut out = PathBuf::new();
    for c in path.components() {
        use std::path::Component::*;
        match c {
            CurDir => {}
            ParentDir => {
                out.pop();
            }
            _ => out.push(c.as_os_str()),
        }
    }
    Ok(out)
}
pub fn resolve_path(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        return Ok(path.canonicalize()?);
    }
    let parent = path.parent().context("path has no parent")?;
    Ok(resolve_path(parent)?.join(path.file_name().context("path has no filename")?))
}
pub fn resolve(root: &Path, name: &str) -> Result<String> {
    if !root.is_dir() {
        bail!("root not found: {}", root.display())
    }
    let short = regex::Regex::new(r"^p\d{2}$")?.is_match(name);
    let pattern = regex::Regex::new(r"^p\d{2}-.+$")?;
    let mut names = Vec::new();
    for p in fs::read_dir(root)? {
        let p = p?;
        let n = p.file_name().to_string_lossy().into_owned();
        if p.file_type()?.is_dir()
            && pattern.is_match(&n)
            && (n == name || short && n.starts_with(&format!("{name}-")))
        {
            names.push(n)
        }
    }
    if names.len() != 1 {
        bail!("persona not found or ambiguous: {name}")
    }
    let n = names.remove(0);
    if root.join(&n).is_symlink() {
        bail!("persona folder is a symlink: {n}")
    }
    Ok(n)
}
pub fn accepted(folder: &Path) -> Result<BTreeSet<String>> {
    let folder = resolve_path(&absolute(folder)?)?;
    let mut directory = folder.as_path();
    while !directory.exists() {
        directory = directory
            .parent()
            .context("accepted inventory requires a Git repository")?
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if !output.status.success() {
        bail!("accepted inventory requires a Git repository")
    }
    let root = PathBuf::from(String::from_utf8(output.stdout)?.trim());
    let prefix = format!("{}/", folder.strip_prefix(&root)?.to_string_lossy());
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-tree", "-rz", "--name-only", "HEAD", "--", &prefix])
        .output()?;
    if !output.status.success() {
        bail!("accepted inventory requires Git HEAD")
    }
    let mut paths = BTreeSet::new();
    for raw in output.stdout.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let p = String::from_utf8(raw.to_vec())?;
        paths.insert(
            p.strip_prefix(&prefix)
                .context("invalid accepted prefix")?
                .to_string(),
        );
    }
    Ok(paths)
}
pub fn safe_relative(rel: &str) -> bool {
    !rel.is_empty()
        && !Path::new(rel).is_absolute()
        && !Path::new(rel).components().any(|c| {
            matches!(
                c,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
        && Path::new(rel)
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
            == rel
}
pub fn digest(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut buf = [0; 1024 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hash.update(&buf[..n])
    }
    Ok(format!("{:x}", hash.finalize()))
}
pub type Row = [String; 4];
pub fn entries(folder: &Path, paths: &BTreeSet<String>) -> Result<Vec<Row>> {
    let base = resolve_path(folder)?;
    let mut rows = Vec::new();
    for rel in paths {
        let path = folder.join(rel);
        if !resolve_path(&path)?.starts_with(&base) {
            bail!("path escapes persona: {}", path.display())
        }
        if path.is_symlink() {
            bail!("symlink cannot be manifested: {}", path.display())
        }
        let meta = path.metadata()?;
        let time: DateTime<Utc> = meta.modified()?.into();
        rows.push([
            rel.clone(),
            digest(&path)?,
            meta.len().to_string(),
            time.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        ]);
    }
    Ok(rows)
}
pub fn read_rows(path: &Path) -> Result<Vec<Row>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.records() {
        let row = row?;
        if row.len() != 4 {
            bail!("malformed manifest: {}", path.display())
        }
        rows.push(std::array::from_fn(|i| row[i].to_string()))
    }
    Ok(rows)
}
pub fn verify(folder: &Path, path: &Path, paths: &BTreeSet<String>) -> Result<Vec<Row>> {
    let rows = read_rows(path)?;
    if rows != entries(folder, paths)? {
        bail!(
            "manifest mismatch (paths, sha256, size or UTC mtime): {}",
            path.display()
        )
    }
    Ok(rows)
}
fn execute(a: Options) -> Result<()> {
    let n = resolve(&a.locations.root, &a.persona)?;
    let folder = a.locations.root.join(&n);
    let output = a.locations.manifests.join(format!("{n}.tsv"));
    if !a.include.is_empty() && (a.verify || a.restore) {
        bail!("--include is only for inventory generation")
    }
    let mut paths = accepted(&folder)?;
    if a.restore {
        let recorded = read_rows(&output)?;
        if recorded.iter().map(|r| r[0].clone()).collect::<Vec<_>>()
            != paths.iter().cloned().collect::<Vec<_>>()
        {
            bail!("manifest paths differ from accepted HEAD files")
        }
        for [rel, hash, _, stamp] in recorded {
            let target = folder.join(&rel);
            if target.is_symlink() || !resolve_path(&target)?.starts_with(resolve_path(&folder)?) {
                bail!("unsafe manifest path: {rel}")
            }
            if !target.is_file() || digest(&target)? != hash {
                bail!("cannot restore time, content differs from manifest: {rel}")
            }
            let stamp = DateTime::parse_from_rfc3339(&stamp)?;
            let time = filetime::FileTime::from_unix_time(
                stamp.timestamp(),
                stamp.timestamp_subsec_nanos(),
            );
            filetime::set_file_times(&target, time, time)?;
        }
        let rows = verify(&folder, &output, &paths)?;
        println!("restored {n}: {} files", rows.len());
    } else if a.verify {
        let rows = verify(&folder, &output, &paths)?;
        println!("verified {n}: {} files", rows.len());
    } else {
        for rel in a.include {
            if !safe_relative(&rel) {
                bail!("unsafe included path: {rel}")
            }
            paths.insert(rel);
        }
        let rows = entries(&folder, &paths)?;
        fs::create_dir_all(&a.locations.manifests)?;
        let mut temporary = tempfile::NamedTempFile::new_in(&a.locations.manifests)?;
        {
            let mut writer = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .terminator(csv::Terminator::Any(b'\n'))
                .from_writer(temporary.as_file_mut());
            for row in &rows {
                writer.write_record(row)?
            }
            writer.flush()?;
        }
        temporary.flush()?;
        temporary.persist(&output)?;
        println!("wrote {}: {} files", output.display(), rows.len());
    }
    Ok(())
}
pub fn run(args: &[String]) -> i32 {
    let a = match Options::try_parse_from(
        std::iter::once("manifest".to_string()).chain(args.iter().cloned()),
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
            eprintln!("{e}");
            1
        }
    }
}
