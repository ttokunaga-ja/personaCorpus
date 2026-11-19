use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::OffsetName;
use clap::Parser;
use image::{GenericImageView, ImageEncoder};
use lopdf::{Document, Object, Stream, dictionary};
use rand::{Rng, SeedableRng};
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use xmltree::{Element, XMLNode};

#[derive(Parser)]
struct MediaArgs {
    file: PathBuf,
    output: PathBuf,
    #[arg(long, default_value_t = 110, value_parser=clap::value_parser!(u32).range(1..))]
    dpi: u32,
    #[arg(long, value_parser=clap::value_parser!(u32).range(1..))]
    max_pages: Option<u32>,
    #[arg(long, default_value_t = 0)]
    seed: u64,
    #[arg(long, default_value_t = 55, value_parser=clap::value_parser!(u8).range(1..=100))]
    quality: u8,
    #[arg(long)]
    gray: bool,
}
#[derive(Parser)]
struct FileArgs {
    file: PathBuf,
}
#[derive(Parser)]
struct StampArgs {
    file: PathBuf,
    #[arg(long)]
    author: String,
    #[arg(long)]
    date: String,
    #[arg(long)]
    modified_by: Option<String>,
    #[arg(long)]
    created: Option<String>,
    #[arg(long, default_value = "UTC")]
    timezone: String,
    #[arg(long)]
    producer: Option<String>,
    #[arg(long)]
    device: Option<String>,
}
pub fn run(command: &str, args: &[String]) -> i32 {
    let result = (|| -> Result<()> {
        let argv = std::iter::once(command.to_string()).chain(args.iter().cloned());
        if command == "stamp" {
            return stamp(StampArgs::try_parse_from(argv)?);
        }
        if command == "recalc" {
            return recalc(&FileArgs::try_parse_from(argv)?.file);
        }
        let allowed: &[&str] = match command {
            "render" => &["--dpi", "--max-pages"],
            "scan" => &["--seed", "--quality", "--gray"],
            _ => &[],
        };
        for argument in args {
            if argument == "--" {
                break;
            }
            if argument.starts_with("--") {
                let option = argument.split('=').next().unwrap();
                cli_check(
                    option == "--help" || allowed.contains(&option),
                    format!("unexpected argument {option}"),
                )?;
            }
        }
        let mut a = MediaArgs::try_parse_from(argv)?;
        a.file = absolute(&a.file)?;
        a.output = absolute(&a.output)?;
        validate_media(command, &a)?;
        match command {
            "render" => render(a),
            "office-pdf" => office_pdf(&a.file, &a.output),
            "tex" => tex(&a.file, &a.output),
            "scan" => scan(a),
            _ => bail!("unknown media command"),
        }
    })();
    match result {
        Ok(()) => 0,
        Err(e) => {
            if let Some(error) = e.downcast_ref::<clap::Error>() {
                let _ = error.print();
                error.exit_code()
            } else {
                eprintln!("{command}: {e:#}");
                1
            }
        }
    }
}
fn ext(path: &Path) -> String {
    path.extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
}
fn parent(path: &Path) -> &Path {
    path.parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
}
fn cli_check(condition: bool, message: impl Into<String>) -> Result<()> {
    if !condition {
        return Err(clap::Error::raw(clap::error::ErrorKind::InvalidValue, message.into()).into());
    }
    Ok(())
}
fn command_available(name: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let paths = std::env::var_os("PATH").unwrap_or_else(|| "/bin:/usr/bin".into());
    std::env::split_paths(&paths).any(|directory| {
        fs::metadata(directory.join(name))
            .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
    })
}
fn validate_media(command: &str, a: &MediaArgs) -> Result<()> {
    cli_check(a.file.is_file(), "input file does not exist")?;
    let formats: &[&str] = match command {
        "render" => &[
            "docx", "xlsx", "pptx", "pdf", "png", "jpg", "jpeg", "tif", "tiff", "bmp", "webp",
        ],
        "scan" => &["pdf", "png", "jpg", "jpeg", "tif", "tiff", "bmp", "webp"],
        "office-pdf" => &["docx", "xlsx", "pptx"],
        "tex" => &["tex"],
        _ => &[],
    };
    cli_check(
        formats.contains(&ext(&a.file).as_str()),
        "unsupported input format",
    )?;
    if command != "render" {
        cli_check(
            ext(&a.output) == "pdf" && a.file != a.output,
            "output must be a separate .pdf file",
        )?;
    }
    let converter = match command {
        "office-pdf" => Some("soffice"),
        "tex" => Some("lualatex"),
        _ => None,
    };
    if let Some(converter) = converter {
        cli_check(command_available(converter), format!("{converter} なし"))?;
    }
    Ok(())
}
fn absolute(path: &Path) -> Result<PathBuf> {
    fn resolve(path: &Path, depth: usize) -> Result<PathBuf> {
        ensure!(depth < 40, "too many symbolic links");
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
        let mut resolved = PathBuf::new();
        for part in path.components() {
            match part {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    resolved.pop();
                }
                std::path::Component::Normal(name) => {
                    resolved.push(name);
                    if fs::symlink_metadata(&resolved)
                        .is_ok_and(|metadata| metadata.file_type().is_symlink())
                    {
                        let target = fs::read_link(&resolved)?;
                        let target = if target.is_absolute() {
                            target
                        } else {
                            parent(&resolved).join(target)
                        };
                        resolved = resolve(&target, depth + 1)?;
                    }
                }
                other => resolved.push(other.as_os_str()),
            }
        }
        Ok(resolved)
    }
    resolve(path, 0)
}
fn file_uri(path: &Path) -> Result<String> {
    use std::os::unix::ffi::OsStrExt;
    let path = absolute(path)?;
    let mut uri = String::from("file://");
    for byte in path.as_os_str().as_bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~/".contains(byte) {
            uri.push(*byte as char);
        } else {
            use std::fmt::Write;
            write!(uri, "%{byte:02X}")?;
        }
    }
    Ok(uri)
}
fn output_pdf(source: &Path, output: &Path) -> Result<()> {
    ensure!(
        ext(output) == "pdf" && absolute(source)? != absolute(output)?,
        "output must be a separate .pdf file"
    );
    Ok(())
}
fn command_output(command: &mut Command) -> Result<Output> {
    let work = tempfile::tempdir()?;
    let stdout = work.path().join("stdout");
    let stderr = work.path().join("stderr");
    let mut child = command
        .stdout(fs::File::create(&stdout)?)
        .stderr(fs::File::create(&stderr)?)
        .spawn()
        .context("could not execute converter")?;
    let started = std::time::Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() > std::time::Duration::from_secs(180) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("converter timed out after 180 seconds");
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    Ok(Output {
        status,
        stdout: fs::read(stdout)?,
        stderr: fs::read(stderr)?,
    })
}
fn execute(command: &mut Command) -> Result<Output> {
    let out = command_output(command)?;
    ensure!(
        out.status.success(),
        "converter failed:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(out)
}
fn clean_pdf(source: &Path, output: &Path) -> Result<()> {
    let mut doc = Document::load(source)?;
    doc.trailer.remove(b"Info");
    if let Ok(root) = doc.trailer.get(b"Root").and_then(Object::as_reference) {
        doc.get_object_mut(root)?.as_dict_mut()?.remove(b"Metadata");
    }
    doc.prune_objects();
    fs::create_dir_all(parent(output))?;
    doc.save(output)?;
    Ok(())
}
fn office_pdf(source: &Path, output: &Path) -> Result<()> {
    ensure!(
        source.is_file() && ["docx", "xlsx", "pptx"].contains(&ext(source).as_str()),
        "input must be an existing DOCX, XLSX or PPTX file"
    );
    output_pdf(source, output)?;
    let work = tempfile::tempdir()?;
    let source = source.canonicalize()?;
    execute(
        Command::new("soffice")
            .arg(format!(
                "-env:UserInstallation={}",
                file_uri(&work.path().join("profile"))?
            ))
            .args(["--headless", "--convert-to", "pdf", "--outdir"])
            .arg(work.path())
            .arg(&source),
    )?;
    // LibreOffice replaces only the last extension: "説明_v0.2.docx" -> "説明_v0.2.pdf".
    let mut name = source.file_stem().unwrap().to_os_string();
    name.push(".pdf");
    let pdf = work.path().join(name);
    ensure!(pdf.is_file(), "LibreOffice conversion failed: no output");
    clean_pdf(&pdf, output)
}
fn rendered_pages(work: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(work)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .starts_with("page-")
                && ext(p) == "png"
        })
        .collect::<Vec<_>>();
    paths.sort_by_key(|p| {
        p.file_stem()
            .unwrap()
            .to_string_lossy()
            .rsplit('-')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap_or(0)
    });
    ensure!(!paths.is_empty(), "no pages rendered");
    Ok(paths)
}
fn render(a: MediaArgs) -> Result<()> {
    ensure!(a.dpi > 0 && a.max_pages != Some(0), "must be positive");
    let source = a.file.canonicalize()?;
    let extension = ext(&source);
    ensure!(
        [
            "docx", "xlsx", "pptx", "pdf", "png", "jpg", "jpeg", "tif", "tiff", "bmp", "webp"
        ]
        .contains(&extension.as_str()),
        "unsupported input format"
    );
    fs::create_dir_all(&a.output)?;
    if !["docx", "xlsx", "pptx", "pdf"].contains(&extension.as_str()) {
        image::open(&source)?;
        let target = a.output.join(source.file_name().unwrap());
        if target.canonicalize().ok().as_ref() != Some(&source) {
            fs::copy(&source, target)?;
        }
        return Ok(());
    }
    let work = tempfile::tempdir()?;
    let pdf = if extension == "pdf" {
        source.clone()
    } else {
        let p = work.path().join("document.pdf");
        office_pdf(&source, &p)?;
        p
    };
    let mut command = Command::new("pdftoppm");
    command.args(["-png", "-r", &a.dpi.to_string()]);
    if let Some(n) = a.max_pages {
        command.args(["-l", &n.to_string()]);
    }
    execute(command.arg(pdf).arg(work.path().join("page")))?;
    for page in rendered_pages(work.path())? {
        let name = page.file_stem().unwrap().to_string_lossy();
        let n = name.rsplit('-').next().unwrap();
        fs::copy(
            &page,
            a.output.join(format!(
                "{}-{n}.png",
                source.file_stem().unwrap().to_string_lossy()
            )),
        )?;
    }
    Ok(())
}
fn tex(source: &Path, output: &Path) -> Result<()> {
    ensure!(
        source.is_file() && ext(source) == "tex",
        "input must be an existing .tex file"
    );
    output_pdf(source, output)?;
    let source = source.canonicalize()?;
    let work = tempfile::tempdir()?;
    for directory in ["texmf-var", "texmf-cache"] {
        fs::create_dir(work.path().join(directory))?;
    }
    let mut previous = None;
    let rerun = Regex::new("(?i)Rerun to get|Please rerun|Label\\(s\\) may have changed")?;
    for attempt in 0..6 {
        let out = command_output(
            Command::new("lualatex")
                .args([
                    "-no-shell-escape",
                    "-interaction=nonstopmode",
                    "-halt-on-error",
                    "-file-line-error",
                    "-jobname=document",
                ])
                .arg(&source)
                .current_dir(work.path())
                .env(
                    "TEXINPUTS",
                    format!(
                        "{}:{}",
                        parent(&source).display(),
                        std::env::var("TEXINPUTS").unwrap_or_default()
                    ),
                )
                .env("TEXMFOUTPUT", work.path())
                .env("TEXMFVAR", work.path().join("texmf-var"))
                .env("TEXMFCACHE", work.path().join("texmf-cache")),
        )?;
        let log = fs::read_to_string(work.path().join("document.log"))
            .unwrap_or_else(|_| String::from_utf8_lossy(&out.stdout).into_owned());
        ensure!(
            out.status.success() && work.path().join("document.pdf").is_file(),
            "LuaLaTeX failed:\n{log}"
        );
        let mut state = BTreeMap::new();
        for entry in fs::read_dir(work.path())? {
            let p = entry?.path();
            if ["aux", "toc", "out", "lof", "lot"].contains(&ext(&p).as_str()) {
                state.insert(p.file_name().unwrap().to_owned(), fs::read(p)?);
            }
        }
        if !rerun.is_match(&log) && (state.is_empty() || previous.as_ref() == Some(&state)) {
            return clean_pdf(&work.path().join("document.pdf"), output);
        }
        ensure!(
            attempt < 5,
            "LuaLaTeX did not converge after 6 runs:\n{log}"
        );
        previous = Some(state);
    }
    unreachable!()
}
fn xml(raw: &[u8]) -> Result<Element> {
    let text = std::str::from_utf8(raw)?;
    let document = roxmltree::Document::parse(text)?;
    fn convert(node: roxmltree::Node<'_, '_>) -> Element {
        let mut e = Element::new(node.tag_name().name());
        e.namespace = node.tag_name().namespace().map(str::to_owned);
        e.prefix = node
            .tag_name()
            .namespace()
            .and_then(|ns| node.lookup_prefix(ns))
            .map(str::to_owned);
        let mut namespaces = xmltree::Namespace::empty();
        for ns in node.namespaces() {
            namespaces.put(ns.name().unwrap_or(""), ns.uri());
        }
        e.namespaces = Some(namespaces);
        for attribute in node.attributes() {
            let prefix = attribute.namespace().and_then(|ns| node.lookup_prefix(ns));
            let key = if let Some(prefix) = prefix {
                format!("{prefix}:{}", attribute.name())
            } else {
                attribute.name().to_string()
            };
            e.attributes.insert(key, attribute.value().into());
        }
        for child in node.children() {
            if child.is_element() {
                e.children.push(XMLNode::Element(convert(child)));
            } else if child.is_text() {
                e.children
                    .push(XMLNode::Text(child.text().unwrap_or_default().into()));
            } else if child.is_comment() {
                e.children
                    .push(XMLNode::Comment(child.text().unwrap_or_default().into()));
            }
        }
        e
    }
    Ok(convert(document.root_element()))
}
fn xml_bytes(doc: &Element) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    doc.write(&mut out)?;
    Ok(out)
}
fn elements_mut<F: FnMut(&mut Element)>(doc: &mut Element, f: &mut F) {
    f(doc);
    for child in &mut doc.children {
        if let XMLNode::Element(e) = child {
            elements_mut(e, f);
        }
    }
}
fn child<'a>(node: &'a Element, name: &str) -> Option<&'a Element> {
    node.children.iter().find_map(|c| {
        if let XMLNode::Element(e) = c {
            if e.name == name { Some(e) } else { None }
        } else {
            None
        }
    })
}
fn has_child(node: &Element, name: &str) -> bool {
    child(node, name).is_some()
}
fn zip_read(path: &Path) -> Result<Vec<(String, Vec<u8>, zip::CompressionMethod, Option<u32>)>> {
    let mut archive = zip::ZipArchive::new(fs::File::open(path)?)?;
    let mut entries = Vec::new();
    for i in 0..archive.len() {
        let mut e = archive.by_index(i)?;
        let mut data = Vec::new();
        e.read_to_end(&mut data)?;
        entries.push((e.name().to_string(), data, e.compression(), e.unix_mode()));
    }
    Ok(entries)
}
fn write_zip(
    entries: &[(String, Vec<u8>, zip::CompressionMethod, Option<u32>)],
    stamp: Option<zip::DateTime>,
) -> Result<Vec<u8>> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, data, compression, mode) in entries {
        let mut opts = zip::write::SimpleFileOptions::default().compression_method(*compression);
        if let Some(stamp) = stamp {
            opts = opts.last_modified_time(stamp);
        }
        if let Some(mode) = mode {
            opts = opts.unix_permissions(*mode);
        }
        writer.start_file(name, opts)?;
        writer.write_all(data)?;
    }
    Ok(writer.finish()?.into_inner())
}
fn replace_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut temporary = tempfile::NamedTempFile::new_in(parent(path))?;
    temporary.write_all(data)?;
    temporary
        .as_file()
        .set_permissions(fs::metadata(path)?.permissions())?;
    temporary.persist(path).map_err(|e| e.error)?;
    Ok(())
}
fn sheet_parts(
    entries: &[(String, Vec<u8>, zip::CompressionMethod, Option<u32>)],
) -> Result<HashMap<String, String>> {
    let data = |name: &str| -> Result<&[u8]> {
        Ok(&entries
            .iter()
            .find(|e| e.0 == name)
            .context("missing workbook part")?
            .1)
    };
    let rels = xml(data("xl/_rels/workbook.xml.rels")?)?;
    let mut targets = HashMap::new();
    for c in &rels.children {
        if let XMLNode::Element(e) = c {
            if let (Some(id), Some(target)) = (e.attributes.get("Id"), e.attributes.get("Target")) {
                let p = if target.starts_with('/') {
                    target.trim_start_matches('/').to_string()
                } else {
                    format!("xl/{target}")
                };
                targets.insert(id.clone(), p);
            }
        }
    }
    let book = xml(data("xl/workbook.xml")?)?;
    let mut result = HashMap::new();
    if let Some(sheets) = child(&book, "sheets") {
        for c in &sheets.children {
            if let XMLNode::Element(e) = c {
                let id = e
                    .attributes
                    .get("r:id")
                    .or_else(|| e.attributes.get("id"))
                    .context("sheet relationship missing")?;
                result.insert(
                    e.attributes
                        .get("name")
                        .context("sheet name missing")?
                        .clone(),
                    targets.get(id).context("sheet target missing")?.clone(),
                );
            }
        }
    }
    Ok(result)
}
fn recalc(source: &Path) -> Result<()> {
    ensure!(
        source.is_file() && !source.is_symlink() && ext(source) == "xlsx",
        "input must be an existing regular XLSX file"
    );
    let source = source.canonicalize()?;
    let original = zip_read(&source)?;
    let mut prepared = original.clone();
    for (name, raw, _, _) in &mut prepared {
        if name.starts_with("xl/worksheets/") && name.ends_with(".xml") {
            let mut doc = xml(raw)?;
            elements_mut(&mut doc, &mut |cell| {
                if cell.name == "c" && has_child(cell, "f") {
                    cell.children
                        .retain(|n| !matches!(n,XMLNode::Element(e) if e.name=="v"||e.name=="is"));
                    cell.attributes.shift_remove("t");
                }
            });
            *raw = xml_bytes(&doc)?;
        } else if name == "xl/workbook.xml" {
            let mut doc = xml(raw)?;
            if !has_child(&doc, "calcPr") {
                doc.children.push(XMLNode::Element(Element::new("calcPr")));
            }
            for n in &mut doc.children {
                if let XMLNode::Element(e) = n {
                    if e.name == "calcPr" {
                        for (k, v) in [
                            ("calcMode", "auto"),
                            ("fullCalcOnLoad", "1"),
                            ("forceFullCalc", "1"),
                        ] {
                            e.attributes.insert(k.into(), v.into());
                        }
                    }
                }
            }
            *raw = xml_bytes(&doc)?;
        }
    }
    let work = tempfile::tempdir()?;
    fs::create_dir(work.path().join("input"))?;
    let input = work.path().join("input").join(source.file_name().unwrap());
    fs::write(&input, write_zip(&prepared, None)?)?;
    execute(
        Command::new("soffice")
            .arg(format!(
                "-env:UserInstallation={}",
                file_uri(&work.path().join("profile"))?
            ))
            .args([
                "--headless",
                "--convert-to",
                "xlsx:Calc MS Excel 2007 XML",
                "--outdir",
            ])
            .arg(work.path())
            .arg(&input),
    )?;
    let computed = zip_read(&work.path().join(source.file_name().unwrap()))?;
    let old_sheets = sheet_parts(&original)?;
    let new_sheets = sheet_parts(&computed)?;
    let mut replacements = HashMap::new();
    for (name, part) in old_sheets {
        let mut doc = xml(&original
            .iter()
            .find(|e| e.0 == part)
            .context("sheet missing")?
            .1)?;
        let mut has_formula = false;
        elements_mut(&mut doc, &mut |e| {
            if e.name == "c" && has_child(e, "f") {
                has_formula = true;
            }
        });
        if !has_formula {
            continue;
        }
        let computed_part = new_sheets
            .get(&name)
            .context("recalculated sheet missing")?;
        let mut cached = xml(&computed
            .iter()
            .find(|e| &e.0 == computed_part)
            .context("recalculated sheet part missing")?
            .1)?;
        let mut cells = HashMap::new();
        elements_mut(&mut cached, &mut |e| {
            if e.name == "c" {
                if let Some(r) = e.attributes.get("r") {
                    cells.insert(r.clone(), e.clone());
                }
            }
        });
        let mut failure = None;
        elements_mut(&mut doc, &mut |cell| {
            if cell.name != "c" || !has_child(cell, "f") {
                return;
            }
            let value = cell.attributes.get("r").and_then(|r| cells.get(r));
            let Some(value) = value else {
                failure = Some("cached result missing");
                return;
            };
            let Some(v) = child(value, "v") else {
                failure = Some("cached result missing");
                return;
            };
            if value
                .attributes
                .get("t")
                .is_some_and(|s| s == "s" || s == "inlineStr")
            {
                failure = Some("unsupported formula cache type");
                return;
            }
            cell.children
                .retain(|n| !matches!(n,XMLNode::Element(e) if e.name=="v"||e.name=="is"));
            if let Some(t) = value.attributes.get("t") {
                cell.attributes.insert("t".into(), t.clone());
            } else {
                cell.attributes.shift_remove("t");
            }
            cell.children.push(XMLNode::Element(v.clone()));
        });
        if let Some(f) = failure {
            bail!("{f}");
        }
        replacements.insert(part, xml_bytes(&doc)?);
    }
    let mut archive = zip::ZipArchive::new(fs::File::open(&source)?)?;
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        if let Some(data) = replacements.remove(entry.name()) {
            let name = entry.name().to_string();
            let options = entry.options();
            writer.start_file(name, options)?;
            writer.write_all(&data)?;
        } else {
            writer.raw_copy_file(entry)?;
        }
    }
    replace_file(&source, &writer.finish()?.into_inner())
}
fn pdf_inherited(doc: &Document, mut id: lopdf::ObjectId, key: &[u8]) -> Result<Object> {
    loop {
        let d = doc.get_object(id)?.as_dict()?;
        if let Ok(v) = d.get(key) {
            return Ok(doc.dereference(v)?.1.clone());
        }
        id = d.get(b"Parent")?.as_reference()?;
    }
}
fn pdf_number(v: &Object) -> Result<f64> {
    Ok(match v {
        Object::Integer(n) => *n as f64,
        Object::Real(n) => *n as f64,
        _ => bail!("invalid PDF number"),
    })
}
fn cubic_weight(distance: f64) -> f64 {
    let x = distance.abs();
    if x <= 1. {
        1.5 * x * x * x - 2.5 * x * x + 1.
    } else if x < 2. {
        -0.5 * x * x * x + 2.5 * x * x - 4. * x + 2.
    } else {
        0.
    }
}
fn scan(a: MediaArgs) -> Result<()> {
    output_pdf(&a.file, &a.output)?;
    let extension = ext(&a.file);
    ensure!(
        ["pdf", "png", "jpg", "jpeg", "tif", "tiff", "bmp", "webp"].contains(&extension.as_str()),
        "input must be PDF or an image"
    );
    ensure!(
        (1..=100).contains(&a.quality),
        "quality must be between 1 and 100"
    );
    let work = tempfile::tempdir()?;
    let mut sizes = Vec::new();
    let pages = if extension == "pdf" {
        let input = Document::load(&a.file)?;
        for (_, id) in input.get_pages() {
            let media = pdf_inherited(&input, id, b"MediaBox")?;
            let rect = media.as_array()?;
            let size = (
                pdf_number(&rect[2])? - pdf_number(&rect[0])?,
                pdf_number(&rect[3])? - pdf_number(&rect[1])?,
            );
            let rotate = pdf_inherited(&input, id, b"Rotate")
                .ok()
                .and_then(|o| o.as_i64().ok())
                .unwrap_or(0);
            sizes.push(if rotate % 180 != 0 {
                (size.1, size.0)
            } else {
                size
            });
        }
        execute(
            Command::new("pdftoppm")
                .args(["-r", "180", "-png"])
                .arg(&a.file)
                .arg(work.path().join("page")),
        )?;
        rendered_pages(work.path())?
    } else {
        vec![a.file.clone()]
    };
    let mut rng = rand::rngs::StdRng::seed_from_u64(a.seed);
    let mut pdf = Document::with_version("1.5");
    let pages_id = pdf.new_object_id();
    let mut kids = Vec::new();
    for (index, path) in pages.iter().enumerate() {
        let mut input = image::open(path)?;
        if let Ok(exif) = exif::Reader::new()
            .read_from_container(&mut std::io::BufReader::new(fs::File::open(path)?))
        {
            if let Some(field) = exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                input = match field.value.get_uint(0).unwrap_or(1) {
                    2 => input.fliph(),
                    3 => input.rotate180(),
                    4 => input.flipv(),
                    5 => input.rotate90().fliph(),
                    6 => input.rotate90(),
                    7 => input.rotate270().fliph(),
                    8 => input.rotate270(),
                    _ => input,
                };
            }
        }
        let (width, height) = input.dimensions();
        let raw = input.to_rgba8();
        let angle = rng.random_range(-0.35f64..0.35).to_radians();
        let (sin, cos) = angle.sin_cos();
        let mut field = vec![0f32; 25];
        for v in &mut field {
            *v = rng.random_range(0.965f32..1.005);
        }
        let field =
            image::ImageBuffer::<image::Luma<f32>, Vec<f32>>::from_raw(5, 5, field).unwrap();
        let shade_field = image::imageops::resize(
            &field,
            width,
            height,
            image::imageops::FilterType::CatmullRom,
        );
        let channels = if a.gray { 1 } else { 3 };
        let mut pixels = Vec::with_capacity((width * height * channels) as usize);
        for y in 0..height {
            for x in 0..width {
                let xx = x as f64 - width as f64 / 2.;
                let yy = y as f64 - height as f64 / 2.;
                let sx = cos * xx - sin * yy + width as f64 / 2.;
                let sy = sin * xx + cos * yy + height as f64 / 2.;
                let mut rgb = [255f64; 3];
                if sx >= 0. && sy >= 0. && sx < width as f64 && sy < height as f64 {
                    let xi = sx.floor() as i64;
                    let yi = sy.floor() as i64;
                    rgb = [0.; 3];
                    for dy in -1..=2 {
                        for dx in -1..=2 {
                            let weight = cubic_weight(sx - (xi + dx) as f64)
                                * cubic_weight(sy - (yi + dy) as f64);
                            let p = raw
                                .get_pixel(
                                    (xi + dx).clamp(0, width as i64 - 1) as u32,
                                    (yi + dy).clamp(0, height as i64 - 1) as u32,
                                )
                                .0;
                            let alpha = p[3] as f64 / 255.;
                            for c in 0..3 {
                                rgb[c] += weight * (p[c] as f64 * alpha + 255. * (1. - alpha));
                            }
                        }
                    }
                    for v in &mut rgb {
                        *v = v.clamp(0., 255.).round();
                    }
                }
                let shade = shade_field.get_pixel(x, y).0[0] as f64;
                let noise = (-2. * rng.random::<f64>().max(1e-12).ln()).sqrt()
                    * (std::f64::consts::TAU * rng.random::<f64>()).cos()
                    * 1.3;
                if a.gray {
                    pixels.push(
                        ((rgb[0] * 0.299 + rgb[1] * 0.587 + rgb[2] * 0.114) * shade + noise)
                            .clamp(0., 255.) as u8,
                    );
                } else {
                    for value in rgb {
                        pixels.push((value * shade + noise).clamp(0., 255.) as u8);
                    }
                }
            }
        }
        let mut jpeg = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, a.quality).write_image(
            &pixels,
            width,
            height,
            if a.gray {
                image::ExtendedColorType::L8
            } else {
                image::ExtendedColorType::Rgb8
            },
        )?;
        let image_id=pdf.add_object(Stream::new(dictionary!{"Type"=>"XObject","Subtype"=>"Image","Width"=>width,"Height"=>height,"ColorSpace"=>if a.gray{"DeviceGray"}else{"DeviceRGB"},"BitsPerComponent"=>8,"Filter"=>"DCTDecode"},jpeg));
        let (w, h) = sizes
            .get(index)
            .copied()
            .unwrap_or((width as f64 * 72. / 180., height as f64 * 72. / 180.));
        let stream = pdf.add_object(Stream::new(
            dictionary! {},
            format!("q {w} 0 0 {h} 0 0 cm /Im0 Do Q\n").into_bytes(),
        ));
        let page=pdf.add_object(dictionary!{"Type"=>"Page","Parent"=>pages_id,"MediaBox"=>vec![Object::Integer(0),Object::Integer(0),Object::Real(w as f32),Object::Real(h as f32)],"Resources"=>dictionary!{"XObject"=>dictionary!{"Im0"=>image_id}},"Contents"=>stream});
        kids.push(Object::Reference(page));
    }
    let count = kids.len() as i64;
    pdf.objects.insert(
        pages_id,
        Object::Dictionary(dictionary! {"Type"=>"Pages","Kids"=>kids,"Count"=>count}),
    );
    let catalog = pdf.add_object(dictionary! {"Type"=>"Catalog","Pages"=>pages_id});
    pdf.trailer.set("Root", catalog);
    fs::create_dir_all(parent(&a.output))?;
    pdf.save(&a.output)?;
    Ok(())
}
fn tool_regex() -> Regex {
    Regex::new(r"(?i)python|openpyxl|reportlab|pypdf|pillow|matplotlib|codex|chatgpt|openai|imagegen|(?:lua|pdf|xe)?latex|(?:lua|pdf|xe)tex|tesseract|ghostscript|libreoffice|/Users/|/home/|/tmp/|scratch/").unwrap()
}
fn timestamp(value: &str, zone: &str) -> Result<DateTime<FixedOffset>> {
    if let Ok(t) = DateTime::parse_from_rfc3339(value) {
        return Ok(t);
    }
    let (local, abbreviation) = match value.rsplit_once(' ') {
        Some((local, suffix))
            if !suffix.is_empty() && suffix.bytes().all(|c| c.is_ascii_alphabetic()) =>
        {
            (local, Some(suffix))
        }
        _ => (value, None),
    };
    let naive = if value.len() == 10 {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")?
            .and_hms_opt(12, 0, 0)
            .unwrap()
    } else {
        [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%dT%H:%M",
        ]
        .iter()
        .find_map(|f| NaiveDateTime::parse_from_str(local, f).ok())
        .with_context(|| format!("invalid date: {value}"))?
    };
    let timezone: chrono_tz::Tz = zone.parse().context("invalid timezone")?;
    let local = timezone.from_local_datetime(&naive);
    let date = if let Some(abbreviation) = abbreviation {
        let date = local
            .single()
            .with_context(|| format!("invalid or ambiguous local time in {zone}: {value}"))?;
        ensure!(
            date.offset().abbreviation() == Some(abbreviation),
            "timezone abbreviation does not match {zone}: {value}"
        );
        date
    } else {
        local.earliest().context("invalid local time")?
    };
    Ok(date.fixed_offset())
}
fn iso(date: DateTime<FixedOffset>) -> String {
    date.with_timezone(&Utc)
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}
fn set_text_property(
    root: &mut Element,
    name: &str,
    prefix: Option<&str>,
    namespace: Option<&str>,
    value: &str,
) {
    let mut property = Element::new(name);
    property.prefix = prefix.map(str::to_owned);
    property.namespace = namespace.map(str::to_owned);
    property.children.push(XMLNode::Text(value.into()));
    root.children
        .retain(|n| !matches!(n,XMLNode::Element(e) if e.name==name));
    root.children.push(XMLNode::Element(property));
}
fn office_xml(
    raw: &[u8],
    name: &str,
    a: &StampArgs,
    date: DateTime<FixedOffset>,
    created: DateTime<FixedOffset>,
    extension: &str,
) -> Result<Vec<u8>> {
    let mut doc = xml(raw)?;
    let tool = tool_regex();
    if name == "docProps/core.xml" {
        let namespaces = doc.namespaces.get_or_insert_with(xmltree::Namespace::empty);
        for (k, v) in [
            ("dc", "http://purl.org/dc/elements/1.1/"),
            (
                "cp",
                "http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
            ),
            ("dcterms", "http://purl.org/dc/terms/"),
            ("xsi", "http://www.w3.org/2001/XMLSchema-instance"),
        ] {
            namespaces.put(k, v);
        }
        for (name, prefix, ns, value) in [
            (
                "creator",
                "dc",
                "http://purl.org/dc/elements/1.1/",
                a.author.clone(),
            ),
            (
                "lastModifiedBy",
                "cp",
                "http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
                a.modified_by.clone().unwrap_or(a.author.clone()),
            ),
            (
                "created",
                "dcterms",
                "http://purl.org/dc/terms/",
                iso(created),
            ),
            (
                "modified",
                "dcterms",
                "http://purl.org/dc/terms/",
                iso(date),
            ),
        ] {
            set_text_property(&mut doc, name, Some(prefix), Some(ns), &value);
        }
        for c in &mut doc.children {
            if let XMLNode::Element(e) = c {
                if e.name == "created" || e.name == "modified" {
                    e.attributes
                        .insert("xsi:type".into(), "dcterms:W3CDTF".into());
                }
            }
        }
        doc.children.retain(|n|!matches!(n,XMLNode::Element(e) if !["creator","lastModifiedBy","created","modified"].contains(&e.name.as_str()) && (e.name=="lastPrinted" || tool.is_match(&e.get_text().unwrap_or_default()))));
    } else if name == "docProps/app.xml" {
        let app = match extension {
            "docx" => "Microsoft Office Word",
            "xlsx" => "Microsoft Excel",
            _ => "Microsoft Office PowerPoint",
        };
        let ns = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";
        doc.namespaces
            .get_or_insert_with(xmltree::Namespace::empty)
            .put("", ns);
        set_text_property(&mut doc, "Application", None, Some(ns), app);
        set_text_property(&mut doc, "AppVersion", None, Some(ns), "16.0000");
        doc.children.retain(|n|!matches!(n,XMLNode::Element(e) if !["Application","AppVersion"].contains(&e.name.as_str()) && tool.is_match(&e.get_text().unwrap_or_default())));
    } else {
        elements_mut(&mut doc, &mut |e| {
            if e.name == "author" {
                for c in &mut e.children {
                    if let XMLNode::Text(text) = c {
                        if tool.is_match(text) {
                            *text = a.author.clone();
                        }
                    }
                }
            }
            let revision = [
                "comment",
                "commentEx",
                "ins",
                "del",
                "moveFrom",
                "moveTo",
                "rPrChange",
                "pPrChange",
                "sectPrChange",
                "tblPrChange",
                "tblGridChange",
                "trPrChange",
                "tcPrChange",
                "numberingChange",
                "cellIns",
                "cellDel",
                "cellMerge",
                "cm",
            ]
            .contains(&e.name.as_str())
                || e.name.ends_with("PrChange");
            for (k, v) in &mut e.attributes {
                let local = k.rsplit(':').next().unwrap();
                if (["author", "initials"].contains(&local)
                    || e.name == "cmAuthor" && local == "name")
                    && tool.is_match(v)
                {
                    *v = a.author.clone();
                }
                if revision && ["date", "dateUtc", "dt"].contains(&local) {
                    if timestamp(v, &a.timezone).is_ok_and(|old| old > date) {
                        *v = iso(date);
                    }
                }
            }
        });
    }
    xml_bytes(&doc)
}
fn stamp(a: StampArgs) -> Result<()> {
    let date = timestamp(&a.date, &a.timezone)?;
    let created = a
        .created
        .as_deref()
        .map(|v| timestamp(v, &a.timezone))
        .transpose()?
        .unwrap_or(date);
    ensure!(created <= date, "created must not be later than date");
    ensure!(a.file.is_file(), "file does not exist");
    let extension = ext(&a.file);
    let raw = fs::read(&a.file)?;
    let output = match extension.as_str() {
        "docx" | "xlsx" | "pptx" | "zip" => {
            ensure!(
                (1980..=2107).contains(&date.year()),
                "ZIP date must be in 1980..2107"
            );
            let stamp = zip::DateTime::from_date_and_time(
                date.year() as u16,
                date.month() as u8,
                date.day() as u8,
                date.hour() as u8,
                date.minute() as u8,
                (date.second() / 2 * 2) as u8,
            )
            .map_err(|_| anyhow::anyhow!("invalid ZIP date"))?;
            let mut entries = zip_read(&a.file)?;
            if extension != "zip" {
                for (name, data, _, _) in &mut entries {
                    if name.ends_with(".xml")
                        && ["docProps/", "word/", "xl/", "ppt/"]
                            .iter()
                            .any(|p| name.starts_with(p))
                    {
                        *data = office_xml(data, name, &a, date, created, &extension)?;
                    }
                }
            }
            write_zip(&entries, Some(stamp))?
        }
        "pdf" => stamp_pdf(&raw, &a, date, created)?,
        "png" | "jpg" | "jpeg" => stamp_image(&raw, &extension, date, created, &a)?,
        _ => raw,
    };
    replace_file(&a.file, &output)
}
fn pdf_string(value: &str) -> Object {
    Object::string_literal(if value.is_ascii() {
        value.as_bytes().to_vec()
    } else {
        let mut v = vec![254, 255];
        for c in value.encode_utf16() {
            v.extend(c.to_be_bytes());
        }
        v
    })
}
fn pdf_text(value: &Object) -> String {
    if let Ok(bytes) = value.as_str() {
        if bytes.starts_with(&[254, 255]) {
            return String::from_utf16_lossy(
                &bytes[2..]
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect::<Vec<_>>(),
            );
        }
        String::from_utf8_lossy(bytes).into_owned()
    } else {
        format!("{value:?}")
    }
}
fn recursive_text(element: &Element) -> String {
    element
        .children
        .iter()
        .map(|node| match node {
            XMLNode::Element(e) => recursive_text(e),
            XMLNode::Text(s) | XMLNode::CData(s) => s.clone(),
            _ => String::new(),
        })
        .collect()
}
fn stamp_pdf(
    raw: &[u8],
    a: &StampArgs,
    date: DateTime<FixedOffset>,
    created: DateTime<FixedOffset>,
) -> Result<Vec<u8>> {
    let mut doc = Document::load_mem(raw)?;
    let tool = tool_regex();
    let old = doc
        .trailer
        .get(b"Info")
        .and_then(Object::as_reference)
        .ok()
        .and_then(|id| doc.get_dictionary(id).ok())
        .cloned()
        .unwrap_or_default();
    let old_text = old
        .iter()
        .map(|(_, v)| pdf_text(v))
        .collect::<Vec<_>>()
        .join(" ");
    let producer = if let Some(producer) = &a.producer {
        producer.clone()
    } else if Regex::new("(?i)Word")?.is_match(&old_text) {
        "Microsoft® Word for Microsoft 365".into()
    } else if Regex::new("(?i)Excel")?.is_match(&old_text) {
        "Microsoft® Excel for Microsoft 365".into()
    } else if Regex::new("(?i)PowerPoint|Impress")?.is_match(&old_text) {
        "Microsoft® PowerPoint for Microsoft 365".into()
    } else {
        let pages = doc.get_pages().keys().copied().collect::<Vec<_>>();
        let text = match doc.extract_text(&pages) {
            Ok(text) => text,
            Err(_) => {
                // Poppler supports CMaps that lopdf cannot currently decode. A text
                // extraction failure must not turn a text PDF into a scanner PDF.
                let mut input = tempfile::NamedTempFile::new()?;
                input.write_all(raw)?;
                String::from_utf8_lossy(
                    &execute(Command::new("pdftotext").arg(input.path()).arg("-"))?.stdout,
                )
                .into_owned()
            }
        };
        if text.trim().is_empty() {
            "Canon iR-ADV C3830".into()
        } else {
            "Adobe PDF Library 23.0".into()
        }
    };
    let mut info = lopdf::Dictionary::new();
    let owned = Regex::new("(?i)date|author|creator|producer")?;
    for (key, value) in old.iter() {
        let text = pdf_text(value);
        if !tool.is_match(&text) && !owned.is_match(&String::from_utf8_lossy(key)) {
            info.set(key.clone(), value.clone());
        }
    }
    for (key, value) in [
        ("Author", a.author.clone()),
        (
            "CreationDate",
            created
                .with_timezone(&Utc)
                .format("D:%Y%m%d%H%M%SZ")
                .to_string(),
        ),
        (
            "ModDate",
            date.with_timezone(&Utc)
                .format("D:%Y%m%d%H%M%SZ")
                .to_string(),
        ),
        ("Producer", producer.clone()),
        ("Creator", producer.clone()),
    ] {
        info.set(key, pdf_string(&value));
    }
    let id = doc.add_object(info);
    doc.trailer.set("Info", id);
    let existing = doc
        .catalog()?
        .get(b"Metadata")
        .ok()
        .and_then(|o| o.as_reference().ok())
        .and_then(|id| doc.get_object(id).ok())
        .and_then(|o| o.as_stream().ok())
        .and_then(|s| {
            s.decompressed_content()
                .ok()
                .or_else(|| Some(s.content.clone()))
        });
    let mut xmp = if let Some(data) = existing {
        xml(&data)?
    } else {
        Element::new("xmpmeta")
    };
    xmp.prefix = Some("x".into());
    let ns = xmp.namespaces.get_or_insert_with(xmltree::Namespace::empty);
    for (prefix, uri) in [
        ("x", "adobe:ns:meta/"),
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("dc", "http://purl.org/dc/elements/1.1/"),
        ("xmp", "http://ns.adobe.com/xap/1.0/"),
        ("pdf", "http://ns.adobe.com/pdf/1.3/"),
    ] {
        ns.put(prefix, uri);
    }
    let owned = [
        "author",
        "creator",
        "creatortool",
        "producer",
        "creationdate",
        "createdate",
        "moddate",
        "modifydate",
        "modified",
        "metadatadate",
    ];
    elements_mut(&mut xmp, &mut |e| {
        if e.name == "Description" {
            e.attributes.retain(|key, value| {
                !owned.contains(&key.rsplit(':').next().unwrap().to_lowercase().as_str())
                    && !tool.is_match(value)
            });
            e.children.retain(|n|!matches!(n,XMLNode::Element(c) if owned.contains(&c.name.to_lowercase().as_str())||tool.is_match(&recursive_text(c))||c.attributes.values().any(|v|tool.is_match(v))));
        }
    });
    if !has_child(&xmp, "RDF") {
        let mut rdf = Element::new("RDF");
        rdf.prefix = Some("rdf".into());
        xmp.children.push(XMLNode::Element(rdf));
    }
    let rdf = xmp
        .children
        .iter_mut()
        .find_map(|c| {
            if let XMLNode::Element(e) = c {
                (e.name == "RDF").then_some(e)
            } else {
                None
            }
        })
        .unwrap();
    let mut description = Element::new("Description");
    description.prefix = Some("rdf".into());
    description
        .attributes
        .insert("rdf:about".into(), String::new());
    for (prefix, name, value) in [
        ("dc", "creator", a.author.clone()),
        ("xmp", "CreateDate", iso(created)),
        ("xmp", "ModifyDate", iso(date)),
        ("xmp", "MetadataDate", iso(date)),
        ("xmp", "CreatorTool", producer.clone()),
        ("pdf", "Producer", producer),
    ] {
        let mut e = Element::new(name);
        e.prefix = Some(prefix.into());
        if name == "creator" {
            let mut seq = Element::new("Seq");
            seq.prefix = Some("rdf".into());
            let mut li = Element::new("li");
            li.prefix = Some("rdf".into());
            li.children.push(XMLNode::Text(value));
            seq.children.push(XMLNode::Element(li));
            e.children.push(XMLNode::Element(seq));
        } else {
            e.children.push(XMLNode::Text(value));
        }
        description.children.push(XMLNode::Element(e));
    }
    rdf.children.push(XMLNode::Element(description));
    let metadata = doc.add_object(Stream::new(
        dictionary! {"Type"=>"Metadata","Subtype"=>"XML"},
        xml_bytes(&xmp)?,
    ));
    doc.catalog_mut()?.set("Metadata", metadata);
    doc.prune_objects();
    let mut out = Vec::new();
    doc.save_to(&mut out)?;
    Ok(out)
}
#[derive(Clone)]
struct TiffField {
    tag: u16,
    kind: u16,
    count: u32,
    data: Vec<u8>,
}
fn tiff_fields(raw: &[u8], offset: usize, little: bool) -> Result<Vec<TiffField>> {
    fn u16at(raw: &[u8], at: usize, little: bool) -> Result<u16> {
        let b: [u8; 2] = raw
            .get(at..at + 2)
            .context("invalid TIFF offset")?
            .try_into()?;
        Ok(if little {
            u16::from_le_bytes(b)
        } else {
            u16::from_be_bytes(b)
        })
    }
    fn u32at(raw: &[u8], at: usize, little: bool) -> Result<u32> {
        let b: [u8; 4] = raw
            .get(at..at + 4)
            .context("invalid TIFF offset")?
            .try_into()?;
        Ok(if little {
            u32::from_le_bytes(b)
        } else {
            u32::from_be_bytes(b)
        })
    }
    let count = u16at(raw, offset, little)?;
    let mut fields = Vec::new();
    for i in 0..count as usize {
        let at = offset + 2 + i * 12;
        let tag = u16at(raw, at, little)?;
        let kind = u16at(raw, at + 2, little)?;
        let count = u32at(raw, at + 4, little)?;
        let size = match kind {
            1 | 2 | 6 | 7 => 1,
            3 | 8 => 2,
            4 | 9 | 11 => 4,
            5 | 10 | 12 => 8,
            _ => bail!("unsupported TIFF field type"),
        };
        let length = (count as usize)
            .checked_mul(size)
            .context("invalid TIFF field size")?;
        let start = if length <= 4 {
            at + 8
        } else {
            u32at(raw, at + 8, little)? as usize
        };
        let mut data = raw
            .get(start..start.checked_add(length).context("invalid TIFF length")?)
            .context("invalid TIFF data")?
            .to_vec();
        if !little && size > 1 {
            let component = if kind == 5 || kind == 10 { 4 } else { size };
            for c in data.chunks_exact_mut(component) {
                c.reverse();
            }
        }
        fields.push(TiffField {
            tag,
            kind,
            count,
            data,
        });
    }
    Ok(fields)
}
fn tiff_ifd(fields: &[TiffField], offset: usize) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend((fields.len() as u16).to_le_bytes());
    let mut overflow = Vec::new();
    for field in fields {
        result.extend(field.tag.to_le_bytes());
        result.extend(field.kind.to_le_bytes());
        result.extend(field.count.to_le_bytes());
        if field.data.len() <= 4 {
            let mut bytes = [0; 4];
            bytes[..field.data.len()].copy_from_slice(&field.data);
            result.extend(bytes);
        } else {
            result.extend(
                ((offset + 2 + fields.len() * 12 + 4 + overflow.len()) as u32).to_le_bytes(),
            );
            overflow.extend(&field.data);
            if overflow.len() % 2 != 0 {
                overflow.push(0);
            }
        }
    }
    result.extend(0u32.to_le_bytes());
    result.extend(overflow);
    result
}
fn clean_tiff(
    raw: Option<&[u8]>,
    date: Option<DateTime<FixedOffset>>,
    device: Option<&str>,
) -> Result<Vec<u8>> {
    let mut fields = Vec::new();
    let mut capture = Vec::new();
    let mut gps = Vec::new();
    let mut interoperability = Vec::new();
    if let Some(raw) = raw {
        let raw = raw.strip_prefix(b"Exif\0\0").unwrap_or(raw);
        ensure!(raw.len() >= 8, "invalid EXIF");
        let little = &raw[..2] == b"II";
        ensure!(little || &raw[..2] == b"MM", "invalid TIFF byte order");
        let offset = if little {
            u32::from_le_bytes(raw[4..8].try_into()?)
        } else {
            u32::from_be_bytes(raw[4..8].try_into()?)
        } as usize;
        fields = tiff_fields(raw, offset, little)?;
        if let Some(field) = fields.iter().find(|f| f.tag == 34665) {
            let offset = u32::from_le_bytes(field.data[..4].try_into()?) as usize;
            capture = tiff_fields(raw, offset, little)?;
            if let Some(field) = capture.iter().find(|f| f.tag == 40965) {
                let offset = u32::from_le_bytes(field.data[..4].try_into()?) as usize;
                interoperability = tiff_fields(raw, offset, little)?;
            }
        }
        if date.is_none() {
            if let Some(field) = fields.iter().find(|f| f.tag == 34853) {
                let offset = u32::from_le_bytes(field.data[..4].try_into()?) as usize;
                gps = tiff_fields(raw, offset, little)?;
            }
        }
    }
    let tool = tool_regex();
    let keep = |field: &TiffField| {
        ![
            306, 36867, 36868, 36880, 36881, 36882, 37520, 37521, 37522, 37500, 37510,
        ]
        .contains(&field.tag)
            && !tool.is_match(&String::from_utf8_lossy(&field.data))
    };
    fields.retain(|f| keep(f) && f.tag != 34665 && f.tag != 513 && f.tag != 514);
    capture.retain(keep);
    fn ascii(tag: u16, value: &str) -> TiffField {
        let mut data = value.as_bytes().to_vec();
        data.push(0);
        TiffField {
            tag,
            kind: 2,
            count: data.len() as u32,
            data,
        }
    }
    if let Some(date) = date {
        fields.retain(|f| ![271, 272, 34853].contains(&f.tag));
        if let Some(device) = device {
            fields.push(ascii(271, device));
            fields.push(ascii(272, device));
        }
        let stamp = date.format("%Y:%m:%d %H:%M:%S").to_string();
        fields.push(ascii(306, &stamp));
        capture.push(ascii(36867, &stamp));
        capture.push(ascii(36868, &stamp));
    }
    fields.retain(|f| f.tag != 34853);
    capture.retain(|f| f.tag != 40965);
    fn pointer(tag: u16) -> TiffField {
        TiffField {
            tag,
            kind: 4,
            count: 1,
            data: vec![0; 4],
        }
    }
    if !interoperability.is_empty() {
        capture.push(pointer(40965));
    }
    if !capture.is_empty() {
        fields.push(pointer(34665));
    }
    if !gps.is_empty() {
        fields.push(pointer(34853));
    }
    fields.sort_by_key(|f| f.tag);
    capture.sort_by_key(|f| f.tag);
    gps.sort_by_key(|f| f.tag);
    let capture_offset = 8 + tiff_ifd(&fields, 8).len();
    let gps_offset = capture_offset
        + if capture.is_empty() {
            0
        } else {
            tiff_ifd(&capture, capture_offset).len()
        };
    let interop_offset = gps_offset
        + if gps.is_empty() {
            0
        } else {
            tiff_ifd(&gps, gps_offset).len()
        };
    for (tag, offset) in [(34665, capture_offset), (34853, gps_offset)] {
        if let Some(field) = fields.iter_mut().find(|f| f.tag == tag) {
            field.data = (offset as u32).to_le_bytes().to_vec();
        }
    }
    if let Some(field) = capture.iter_mut().find(|f| f.tag == 40965) {
        field.data = (interop_offset as u32).to_le_bytes().to_vec();
    }
    let mut out = b"II*\0\x08\0\0\0".to_vec();
    out.extend(tiff_ifd(&fields, 8));
    if !capture.is_empty() {
        out.extend(tiff_ifd(&capture, capture_offset));
    }
    if !gps.is_empty() {
        out.extend(tiff_ifd(&gps, gps_offset));
    }
    if !interoperability.is_empty() {
        out.extend(tiff_ifd(&interoperability, interop_offset));
    }
    Ok(out)
}
fn png_chunk(out: &mut Vec<u8>, kind: &[u8], data: &[u8]) {
    out.extend((data.len() as u32).to_be_bytes());
    out.extend(kind);
    out.extend(data);
    let mut hash = crc32fast::Hasher::new();
    hash.update(kind);
    hash.update(data);
    out.extend(hash.finalize().to_be_bytes());
}
fn stamp_image(
    raw: &[u8],
    extension: &str,
    date: DateTime<FixedOffset>,
    created: DateTime<FixedOffset>,
    args: &StampArgs,
) -> Result<Vec<u8>> {
    let device = args.device.as_deref();
    if extension == "png" {
        ensure!(raw.starts_with(b"\x89PNG\r\n\x1a\n"), "invalid PNG");
        let mut result = raw[..8].to_vec();
        let mut pos = 8;
        while pos < raw.len() {
            let size = u32::from_be_bytes(
                raw.get(pos..pos + 4)
                    .context("invalid PNG chunk")?
                    .try_into()?,
            ) as usize;
            let kind = raw.get(pos + 4..pos + 8).context("invalid PNG kind")?;
            let data = raw
                .get(pos + 8..pos + 8 + size)
                .context("invalid PNG data")?;
            if ![
                b"tEXt".as_slice(),
                b"zTXt".as_slice(),
                b"iTXt".as_slice(),
                b"tIME".as_slice(),
            ]
            .contains(&kind)
            {
                let cleaned = if kind == b"eXIf" {
                    clean_tiff(Some(data), None, None)?
                } else {
                    data.to_vec()
                };
                png_chunk(&mut result, kind, &cleaned);
            }
            if kind == b"IHDR" {
                for (key, value) in [
                    ("Author", args.author.clone()),
                    ("Creation Time", created.to_rfc3339()),
                    ("Modification Time", date.to_rfc3339()),
                ] {
                    // Uncompressed UTF-8 iTXt, with empty language and translated keyword.
                    png_chunk(
                        &mut result,
                        b"iTXt",
                        format!("{key}\0\0\0\0\0{value}").as_bytes(),
                    );
                }
            }
            pos += size + 12;
        }
        return Ok(result);
    }
    ensure!(raw.starts_with(b"\xff\xd8"), "invalid JPEG");
    let mut result = raw[..2].to_vec();
    let mut pos = 2;
    let mut stamped = false;
    let exif_segment = |data: Option<&[u8]>| -> Result<Vec<u8>> {
        let mut exif = b"Exif\0\0".to_vec();
        exif.extend(clean_tiff(data, Some(date), device)?);
        ensure!(exif.len() + 2 <= 65535, "EXIF too large");
        let mut out = vec![255, 225];
        out.extend(((exif.len() + 2) as u16).to_be_bytes());
        out.extend(exif);
        Ok(out)
    };
    while pos < raw.len() {
        let start = pos;
        ensure!(raw[pos] == 255, "invalid JPEG marker");
        while raw.get(pos) == Some(&255) {
            pos += 1;
        }
        let marker = *raw.get(pos).context("invalid JPEG marker")?;
        pos += 1;
        if marker == 0xda || marker == 0xd9 {
            if !stamped {
                result.extend(exif_segment(None)?);
            }
            result.extend(&raw[start..]);
            break;
        }
        if marker == 1 || (0xd0..=0xd7).contains(&marker) {
            result.extend(&raw[start..pos]);
            continue;
        }
        let size = u16::from_be_bytes(
            raw.get(pos..pos + 2)
                .context("invalid JPEG segment")?
                .try_into()?,
        ) as usize;
        ensure!(size >= 2, "invalid JPEG segment length");
        let data = raw
            .get(pos + 2..pos + size)
            .context("invalid JPEG segment data")?;
        pos += size;
        if marker == 0xfe {
            continue;
        }
        if marker == 0xe1 {
            if data.starts_with(b"Exif\0\0") && !stamped {
                result.extend(exif_segment(Some(data))?);
                stamped = true;
            }
            continue;
        }
        result.extend(&raw[start..pos]);
    }
    Ok(result)
}
