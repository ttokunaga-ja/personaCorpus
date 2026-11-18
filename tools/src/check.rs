use anyhow::{Result, anyhow, bail};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use clap::Parser;
use fancy_regex::Regex as Fancy;
use mailparse::MailHeaderMap;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, UNIX_EPOCH},
};
use unicode_casefold::UnicodeCaseFold;
use unicode_normalization::UnicodeNormalization;

const EXT: &str = "docx pdf xlsx csv tsv pptx txt md log eml html png jpg json xml wav mp3 m4a zip";
const TRACE: &str = r"(?i)(?<!\w)(?:kio(?:-eval)?|personacorpus|persona|corpus|fixture|content\s+spine|synthetic|fictional|not\s+a\s+real|artifact\s+id|source\s+id|batch|scope\s+(?=[\w-]*\d)[\w-]+|p\d{2}-[\w-]+|lorem\s+ipsum|placeholder)(?!\w)|/Users/|scratch/";
const SECRET: &str = r"(?:AKIA|ASIA)[0-9A-Z]{16}|(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{20,}|(?:xox[baprs]|xapp)-[A-Za-z0-9-]{10,}|(?:hvs\.|hvb\.|s\.)[A-Za-z0-9_-]{20,}|-----BEGIN (?:RSA |EC |DSA |OPENSSH |ENCRYPTED |PGP )?PRIVATE KEY(?: BLOCK)?-----|sk-[A-Za-z0-9_-]{20,}|eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+|/ASCII85Decode";
fn re(s: &str) -> Regex {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Regex>>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap();
    cache
        .entry(s.into())
        .or_insert_with(|| Regex::new(s).unwrap())
        .clone()
}
fn fancy(s: &str) -> Fancy {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Fancy>>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
    let mut cache = CACHE.lock().unwrap();
    cache
        .entry(s.into())
        .or_insert_with(|| Fancy::new(s).unwrap())
        .clone()
}
fn has(s: &str, t: &str) -> bool {
    fancy(s).is_match(t).unwrap_or(false)
}
fn epoch(t: std::time::SystemTime) -> f64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs_f64(),
        Err(e) => -e.duration().as_secs_f64(),
    }
}
fn strpath(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}
fn utf(raw: &[u8]) -> Result<String> {
    Ok(std::str::from_utf8(raw)?
        .trim_start_matches('\u{feff}')
        .into())
}
fn read(p: &Path) -> Result<String> {
    utf(&fs::read(p)?)
}
fn command(args: &[String]) -> Result<Vec<u8>> {
    use std::process::Stdio;
    let mut command = Command::new(&args[0]);
    if args[0] == "tesseract" {
        // Files already run in parallel. Keep each OCR worker on one CPU thread.
        command.env("OMP_THREAD_LIMIT", "1");
    }
    let mut child = command
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let out = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let err = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= std::time::Duration::from_secs(120) {
            let _ = child.kill();
            let _ = child.wait();
            bail!("{} timed out after 120 seconds", args[0]);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    };
    let stdout = out
        .join()
        .map_err(|_| anyhow!("command stdout reader failed"))??;
    let stderr = err
        .join()
        .map_err(|_| anyhow!("command stderr reader failed"))??;
    if !status.success() {
        bail!(
            "{} exit {}: {}",
            args[0],
            status.code().unwrap_or(1),
            String::from_utf8_lossy(&stderr)
                .chars()
                .take(240)
                .collect::<String>()
        );
    }
    Ok(stdout)
}
fn cmd(args: &[&str]) -> Result<Vec<u8>> {
    command(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}
fn which(name: &str) -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|p| p.join(name).is_file()))
}
fn html(text: &str) -> String {
    let stripped =
        re(r"(?is)<(?:script|style)\b[^>]*>.*?</(?:script|style)\s*>").replace_all(text, "");
    html_escape::decode_html_entities(&re(r"(?s)<[^>]*>").replace_all(&stripped, " ")).into_owned()
}
fn zip_parts(p: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut z = zip::ZipArchive::new(fs::File::open(p)?)?;
    let mut parts = BTreeMap::new();
    for i in 0..z.len() {
        let mut f = z.by_index(i)?;
        let mut raw = Vec::new();
        f.read_to_end(&mut raw)?;
        parts.insert(f.name().into(), raw);
    }
    Ok(parts)
}
fn xml(raw: &[u8]) -> Result<xmltree::Element> {
    Ok(xmltree::Element::parse(raw)?)
}
fn visit(e: &xmltree::Element, f: &mut impl FnMut(&xmltree::Element)) {
    f(e);
    for c in &e.children {
        if let xmltree::XMLNode::Element(c) = c {
            visit(c, f)
        }
    }
}
fn xml_text(e: &xmltree::Element) -> String {
    e.get_text().map(|s| s.into_owned()).unwrap_or_default()
}
fn units(e: &xmltree::Element) -> String {
    let mut out = String::new();
    visit(e, &mut |n| match n.name.as_str() {
        "t" | "v" | "delText" => out.push_str(&xml_text(n)),
        "br" => out.push('\n'),
        "tab" => out.push('\t'),
        _ => (),
    });
    out
}
fn office_text(p: &Path, ext: &str) -> Result<String> {
    let parts = zip_parts(p)?;
    let main = match ext {
        "docx" => "word/document.xml",
        "xlsx" => "xl/workbook.xml",
        _ => "ppt/presentation.xml",
    };
    for n in ["[Content_Types].xml", "_rels/.rels", main] {
        if !parts.contains_key(n) {
            bail!("missing or corrupt Office parts")
        }
    }
    let mut texts = Vec::new();
    for (n, raw) in &parts {
        if !n.ends_with(".xml") && !n.ends_with(".rels") {
            continue;
        }
        let e = xml(raw)?;
        if n == main
            && e.name
                != match ext {
                    "docx" => "document",
                    "xlsx" => "workbook",
                    _ => "presentation",
                }
        {
            bail!("incorrect Office root element")
        }
        if n.ends_with(".rels") {
            let base = Path::new(n)
                .parent()
                .and_then(Path::parent)
                .unwrap_or(Path::new(""));
            for c in &e.children {
                if let xmltree::XMLNode::Element(c) = c {
                    if c.attributes.get("TargetMode").map(String::as_str) == Some("External") {
                        continue;
                    }
                    if let Some(t) = c.attributes.get("Target") {
                        let joined = if t.starts_with('/') {
                            PathBuf::from(t.trim_start_matches('/'))
                        } else {
                            base.join(t)
                        };
                        let mut components = Vec::new();
                        for c in joined.components() {
                            match c {
                                std::path::Component::ParentDir => {
                                    components.pop();
                                }
                                std::path::Component::CurDir => (),
                                _ => components.push(c.as_os_str()),
                            }
                        }
                        let dest = components.iter().collect::<PathBuf>();
                        if !parts.contains_key(&strpath(&dest)) {
                            bail!("missing Office relationship target: {}", dest.display())
                        }
                    }
                }
            }
        }
        if n.starts_with("word/") || n.starts_with("ppt/") || n.starts_with("xl/") {
            let mut found = false;
            visit(&e, &mut |node| {
                if ["p", "si", "c"].contains(&node.name.as_str()) {
                    texts.push(units(node));
                    found = true
                }
            });
            if !found {
                let t = units(&e);
                if !t.is_empty() {
                    texts.push(t)
                }
            }
        }
    }
    Ok(texts.join(" "))
}
fn ocr(p: &Path, language: &str) -> Result<String> {
    Ok(
        String::from_utf8_lossy(&cmd(&["tesseract", &strpath(p), "stdout", "-l", language])?)
            .into_owned(),
    )
}
fn email_text(m: &mailparse::ParsedMail<'_>, parts: &mut Vec<String>) -> Result<()> {
    for h in &m.headers {
        parts.push(format!("{}: {}", h.get_key(), h.get_value()))
    }
    if let Some(n) = filename(m) {
        parts.push(n)
    }
    if m.subparts.is_empty() && m.ctype.mimetype.starts_with("text/") {
        let body = m.get_body()?;
        parts.push(if m.ctype.mimetype == "text/html" {
            html(&body)
        } else {
            body
        });
    }
    for c in &m.subparts {
        email_text(c, parts)?
    }
    Ok(())
}
fn filename(m: &mailparse::ParsedMail<'_>) -> Option<String> {
    m.get_content_disposition()
        .params
        .get("filename")
        .cloned()
        .or_else(|| m.ctype.params.get("name").cloned())
}
fn extract(p: &Path, ext: &str, language: Option<&str>) -> Result<(String, bool, bool)> {
    let path = strpath(p);
    if ["docx", "xlsx", "pptx"].contains(&ext) {
        return Ok((office_text(p, ext)?, false, false));
    }
    if ext == "pdf" {
        cmd(&["pdfinfo", &path])?;
        let mut text = utf(&cmd(&["pdftotext", &path, "-"])?)?;
        let mut pages: Vec<_> = text.split('\x0c').collect();
        if pages.last().is_some_and(|p| p.trim().is_empty()) {
            pages.pop();
        }
        let mut blanks: Vec<_> = pages
            .iter()
            .enumerate()
            .filter(|(_, p)| p.trim().is_empty())
            .map(|(i, _)| i + 1)
            .collect();
        if blanks.is_empty() && text.trim().is_empty() {
            blanks.push(1)
        }
        let image_only = !blanks.is_empty() && text.trim().is_empty();
        if let Some(lang) = language {
            if !blanks.is_empty() {
                let d = tempfile::tempdir()?;
                for i in &blanks {
                    let prefix = d.path().join(format!("page-{i}"));
                    cmd(&[
                        "pdftoppm",
                        "-f",
                        &i.to_string(),
                        "-l",
                        &i.to_string(),
                        "-singlefile",
                        "-r",
                        "150",
                        "-png",
                        &path,
                        &strpath(&prefix),
                    ])?;
                    text.push('\n');
                    text.push_str(&ocr(&prefix.with_extension("png"), lang)?);
                }
            }
        }
        return Ok((text, image_only, !blanks.is_empty() && language.is_none()));
    }
    if ["png", "jpg"].contains(&ext) {
        let reader = image::ImageReader::open(p)?.with_guessed_format()?;
        let want = if ext == "png" {
            image::ImageFormat::Png
        } else {
            image::ImageFormat::Jpeg
        };
        if reader.format() != Some(want) {
            bail!("image format mismatch")
        }
        reader.decode()?;
        return Ok((
            match language {
                Some(l) => ocr(p, l)?,
                None => String::new(),
            },
            true,
            language.is_none(),
        ));
    }
    if ["wav", "mp3", "m4a"].contains(&ext) {
        let info: serde_json::Value = serde_json::from_slice(&cmd(&[
            "ffprobe",
            "-v",
            "error",
            "-show_format",
            "-show_streams",
            "-show_frames",
            "-of",
            "json",
            &path,
        ])?)?;
        let formats = info["format"]["format_name"].as_str().unwrap_or("");
        let good = match ext {
            "wav" => vec!["wav"],
            "mp3" => vec!["mp3"],
            _ => vec!["mov", "mp4", "m4a"],
        };
        if !formats.split(',').any(|f| good.contains(&f))
            || !info["streams"]
                .as_array()
                .is_some_and(|s| s.iter().any(|s| s["codec_type"] == "audio"))
            || !info["frames"].as_array().is_some_and(|s| !s.is_empty())
        {
            bail!("audio header or frame decoding failed")
        }
        return Ok((String::new(), false, false));
    }
    if ext == "zip" {
        zip_parts(p)?;
        return Ok((String::new(), false, false));
    }
    let raw = fs::read(p)?;
    if ext == "eml" {
        let m = mailparse::parse_mail(&raw)?;
        if ["From", "To", "Subject"]
            .iter()
            .any(|k| m.headers.get_first_value(k).is_none_or(|v| v.is_empty()))
        {
            bail!("invalid email headers")
        }
        for field in [
            "From",
            "To",
            "Cc",
            "Bcc",
            "Reply-To",
            "Sender",
            "Resent-From",
            "Resent-To",
            "Resent-Cc",
            "Resent-Bcc",
            "Resent-Sender",
        ] {
            if let Some(value) = m.headers.get_first_value(field) {
                let addresses = mailparse::addrparse(&value)?;
                for address in addresses.iter() {
                    let valid = match address {
                        mailparse::MailAddr::Single(info) => info.addr.contains('@'),
                        mailparse::MailAddr::Group(info) => {
                            info.addrs.iter().all(|a| a.addr.contains('@'))
                        }
                    };
                    if !valid {
                        bail!("invalid email headers");
                    }
                }
            }
        }
        let mut parts = Vec::new();
        email_text(&m, &mut parts)?;
        return Ok((parts.join("\n"), false, false));
    }
    let mut text = utf(&raw)?;
    match ext {
        "json" => {
            serde_json::from_str::<serde_json::Value>(&text)?;
        }
        "xml" => {
            xml(&raw)?;
        }
        "csv" | "tsv" => {
            let mut r = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(if ext == "csv" { b',' } else { b'\t' })
                .from_reader(text.as_bytes());
            for row in r.records() {
                row?;
            }
        }
        "html" => text = html(&text),
        _ => (),
    }
    if text.contains('\0') {
        bail!("NUL in text")
    }
    Ok((text, false, false))
}
fn config_text(text: &str, source: bool) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    if !text.starts_with("---\n") {
        if source {
            bail!("source front matter missing")
        }
        return Ok(out);
    }
    let head = text[4..]
        .split_once("\n---")
        .map(|(s, _)| s)
        .unwrap_or(&text[4..]);
    for l in head.lines() {
        if let Some((k, v)) = l.split_once(':') {
            out.insert(
                k.trim().into(),
                if source {
                    v.trim().trim_matches(['\"', '\'']).into()
                } else {
                    v.trim().into()
                },
            );
        }
    }
    Ok(out)
}
fn source(
    root: &Path,
    rel: &str,
    chain: &mut Vec<String>,
) -> Result<(BTreeMap<String, String>, String)> {
    let rp = Path::new(rel);
    if rp.is_absolute()
        || rp
            .components()
            .any(|c| c == std::path::Component::ParentDir)
    {
        bail!("unsafe derived_from path")
    }
    if chain.iter().any(|r| r == rel) {
        bail!("derived_from cycle: {rel}")
    }
    let p = root.join(format!("{rel}.md"));
    for parent in p.ancestors() {
        if parent.starts_with(root)
            && parent
                .symlink_metadata()
                .is_ok_and(|m| m.file_type().is_symlink())
        {
            bail!("source path escapes root or contains symbolic link")
        }
    }
    let text = read(&p)?;
    let cfg = config_text(&text, true)?;
    let (_, body) = text[4..]
        .split_once("\n---\n")
        .ok_or_else(|| anyhow!("unclosed source front matter"))?;
    let mut body = body.to_string();
    if let Some(parent) = cfg.get("derived_from").filter(|s| !s.is_empty()) {
        chain.push(rel.into());
        body = source(root, parent, chain)?.1 + "\n" + &body;
        chain.pop();
    }
    if body.trim().is_empty() && cfg.get("kind").map(String::as_str) != Some("photo") {
        bail!("empty source body")
    }
    Ok((cfg, body))
}
fn normalized(text: &str) -> String {
    text.nfkc().case_fold().collect()
}
fn tokens(text: &str) -> Vec<String> {
    re(r"[\u3040-\u30ff\u3400-\u9fff]|[^\W_\u3040-\u30ff\u3400-\u9fff]+")
        .find_iter(text)
        .map(|m| m.as_str().into())
        .collect()
}
fn counts(text: &str) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    for w in tokens(&normalized(text)) {
        if !["年", "月", "日", "円"].contains(&w.as_str()) {
            *out.entry(w).or_default() += 1
        }
    }
    out
}
fn ungroup(text: &str) -> String {
    fancy(r"(?<![\d,.])\d{1,3}(?:,\d{3})+(?![\d,])")
        .replace_all(text, |c: &fancy_regex::Captures<'_>| c[0].replace(',', ""))
        .into_owned()
}
fn numbers(text: &str) -> BTreeSet<String> {
    let text = ungroup(&normalized(text));
    let mut out = BTreeSet::new();
    for c in fancy(r"(?<![\d.])(\d{4,})(?:\.(\d+))?(?![\d])")
        .captures_iter(&text)
        .flatten()
    {
        let whole = &c[1];
        if whole.len() == 4 && (1900..=2100).contains(&whole.parse::<u32>().unwrap_or(0)) {
            continue;
        }
        let frac = c
            .get(2)
            .map(|m| m.as_str().trim_end_matches('0'))
            .unwrap_or("");
        out.insert(format!(
            "{}{}{}",
            whole.trim_start_matches('0'),
            if frac.is_empty() { "" } else { "." },
            frac
        ));
    }
    out
}
fn cells(line: &str) -> Vec<String> {
    fancy(r"(?<!\\)\|")
        .split(line.trim().trim_matches('|'))
        .flatten()
        .map(|s| s.trim().to_string())
        .collect()
}
fn separator(cells: &[String]) -> bool {
    cells.iter().all(|s| re(r"^:?-+:?$").is_match(s))
}
fn source_body(text: &str) -> String {
    let text = re(r"`+\s*=[^`\n]*`+").replace_all(text, "");
    let lines: Vec<_> = text.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].contains('|') && i + 1 < lines.len() && separator(&cells(lines[i + 1])) {
            let head = cells(lines[i]);
            let mut rows = Vec::new();
            let mut j = i + 2;
            while j < lines.len() && lines[j].contains('|') && !lines[j].trim().is_empty() {
                rows.push(cells(lines[j]));
                j += 1;
            }
            let remove: HashSet<_> = head
                .iter()
                .enumerate()
                .filter(|(k, c)| {
                    ["行", "#", "No."].contains(&c.trim_matches(['`', ' ']))
                        && !rows.is_empty()
                        && rows
                            .iter()
                            .all(|r| r.len() == head.len() && re(r"^`?\d+`?$").is_match(&r[*k]))
                })
                .map(|(k, _)| k)
                .collect();
            for row in std::iter::once(&head).chain(rows.iter()) {
                result.push(
                    row.iter()
                        .enumerate()
                        .filter(|(k, _)| !remove.contains(k))
                        .map(|(_, c)| {
                            if c.trim_start_matches(['`', ' ']).starts_with('=') {
                                ""
                            } else {
                                c.as_str()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" | "),
                )
            }
            i = j;
        } else {
            if !separator(&cells(lines[i])) {
                result.push(if lines[i].trim().starts_with('=') {
                    String::new()
                } else {
                    lines[i].into()
                })
            }
            i += 1;
        }
    }
    result.join("\n")
}
fn data_lines(text: &str) -> String {
    let mut lines = Vec::new();
    let mut fenced = false;
    for l in text.lines() {
        if l.trim().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced || l.trim_start().starts_with('|') {
            lines.push(l)
        }
    }
    if lines.is_empty() {
        text.into()
    } else {
        lines.join("\n")
    }
}
fn source_reason(
    expected: &str,
    actual: &str,
    scan: bool,
    ext: &str,
    reviewed: bool,
    derived: bool,
) -> Option<String> {
    let table = ["xlsx", "csv", "tsv"].contains(&ext);
    let mut expected = source_body(expected);
    if !table {
        let fig = re(r"(?i)図|グラフ|軸|凡例|目盛|chart|axis|axes|legend|graph");
        expected = expected
            .lines()
            .filter(|l| !fig.is_match(l))
            .collect::<Vec<_>>()
            .join("\n")
    }
    let want = counts(&ungroup(&expected));
    let have = counts(&ungroup(actual));
    let sum: usize = want.values().sum();
    let overlap = if sum == 0 {
        1.0
    } else {
        want.iter()
            .map(|(k, v)| (*v).min(*have.get(k).unwrap_or(&0)))
            .sum::<usize>() as f64
            / sum as f64
    };
    if !scan && !derived {
        let vals = numbers(&if ["csv", "tsv"].contains(&ext) {
            data_lines(&expected)
        } else {
            expected.clone()
        });
        let actual = numbers(actual);
        let missing: Vec<_> = vals.difference(&actual).take(5).cloned().collect();
        let allowed = if table {
            (vals.len() / 10).max(usize::from(vals.len() >= 5))
        } else {
            vals.len() / 20
        };
        if vals.difference(&actual).count() > allowed {
            return Some(format!("missing source values: {}", missing.join(", ")));
        }
    }
    if table && !scan {
        return None;
    }
    let threshold = if scan {
        0.3
    } else if reviewed {
        0.6
    } else if derived {
        0.5
    } else if ext == "pptx" {
        0.7
    } else {
        0.8
    };
    if overlap < threshold {
        Some(format!("body token overlap {overlap:.3} < {threshold}"))
    } else {
        None
    }
}
fn email_reasons(expected: &str, p: &Path) -> Result<Vec<(String, String)>> {
    let raw = fs::read(p)?;
    let actual = mailparse::parse_mail(&raw)?;
    let mut issues = Vec::new();
    let header = expected.trim().split("\n\n").next().unwrap_or("");
    let mut headers = BTreeMap::<String, String>::new();
    let mut last = String::new();
    for line in header.lines() {
        if line.starts_with([' ', '\t']) {
            if let Some(v) = headers.get_mut(&last) {
                v.push(' ');
                v.push_str(line.trim());
            }
        } else if let Some((k, v)) = line.split_once(':') {
            last = k.to_lowercase();
            headers.insert(last.clone(), v.trim().into());
        } else {
            break;
        }
    }
    for field in ["Message-ID", "In-Reply-To", "References"] {
        let value = headers
            .get(&field.to_lowercase())
            .map(String::as_str)
            .unwrap_or("");
        if !re(r"^<[^<>\s]+>(?:\s+<[^<>\s]+>)*$").is_match(value) {
            continue;
        }
        let want = value.split_whitespace().collect::<Vec<_>>().join(" ");
        let have = actual
            .headers
            .get_first_value(field)
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if want != have {
            issues.push((
                "email-headers".into(),
                format!(
                    "{field}: manuscript {want} != email {}",
                    if have.is_empty() { "(missing)" } else { &have }
                ),
            ));
            break;
        }
    }
    let mut declared = BTreeSet::new();
    let mut listing = false;
    let declaration = re(
        r"(?i)^\s*(?:[-*]\s*)?(?:添付(?:ファイル)?(?:（実ファイル）)?|attachments?)\s*[:：](.*)$",
    );
    let file = re(&format!(
        r"(?i)^[^<>,\n]+\.(?:{})$",
        EXT.split_whitespace().collect::<Vec<_>>().join("|")
    ));
    for l in expected.lines() {
        let value = if let Some(c) = declaration.captures(l) {
            let v = c[1].trim().to_string();
            listing = v.is_empty();
            v
        } else if listing && re(r"^\s*[-*]\s+").is_match(l) {
            re(r"^\s*[-*]\s+").replace(l, "").trim().to_string()
        } else {
            listing = false;
            continue;
        };
        for item in value.split('、') {
            let item = item.trim().trim_matches('`');
            if file.is_match(item) {
                declared.insert(
                    Path::new(item)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let mut available = BTreeSet::new();
    for part in &actual.subparts {
        if let Some(name) = filename(part) {
            let body = part.get_body_raw()?;
            if !body.is_empty() || !part.subparts.is_empty() {
                available.insert(name);
            }
        }
    }
    let missing = declared.difference(&available).cloned().collect::<Vec<_>>();
    if !missing.is_empty() {
        issues.push((
            "email-attachments".into(),
            format!("missing or empty MIME attachments: {}", missing.join(", ")),
        ))
    }
    Ok(issues)
}
fn reviewed(p: &Path) -> bool {
    zip_parts(p).is_ok_and(|parts| {
        parts.iter().any(|(n, r)| {
            n.starts_with("word/")
                && n.ends_with(".xml")
                && xml(r).is_ok_and(|e| {
                    let mut found = false;
                    visit(&e, &mut |n| {
                        if [
                            "ins",
                            "del",
                            "moveFrom",
                            "moveTo",
                            "comment",
                            "commentRangeStart",
                            "commentReference",
                        ]
                        .contains(&n.name.as_str())
                        {
                            found = true
                        }
                    });
                    found
                })
        })
    })
}
fn formula(p: &Path) -> Result<Option<String>> {
    for (n, raw) in zip_parts(p)? {
        if !n.starts_with("xl/worksheets/") || !n.ends_with(".xml") {
            continue;
        }
        let e = xml(&raw)?;
        let mut reason = None;
        visit(&e, &mut |c| {
            if c.name != "c" || c.get_child("f").is_none() || reason.is_some() {
                return;
            }
            let label = format!(
                "{n}:{}",
                c.attributes.get("r").map(String::as_str).unwrap_or("?")
            );
            let v = c.get_child("v");
            let value = v.map(xml_text).unwrap_or_default();
            let kind = c.attributes.get("t").map(String::as_str);
            if v.is_none() || (value.is_empty() && kind != Some("str")) {
                reason = Some(format!("{label}: formula cached result missing"));
            } else if kind == Some("e")
                || ["#REF!", "#DIV/0!", "#VALUE!", "#NAME?", "#N/A"].contains(&value.as_str())
            {
                reason = Some(format!("{label}: formula error {value}"));
            }
        });
        if reason.is_some() {
            return Ok(reason);
        }
    }
    Ok(None)
}
fn xml_entries(e: &xmltree::Element, parent: &str, out: &mut Vec<(String, String)>) {
    let key = format!("{parent}:{}", e.name);
    for c in &e.children {
        match c {
            xmltree::XMLNode::Text(t) | xmltree::XMLNode::CData(t) => {
                if !t.trim().is_empty() {
                    out.push((key.clone(), t.trim().into()));
                }
            }
            xmltree::XMLNode::Element(c) => xml_entries(c, &key, out),
            _ => (),
        }
    }
    for (k, v) in &e.attributes {
        out.push((format!("{key}:{k}"), v.clone()));
    }
}
fn pdf_string(o: &lopdf::Object) -> String {
    match o {
        lopdf::Object::String(raw, _) => {
            if raw.starts_with(&[254, 255]) {
                String::from_utf16_lossy(
                    &raw[2..]
                        .chunks_exact(2)
                        .map(|c| u16::from_be_bytes([c[0], c[1]]))
                        .collect::<Vec<_>>(),
                )
            } else {
                String::from_utf8_lossy(raw).into_owned()
            }
        }
        lopdf::Object::Name(v) => String::from_utf8_lossy(v).into_owned(),
        _ => format!("{o:?}"),
    }
}
fn metadata_entries(p: &Path, ext: &str) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    if ["docx", "xlsx", "pptx"].contains(&ext) {
        for (n, raw) in zip_parts(p)? {
            if n.starts_with("docProps/") && n.ends_with(".xml") {
                xml_entries(&xml(&raw)?, &n, &mut out);
            } else if n.ends_with(".xml")
                && ["word/", "xl/", "ppt/"].iter().any(|s| n.starts_with(s))
            {
                visit(&xml(&raw)?, &mut |e| {
                    if e.name == "author" {
                        let t = xml_text(e);
                        if !t.is_empty() {
                            out.push((format!("{n}:author"), t));
                        }
                    }
                    for (k, v) in &e.attributes {
                        if ["author", "date", "initials"].contains(&k.as_str())
                            || e.name.contains("commentAuthor") && k == "name"
                        {
                            out.push((
                                format!("{n}:{}", if k == "name" { "author" } else { k }),
                                v.clone(),
                            ));
                        }
                    }
                });
            }
        }
    } else if ext == "pdf" {
        let doc = lopdf::Document::load(p)?;
        if let Ok(obj) = doc.trailer.get(b"Info") {
            let obj = match obj {
                lopdf::Object::Reference(id) => doc.get_object(*id)?,
                _ => obj,
            };
            if let Ok(dict) = obj.as_dict() {
                for (k, v) in dict {
                    out.push((String::from_utf8_lossy(k).into_owned(), pdf_string(v)));
                }
            }
        }
        if let Ok(root) = doc
            .trailer
            .get(b"Root")
            .and_then(lopdf::Object::as_reference)
            .and_then(|id| doc.get_object(id))
            .and_then(lopdf::Object::as_dict)
        {
            if let Ok(id) = root.get(b"Metadata").and_then(lopdf::Object::as_reference) {
                if let Ok(s) = doc.get_object(id).and_then(lopdf::Object::as_stream) {
                    let raw = s
                        .decompressed_content()
                        .unwrap_or_else(|_| s.content.clone());
                    xml_entries(&xml(&raw)?, "XMP", &mut out);
                }
            }
        }
    } else if ext == "png" || ext == "jpg" {
        if ext == "png" {
            let raw = fs::read(p)?;
            let mut i = 8;
            while i + 12 <= raw.len() {
                let n = u32::from_be_bytes(raw[i..i + 4].try_into()?) as usize;
                if i + 12 + n > raw.len() {
                    bail!("invalid PNG chunk")
                }
                let kind = &raw[i + 4..i + 8];
                let chunk = &raw[i + 8..i + 8 + n];
                if [b"tEXt".as_slice(), b"zTXt", b"iTXt"].contains(&kind) {
                    if let Some(z) = chunk.iter().position(|&b| b == 0) {
                        let key = String::from_utf8_lossy(&chunk[..z]).into_owned();
                        let payload = if kind == b"tEXt" {
                            chunk[z + 1..].to_vec()
                        } else if kind == b"zTXt" {
                            let mut out = Vec::new();
                            flate2::read::ZlibDecoder::new(&chunk[z + 2..])
                                .read_to_end(&mut out)?;
                            out
                        } else {
                            let compressed = chunk.get(z + 1) == Some(&1);
                            let mut start = z + 3;
                            for _ in 0..2 {
                                start += chunk[start..]
                                    .iter()
                                    .position(|&b| b == 0)
                                    .ok_or_else(|| anyhow!("invalid iTXt"))?
                                    + 1;
                            }
                            if compressed {
                                let mut out = Vec::new();
                                flate2::read::ZlibDecoder::new(&chunk[start..])
                                    .read_to_end(&mut out)?;
                                out
                            } else {
                                chunk[start..].to_vec()
                            }
                        };
                        if key.to_lowercase().contains("xmp") {
                            xml_entries(&xml(&payload)?, "XMP", &mut out)
                        } else {
                            out.push((key, String::from_utf8_lossy(&payload).into_owned()))
                        }
                    }
                }
                i += 12 + n;
            }
        }
        if ext == "jpg" {
            let raw = fs::read(p)?;
            let mut pos = 2;
            while pos + 1 < raw.len() && raw[pos] == 0xff {
                while pos < raw.len() && raw[pos] == 0xff {
                    pos += 1;
                }
                let Some(&marker) = raw.get(pos) else {
                    break;
                };
                pos += 1;
                if marker == 0xda || marker == 0xd9 {
                    break;
                }
                if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
                    continue;
                }
                let size = raw
                    .get(pos..pos + 2)
                    .ok_or_else(|| anyhow!("truncated JPEG segment"))?;
                let size = u16::from_be_bytes(size.try_into()?) as usize;
                if size < 2 {
                    bail!("invalid JPEG segment size");
                }
                let payload = raw
                    .get(pos + 2..pos + size)
                    .ok_or_else(|| anyhow!("truncated JPEG segment"))?;
                if marker == 0xe1 {
                    if let Some(xmp) = payload.strip_prefix(b"http://ns.adobe.com/xap/1.0/\0") {
                        xml_entries(&xml(xmp)?, "XMP", &mut out);
                    }
                }
                pos += size;
            }
        }
        let mut file = std::io::BufReader::new(fs::File::open(p)?);
        if let Ok(exif) = exif::Reader::new().read_from_container(&mut file) {
            for f in exif.fields() {
                let key = match f.tag {
                    exif::Tag::DateTime => "DateTime".into(),
                    exif::Tag::DateTimeOriginal => "DateTimeOriginal".into(),
                    exif::Tag::DateTimeDigitized => "DateTimeDigitized".into(),
                    _ => f.tag.to_string(),
                };
                let value = match &f.value {
                    exif::Value::Ascii(a) => a
                        .iter()
                        .map(|s| String::from_utf8_lossy(s).into_owned())
                        .collect::<Vec<_>>()
                        .join(" "),
                    _ => f.display_value().to_string(),
                };
                out.push((key, value));
            }
        }
    }
    Ok(out)
}
fn stamp(value: &str, zone: chrono_tz::Tz, offset: Option<&String>) -> Option<DateTime<Utc>> {
    let pdf = re(r"^(?:D:)?(\d{4})(\d{2})(\d{2})(\d{2})?(\d{2})?(\d{2})?");
    let mut v = value.to_string();
    if let Some(c) = pdf.captures(value) {
        let vals = (1..=6)
            .map(|i| {
                c.get(i)
                    .and_then(|m| m.as_str().parse::<u32>().ok())
                    .unwrap_or(0)
            })
            .collect::<Vec<_>>();
        let naive = NaiveDate::from_ymd_opt(vals[0] as i32, vals[1], vals[2])?
            .and_hms_opt(vals[3], vals[4], vals[5])?;
        if value.ends_with('Z') {
            return Some(naive.and_utc());
        }
        if let Some(c) = re(r"([+-])(\d{2})'?(\d{2})'?").captures(&value[c.get(0)?.end()..]) {
            let seconds = (c[2].parse::<i32>().ok()? * 3600 + c[3].parse::<i32>().ok()? * 60)
                * if &c[1] == "+" { 1 } else { -1 };
            return chrono::FixedOffset::east_opt(seconds)?
                .from_local_datetime(&naive)
                .single()
                .map(|t| t.with_timezone(&Utc));
        }
        return zone
            .from_local_datetime(&naive)
            .single()
            .map(|t| t.with_timezone(&Utc));
    }
    v = re(r"^(\d{4}):(\d{2}):(\d{2})")
        .replace(&v, "$1-$2-$3")
        .into_owned();
    if let Some(o) = offset {
        v.push_str(o)
    }
    if let Ok(t) = DateTime::parse_from_rfc3339(&v) {
        return Some(t.with_timezone(&Utc));
    }
    for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"] {
        if let Ok(t) = NaiveDateTime::parse_from_str(&v, fmt) {
            return zone
                .from_local_datetime(&t)
                .single()
                .map(|t| t.with_timezone(&Utc));
        }
    }
    None
}
fn metadata(
    p: &Path,
    ext: &str,
    cfg: &BTreeMap<String, String>,
    mtime: f64,
) -> Result<Option<String>> {
    let entries = metadata_entries(p, ext)?;
    let tool = re(
        r"(?i)python[- ]?(?:docx|pptx)?|openpyxl|reportlab|pypdf|pillow|matplotlib|codex|chatgpt|openai|imagegen|(?:lua|pdf|xe)?latex|(?:lua|pdf|xe)tex|tesseract|ghostscript|libreoffice",
    );
    let trace = re(&format!(
        r"(?i)/Users/|/home/|/tmp/|(?:[A-Z]:[\\/](?:Users|Temp)[\\/])|scratch/|{}",
        regex::escape(&strpath(&crate::base()))
    ));
    let user = std::env::var("USER").unwrap_or_default();
    let zone = cfg
        .get("timezone")
        .map(String::as_str)
        .unwrap_or("UTC")
        .parse::<chrono_tz::Tz>()?;
    for (key, value) in &entries {
        let identity = ["png", "jpg"].contains(&ext)
            || re(
                r"(?i)author|creator|lastModifiedBy|Application|producer|company|manager|initials",
            )
            .is_match(key);
        if identity
            && (tool.is_match(value)
                || trace.is_match(value)
                || user.len() > 2
                    && has(
                        &format!(r"(?i)(?<!\w){}(?!\w)", regex::escape(&user)),
                        value,
                    ))
        {
            return Ok(Some(format!(
                "production metadata {key}: {}",
                value.chars().take(180).collect::<String>()
            )));
        }
        if re(r"(?i)date|created|modified|creation|lastprinted|datetime").is_match(key) {
            let offset_key = match key.as_str() {
                "DateTime" => "OffsetTime",
                "DateTimeOriginal" => "OffsetTimeOriginal",
                "DateTimeDigitized" => "OffsetTimeDigitized",
                _ => "",
            };
            let offset = entries
                .iter()
                .find(|(k, _)| k == offset_key)
                .map(|(_, v)| v);
            if let Some(t) = stamp(value, zone, offset) {
                if let Some(period) = cfg.get("period") {
                    let (start, end) = period
                        .split_once("..")
                        .ok_or_else(|| anyhow!("invalid period"))?;
                    if t.date_naive() < NaiveDate::parse_from_str(start, "%Y-%m-%d")?
                        || t.date_naive() > NaiveDate::parse_from_str(end, "%Y-%m-%d")?
                    {
                        return Ok(Some(format!("{key}: outside persona period: {value}")));
                    }
                }
                if t.timestamp() as f64 > mtime {
                    return Ok(Some(format!("{key}: later than file mtime: {value}")));
                }
            }
        }
    }
    Ok(None)
}
#[derive(Default)]
struct Inspection {
    rel: String,
    issues: BTreeMap<String, String>,
    digest: Option<Vec<u8>>,
    grams: BTreeSet<u64>,
    skipped: Vec<String>,
}
fn fail(r: &mut Inspection, k: &str, v: impl ToString) {
    r.issues
        .entry(k.into())
        .or_insert_with(|| v.to_string().replace('\n', " ").chars().take(400).collect());
}
fn inspect(
    p: &Path,
    folder: &Path,
    cfg: &BTreeMap<String, String>,
    language: Option<&str>,
    source_root: &Path,
) -> Inspection {
    let rel = strpath(p.strip_prefix(folder).unwrap());
    let mut r = Inspection {
        rel: rel.clone(),
        ..Default::default()
    };
    let result = (|| -> Result<()> {
        let st = p.symlink_metadata()?;
        if st.file_type().is_symlink() {
            fail(&mut r, "format", "symbolic link");
            return Ok(());
        }
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let supported = EXT.split_whitespace().any(|s| s == ext);
        if !supported
            || p.strip_prefix(folder)?
                .components()
                .any(|s| s.as_os_str().to_string_lossy().starts_with('.'))
            || st.len() == 0
        {
            fail(
                &mut r,
                "format",
                "unsupported extension, hidden path, or zero bytes",
            )
        }
        if has(
            r"(?i)foundation|visual|source-art|full|(?<![a-z0-9])m1(?![a-z0-9])|primary|secondary|pdf_scan|pdf_text|txt_log|csv_tsv|html_eml|structured_text|domain_binary|imagegen",
            &rel,
        ) || has(
            r"(?i)(?<![a-z0-9])p\d{2}(?!\d)|-s\d{6}(?!\d)|(?<!\d)\d{5,}(?!\d)",
            &p.file_name().unwrap_or_default().to_string_lossy(),
        ) {
            fail(&mut r, "path", "production category or identifier in path")
        }
        let raw = fs::read(p)?;
        r.digest = Some(Sha256::digest(&raw).to_vec());
        let secret = regex::bytes::Regex::new(SECRET).unwrap();
        if secret.is_match(&raw) {
            fail(&mut r, "secret", "secret-like bytes or PDF ASCII85 filter")
        }
        let mtime = epoch(st.modified()?);
        let mut text = String::new();
        if supported && st.len() > 0 {
            match extract(p, &ext, language) {
                Err(e) => fail(&mut r, "opens", format!("Error: {e}")),
                Ok((body, scan, missing)) => {
                    text = body;
                    if ext == "xlsx" {
                        match formula(p) {
                            Ok(Some(reason)) => fail(&mut r, "formula", reason),
                            Err(e) => fail(&mut r, "opens", e),
                            _ => (),
                        }
                    }
                    match metadata(p, &ext, cfg, mtime) {
                        Ok(Some(reason)) => fail(&mut r, "metadata", reason),
                        Err(e) => fail(&mut r, "metadata", format!("Error: {e}")),
                        _ => (),
                    }
                    match source(source_root, &rel, &mut Vec::new()) {
                        Err(e) => fail(&mut r, "source", format!("Error: {e}")),
                        Ok((scfg, body)) => {
                            if ext == "eml" {
                                match email_reasons(&body, p) {
                                    Ok(issues) => {
                                        for (k, v) in issues {
                                            fail(&mut r, &k, v)
                                        }
                                    }
                                    Err(e) => fail(&mut r, "source", e),
                                }
                            }
                            let visual = ["png", "jpg"].contains(&ext.as_str())
                                || scfg.get("kind").is_some_and(|s| {
                                    s.split(':').any(|s| {
                                        ["photo", "handwriting", "chart", "diagram"].contains(&s)
                                    })
                                });
                            if !visual {
                                if missing {
                                    r.skipped.push("source(OCR)".into())
                                } else if let Some(reason) = source_reason(
                                    &body,
                                    &text,
                                    scan,
                                    &ext,
                                    ext == "docx" && reviewed(p),
                                    scfg.get("derived_from").is_some_and(|s| !s.is_empty()),
                                ) {
                                    fail(&mut r, "source", reason)
                                }
                            }
                        }
                    }
                    let photo = cfg.get("photo_dirs").is_some_and(|dirs| {
                        dirs.split(',').any(|s| {
                            !s.trim().is_empty()
                                && rel.starts_with(
                                    &(s.trim().trim_end_matches('/').to_string() + "/"),
                                )
                        })
                    });
                    if missing {
                        if !photo {
                            r.skipped.push("empty-scan".into())
                        }
                        r.skipped.extend(
                            ["trace(OCR)", "secret(OCR)", "duplicate(OCR)"].map(String::from),
                        );
                    }
                    if scan
                        && !missing
                        && !photo
                        && text.chars().filter(|c| !c.is_whitespace()).count() < 20
                    {
                        fail(
                            &mut r,
                            "empty-scan",
                            "OCR contains fewer than 20 non-whitespace characters",
                        )
                    }
                    if secret.is_match(text.as_bytes()) {
                        fail(&mut r, "secret", "secret-like extracted text")
                    }
                    let traced = rel.clone() + "\n" + &text;
                    let mut allowed = Vec::new();
                    for term in cfg
                        .get("allowed_terms")
                        .map(String::as_str)
                        .unwrap_or("")
                        .split(',')
                    {
                        let words = term.split_whitespace().collect::<Vec<_>>();
                        if words.len() >= 2 {
                            let pattern = format!(
                                r"(?i)(?<!\w){}(?!\w)",
                                words
                                    .iter()
                                    .map(|s| regex::escape(s))
                                    .collect::<Vec<_>>()
                                    .join(r"\s+")
                            );
                            for m in fancy(&pattern).find_iter(&traced).flatten() {
                                allowed.push((m.start(), m.end()));
                            }
                        }
                    }
                    let mut found = false;
                    for m in fancy(TRACE).find_iter(&traced).flatten() {
                        if !allowed
                            .iter()
                            .any(|(a, b)| *a <= m.start() && m.end() <= *b)
                        {
                            fail(&mut r, "trace", format!("production trace: {}", m.as_str()));
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if let Some(m)=re(r"本文への埋め込み|【[^】\n]{0,60}(?:削除・取り消し線|赤字の挿入|赤字追記)[^】\n]{0,20}】").find(&traced){fail(&mut r,"trace",format!("production trace: {}",m.as_str()));}
                    }
                }
            }
        }
        if let Some(period) = cfg.get("period") {
            let (start, end) = period
                .split_once("..")
                .ok_or_else(|| anyhow!("invalid period"))?;
            let day = DateTime::<Utc>::from(st.modified()?).date_naive();
            if day < NaiveDate::parse_from_str(start, "%Y-%m-%d")?
                || day > NaiveDate::parse_from_str(end, "%Y-%m-%d")?
            {
                fail(&mut r, "mtime", "outside persona period (UTC)")
            }
        }
        let normalized = re(r"\d+")
            .replace_all(&text.case_fold().collect::<String>(), "0")
            .into_owned();
        let words = tokens(&normalized);
        r.grams = words.windows(5).map(|w| shingle(w)).collect();
        Ok(())
    })();
    if let Err(e) = result {
        fail(&mut r, "opens", format!("Error: {e}"))
    }
    r
}
fn shingle(words: &[String]) -> u64 {
    use blake2::digest::{Update, VariableOutput};
    let mut digest = blake2::Blake2bVar::new(8).unwrap();
    digest.update(words.join(" ").as_bytes());
    let mut bytes = [0u8; 8];
    digest.finalize_variable(&mut bytes).unwrap();
    u64::from_be_bytes(bytes)
}
fn duplicates(results: &mut [Inspection], related: &BTreeMap<String, BTreeSet<String>>) {
    fn mark(
        results: &mut [Inspection],
        left: &[usize],
        right: &[usize],
        reason: &str,
        exact: bool,
        related: &BTreeMap<String, BTreeSet<String>>,
    ) {
        for (a, b) in [(left, right), (right, left)] {
            for &i in a {
                if results[i].issues.contains_key("duplicate") {
                    continue;
                }
                if let Some(&j) = b.iter().find(|&&j| {
                    i != j
                        && results[i].rel != results[j].rel
                        && (exact
                            || !(related
                                .get(&results[i].rel)
                                .is_some_and(|v| v.contains(&results[j].rel))
                                || related
                                    .get(&results[j].rel)
                                    .is_some_and(|v| v.contains(&results[i].rel))))
                }) {
                    let value = format!("{reason}: {}", results[j].rel);
                    fail(&mut results[i], "duplicate", value);
                }
            }
        }
    }
    let mut hashes = BTreeMap::<Vec<u8>, Vec<usize>>::new();
    let mut groups = BTreeMap::<BTreeSet<u64>, Vec<usize>>::new();
    for (i, r) in results.iter().enumerate() {
        if let Some(d) = &r.digest {
            hashes.entry(d.clone()).or_default().push(i)
        }
        if !r.grams.is_empty() {
            groups.entry(r.grams.clone()).or_default().push(i)
        }
    }
    for indices in hashes.values() {
        if indices.len() > 1 {
            mark(results, indices, indices, "identical bytes", true, related)
        }
    }
    for indices in groups.values() {
        if indices.len() > 1 {
            mark(
                results,
                indices,
                indices,
                "identical normalized shingles",
                false,
                related,
            )
        }
    }
    let mut frequency = HashMap::<&u64, usize>::new();
    for grams in groups.keys() {
        for g in grams {
            *frequency.entry(g).or_default() += 1
        }
    }
    let mut ordered = groups.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(g, _)| g.len());
    let mut index = HashMap::<&u64, Vec<usize>>::new();
    for (i, (grams, indices)) in ordered.iter().enumerate() {
        let n = grams.len();
        let mut prefix = grams.iter().collect::<Vec<_>>();
        prefix.sort_by_key(|g| (frequency[g], *g));
        prefix.truncate(n - (4 * n).div_ceil(5) + 1);
        let mut candidates = BTreeSet::new();
        for g in &prefix {
            if let Some(others) = index.get(g) {
                for &j in others {
                    if 5 * ordered[j].0.len() >= 4 * n {
                        candidates.insert(j);
                    }
                }
            }
        }
        for j in candidates {
            let other = ordered[j].0;
            let overlap = grams.intersection(other).count();
            if 5 * overlap >= 4 * (n + other.len() - overlap) {
                mark(
                    results,
                    indices,
                    ordered[j].1,
                    "normalized 5-word Jaccard >= 0.8",
                    false,
                    related,
                )
            }
        }
        for g in prefix {
            index.entry(g).or_default().push(i);
        }
    }
}
fn plan(
    p: &Path,
    relations: &mut BTreeMap<String, BTreeSet<String>>,
) -> Result<BTreeMap<String, String>> {
    let text = read(p)?;
    let mut result = BTreeMap::new();
    let mut indices = None;
    for l in text.lines() {
        if !l.trim().starts_with('|') {
            continue;
        }
        let cells = cells(l)
            .into_iter()
            .map(|c| c.trim_matches('`').to_string())
            .collect::<Vec<_>>();
        if ["パス", "日付", "内容", "関連"]
            .iter()
            .all(|s| cells.iter().any(|c| c == s))
        {
            indices = Some(
                ["パス", "日付", "内容", "関連", "先に要るもの"]
                    .map(|s| cells.iter().position(|c| c == s)),
            );
            continue;
        }
        let Some(idx) = indices else { continue };
        if separator(&cells) {
            continue;
        }
        if idx[..3].iter().flatten().any(|&i| i >= cells.len()) {
            bail!("malformed plan row")
        }
        let rel = &cells[idx[0].unwrap()];
        let date = &cells[idx[1].unwrap()];
        let content = &cells[idx[2].unwrap()];
        for &j in idx[3..].iter().flatten() {
            if let Some(c) = cells.get(j) {
                for val in re(r"[,、;；\n]+").split(c) {
                    let val = val.trim().trim_matches('`');
                    if !val.is_empty() {
                        relations.entry(rel.clone()).or_default().insert(val.into());
                    }
                }
            }
        }
        if content.starts_with("破棄：") || content.starts_with("成形待ち：") {
            continue;
        }
        if rel.is_empty()
            || Path::new(rel).is_absolute()
            || Path::new(rel)
                .components()
                .any(|c| c == std::path::Component::ParentDir)
        {
            bail!("unsafe plan path")
        }
        if result.insert(rel.clone(), date.clone()).is_some() {
            bail!("duplicate plan path: {rel}")
        }
    }
    if indices.is_none() {
        bail!("plan table not found")
    }
    Ok(result)
}
fn md_paths(folder: &Path) -> Vec<PathBuf> {
    let mut v = fs::read_dir(folder)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|s| s == "md"))
        .collect::<Vec<_>>();
    v.sort();
    v
}
fn relations(folder: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let mut out = BTreeMap::new();
    for p in md_paths(folder) {
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        if name.ends_with(".request.md") || name.ends_with(".result.md") || name == "facts.md" {
            continue;
        }
        let _ = plan(&p, &mut out);
    }
    out
}
#[derive(Parser, Debug)]
#[command(
    name = "check",
    about = "Read-only corpus checks. Reports contain failures only, never skipped checks."
)]
struct Args {
    persona: Option<String>,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    batch: Option<String>,
    #[arg(long)]
    report: Option<PathBuf>,
    #[arg(long)]
    self_test: bool,
    #[arg(long,default_value_os_t=crate::base().join("people"))]
    root: PathBuf,
    #[arg(long,default_value_os_t=crate::base().join("personas"))]
    personas: PathBuf,
    #[arg(long,default_value_os_t=crate::base().join("progress"))]
    progress: PathBuf,
    #[arg(long,default_value_os_t=crate::base().join("manifests"))]
    manifests: PathBuf,
    #[arg(long,default_value_os_t=crate::base().join("sources"))]
    sources: PathBuf,
}
fn names(a: &Args) -> Result<Vec<String>> {
    if a.all {
        if !a.root.is_dir() {
            bail!("root not found: {}", a.root.display())
        }
        let mut names = fs::read_dir(&a.root)?
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir() && !t.is_symlink()))
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|n| re(r"^p\d{2}-.+$").is_match(n))
            .collect::<Vec<_>>();
        names.sort();
        if names.is_empty() {
            bail!("no personas")
        }
        return Ok(names);
    }
    let name = a.persona.as_deref().unwrap_or("");
    let found = md_paths(&a.personas)
        .into_iter()
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .filter(|n| {
            re(r"^p\d{2}-.+$").is_match(n)
                && (n == name
                    || re(r"^p\d{2}$").is_match(name) && n.starts_with(&(name.to_string() + "-")))
        })
        .collect::<Vec<_>>();
    if found.len() != 1 {
        bail!("persona not found or ambiguous: {name}")
    }
    if a.root
        .join(&found[0])
        .symlink_metadata()
        .is_ok_and(|s| s.file_type().is_symlink())
    {
        bail!("persona folder is a symlink: {}", found[0])
    }
    Ok(found)
}
fn files(folder: &Path) -> Vec<PathBuf> {
    let mut paths = walkdir::WalkDir::new(folder)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.depth() > 0 && (!e.file_type().is_dir() || e.file_type().is_symlink()))
        .map(|e| e.into_path())
        .collect::<Vec<_>>();
    paths.sort();
    paths
}
fn tracked(folder: &Path) -> Result<BTreeSet<String>> {
    use std::os::unix::ffi::OsStringExt;
    let folder = folder
        .canonicalize()
        .unwrap_or_else(|_| folder.to_path_buf());
    let mut root = folder.clone();
    while !root.join(".git").exists() {
        if !root.pop() {
            return Ok(BTreeSet::new());
        }
    }
    let mut gitdir = root.join(".git");
    if gitdir.is_file() {
        let text = read(&gitdir)?;
        gitdir = root.join(text.trim().strip_prefix("gitdir: ").unwrap_or(text.trim()));
    }
    let index = gitdir.join("index");
    if !index.exists() {
        return Ok(BTreeSet::new());
    }
    let raw = fs::read(index)?;
    fn word(raw: &[u8], i: usize) -> Result<usize> {
        Ok(u32::from_be_bytes(
            raw.get(i..i + 4)
                .ok_or_else(|| anyhow!("truncated Git index"))?
                .try_into()?,
        ) as usize)
    }
    let version = word(&raw, 4)?;
    let count = word(&raw, 8)?;
    if raw.get(..4) != Some(b"DIRC") || ![2, 3, 4].contains(&version) {
        bail!("unsupported Git index")
    }
    let mut pos = 12;
    let mut previous = Vec::<u8>::new();
    let mut paths = BTreeSet::new();
    for _ in 0..count {
        let start = pos;
        let flags = u16::from_be_bytes(
            raw.get(pos + 60..pos + 62)
                .ok_or_else(|| anyhow!("truncated Git index"))?
                .try_into()?,
        );
        pos += 62;
        if flags & 0x4000 != 0 {
            pos += 2
        }
        let name = if version == 4 {
            let mut byte = *raw.get(pos).ok_or_else(|| anyhow!("truncated Git index"))?;
            pos += 1;
            let mut remove = (byte & 0x7f) as usize;
            while byte & 0x80 != 0 {
                byte = *raw.get(pos).ok_or_else(|| anyhow!("truncated Git index"))?;
                pos += 1;
                remove = ((remove + 1) << 7) + (byte & 0x7f) as usize;
            }
            if remove > previous.len() {
                bail!("invalid Git index prefix")
            }
            let end = pos
                + raw
                    .get(pos..)
                    .ok_or_else(|| anyhow!("truncated Git index"))?
                    .iter()
                    .position(|&b| b == 0)
                    .ok_or_else(|| anyhow!("unterminated Git index path"))?;
            let mut name = previous[..previous.len() - remove].to_vec();
            name.extend(&raw[pos..end]);
            pos = end + 1;
            name
        } else {
            let end = pos
                + raw
                    .get(pos..)
                    .ok_or_else(|| anyhow!("truncated Git index"))?
                    .iter()
                    .position(|&b| b == 0)
                    .ok_or_else(|| anyhow!("unterminated Git index path"))?;
            let name = raw[pos..end].to_vec();
            pos = start + (end + 1 - start).div_ceil(8) * 8;
            name
        };
        previous = name.clone();
        let path = root.join(std::ffi::OsString::from_vec(name));
        if let Ok(rel) = path.strip_prefix(&folder) {
            paths.insert(strpath(rel));
        }
    }
    while pos + 8 <= raw.len().saturating_sub(20) {
        let kind = &raw[pos..pos + 4];
        let size = word(&raw, pos + 4)?;
        pos += 8 + size;
        if kind == b"link" {
            bail!("split Git index unsupported; cannot certify untracked files")
        }
    }
    Ok(paths)
}
fn run_inner(a: &Args, selected: Option<&BTreeMap<String, BTreeSet<String>>>) -> Result<i32> {
    let names = match selected {
        Some(s) => s.keys().cloned().collect(),
        None => names(a)?,
    };
    let started = Instant::now();
    let mut failures = Vec::<[String; 4]>::new();
    let (mut passed, mut pending) = (0, 0);
    let mut failed_files = BTreeSet::new();
    let mut skips = BTreeMap::<String, usize>::new();
    let language = if which("tesseract") {
        let langs = utf(&cmd(&["tesseract", "--list-langs"])?)?;
        Some(if langs.lines().any(|s| s == "jpn") {
            "eng+jpn"
        } else {
            "eng"
        })
    } else {
        None
    };
    for name in names {
        let folder = a.root.join(&name);
        let cfg_path = a.personas.join(format!("{name}.md"));
        let cfg = if cfg_path.exists() {
            config_text(&read(&cfg_path)?, false)?
        } else {
            eprintln!("定義なし：{}（期間検査・許可語なし）", cfg_path.display());
            BTreeMap::new()
        };
        let paths = if let Some(s) = selected {
            s[&name].iter().map(|p| folder.join(p)).collect::<Vec<_>>()
        } else {
            files(&folder)
        };
        let target = a
            .batch
            .as_ref()
            .map(|b| {
                plan(
                    &a.progress.join(&name).join(format!("{b}.md")),
                    &mut BTreeMap::new(),
                )
            })
            .transpose()?;
        let mut local = Vec::<(String, String, String)>::new();
        if let Some(target) = &target {
            let existing = paths
                .iter()
                .map(|p| strpath(p.strip_prefix(&folder).unwrap()))
                .collect::<BTreeSet<_>>();
            for rel in target.keys().filter(|r| !existing.contains(*r)) {
                local.push((rel.clone(), "plan".into(), "planned file missing".into()))
            }
            let mut planned = target.keys().cloned().collect::<BTreeSet<_>>();
            for p in md_paths(&a.progress.join(&name)) {
                let filename = p.file_name().unwrap_or_default().to_string_lossy();
                if filename.ends_with(".request.md")
                    || filename.ends_with(".result.md")
                    || p.file_stem()
                        .is_some_and(|s| s == "facts" || a.batch.as_deref() == s.to_str())
                {
                    continue;
                }
                if let Ok(other) = plan(&p, &mut BTreeMap::new()) {
                    planned.extend(other.into_keys());
                }
            }
            let tracked = tracked(&folder)?;
            for rel in existing
                .difference(&planned)
                .filter(|r| !tracked.contains(*r))
            {
                local.push((
                    rel.clone(),
                    "plan".into(),
                    "untracked file outside batch plan".into(),
                ))
            }
        }
        use rayon::prelude::*;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(
                std::thread::available_parallelism()
                    .map(usize::from)
                    .unwrap_or(1)
                    .min(16),
            )
            .build()?;
        let mut results = pool.install(|| {
            paths
                .par_iter()
                .map(|p| inspect(p, &folder, &cfg, language, &a.sources.join(&name)))
                .collect::<Vec<_>>()
        });
        eprintln!(
            "{name}: {}/{} files, {:.1}s",
            results.len(),
            paths.len(),
            started.elapsed().as_secs_f64()
        );
        eprintln!("{name}: duplicate comparison");
        let mut related = relations(&a.progress.join(&name));
        for r in &results {
            if let Ok((cfg, _)) = source(&a.sources.join(&name), &r.rel, &mut Vec::new()) {
                if let Some(parent) = cfg.get("derived_from").filter(|s| !s.is_empty()) {
                    related
                        .entry(r.rel.clone())
                        .or_default()
                        .insert(parent.clone());
                }
            }
        }
        duplicates(&mut results, &related);
        for mut r in results {
            if target.as_ref().is_some_and(|t| !t.contains_key(&r.rel)) {
                continue;
            }
            for skip in &r.skipped {
                *skips.entry(skip.clone()).or_default() += 1
            }
            if let Some(target) = &target {
                // Plan dates may be written to the minute ("2026-03-06 18:42").
                let expected = stamp(&target[&r.rel], chrono_tz::UTC, None)
                    .or_else(|| {
                        ["%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M"]
                            .iter()
                            .find_map(|f| NaiveDateTime::parse_from_str(&target[&r.rel], f).ok())
                            .map(|d| d.and_utc())
                    })
                    .or_else(|| {
                        NaiveDate::parse_from_str(&target[&r.rel], "%Y-%m-%d")
                            .ok()
                            .and_then(|d| d.and_hms_opt(0, 0, 0))
                            .map(|d| d.and_utc())
                    });
                match expected {
                    None => {
                        r.issues
                            .insert("mtime".into(), "invalid plan timestamp".into());
                    }
                    Some(t) => {
                        let mtime = folder
                            .join(&r.rel)
                            .metadata()?
                            .modified()?
                            .duration_since(UNIX_EPOCH)?
                            .as_secs_f64();
                        if (mtime - t.timestamp() as f64).abs() >= 86400.0 {
                            r.issues.insert(
                                "mtime".into(),
                                "differs from plan by at least one day".into(),
                            );
                        }
                    }
                }
            }
            if r.issues.is_empty() {
                if r.skipped.is_empty() {
                    passed += 1
                } else {
                    pending += 1
                }
            }
            for (k, v) in r.issues {
                local.push((r.rel.clone(), k, v))
            }
        }
        local.sort();
        for (rel, kind, reason) in local {
            failed_files.insert((name.clone(), rel.clone()));
            let row = [name.clone(), rel, kind, reason];
            println!("{}", row.join("\t"));
            failures.push(row)
        }
    }
    if let Some(report) = &a.report {
        if let Some(parent) = report.parent() {
            fs::create_dir_all(parent)?
        }
        let mut w = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .terminator(csv::Terminator::Any(b'\n'))
            .from_path(report)?;
        for row in &failures {
            w.write_record(row)?
        }
        w.flush()?
    }
    for (k, c) in &skips {
        eprintln!("未実行：{k}: {c} files（tesseract なし）")
    }
    println!(
        "passed: {passed}, failed: {}, pending: {pending}, findings: {}, skipped OCR files: {}, elapsed: {:.2}s",
        failed_files.len(),
        failures.len(),
        skips.get("trace(OCR)").unwrap_or(&0),
        started.elapsed().as_secs_f64()
    );
    Ok(i32::from(!failures.is_empty() || !skips.is_empty()))
}
pub fn run(args: &[String]) -> i32 {
    run_selected(args, None)
}
pub fn run_selected(args: &[String], selected: Option<&BTreeMap<String, BTreeSet<String>>>) -> i32 {
    let a = match Args::try_parse_from(
        std::iter::once("check".to_string()).chain(args.iter().cloned()),
    ) {
        Ok(a) => a,
        Err(e) => {
            let code = if e.use_stderr() { 2 } else { 0 };
            let _ = e.print();
            return code;
        }
    };
    if a.self_test {
        let mut missing = Vec::new();
        for name in [
            "pdftotext",
            "pdfinfo",
            "pdftoppm",
            "tesseract",
            "soffice",
            "ffprobe",
            "lualatex",
            "fc-list",
        ] {
            if !which(name) {
                missing.push(name)
            }
        }
        if which("fc-list") && cmd(&["fc-list", ":lang=ja"]).is_ok_and(|o| o.is_empty()) {
            missing.push("日本語フォント")
        }
        if missing.is_empty() {
            println!("dependencies available");
            return 0;
        }
        println!("missing: {}", missing.join(", "));
        return 1;
    }
    if a.persona.is_some() == a.all || a.all && a.batch.is_some() {
        eprintln!("check: choose persona or --all; --batch requires persona");
        return 2;
    }
    match run_inner(&a, selected) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("check: Error: {e}");
            1
        }
    }
}
