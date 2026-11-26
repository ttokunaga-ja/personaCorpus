use anyhow::{Context, Result, anyhow, bail};
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
pub(crate) const TRACE: &str = concat!(
    r"(?i)(?<!\w)(?:kio(?:-eval)?|personacorpus|persona|corpus|fixture|content\s+spine|synthetic|fictional|not\s+a\s+real|artifact\s+id|source\s+id|batch|",
    // A versioned support scope cites the service document in customer correspondence.
    r"(?!(?<=\bsupport )scope[ \t]+(?-i:[A-Z]+-[A-Z]+-[0-9]{8}-SVC-[0-9]{2})[ \t]+v[0-9]+\.[0-9]+\b)",
    r"scope\s+(?=[\w-]*\d)[\w-]+|p\d{2}-[\w-]+|lorem\s+ipsum|placeholder)(?!\w)|/Users/|scratch/",
);
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
pub(crate) fn traces<'a>(
    text: &'a str,
    allowed_terms: Option<&str>,
) -> impl Iterator<Item = fancy_regex::Match<'a>> {
    static PATTERN: std::sync::LazyLock<Fancy> =
        std::sync::LazyLock::new(|| Fancy::new(TRACE).unwrap());
    let mut allowed = Vec::new();
    for term in allowed_terms.unwrap_or("").split(',') {
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
            allowed.extend(
                fancy(&pattern)
                    .find_iter(text)
                    .flatten()
                    .map(|m| (m.start(), m.end())),
            );
        }
    }
    PATTERN.find_iter(text).flatten().filter(move |m| {
        !allowed
            .iter()
            .any(|(a, b)| *a <= m.start() && m.end() <= *b)
    })
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
// A saved page may add a stylesheet and a text-free decorative seal. Compare
// everything else literally, including link destinations and agenda item IDs.
fn matches_styled_html(manuscript: &str, bytes: &[u8]) -> bool {
    let Ok(saved) = std::str::from_utf8(bytes.trim_ascii()) else {
        return false;
    };
    if manuscript == saved {
        return true;
    }
    let Ok(dom) = tl::parse(saved, tl::ParserOptions::default()) else {
        return false;
    };
    let mut remaining = manuscript;
    let mut cursor = 0;
    for tag in dom.nodes().iter().filter_map(tl::Node::as_tag) {
        let style = tag.name() == "style";
        let geometry = tag.name() == "svg"
            && tag
                .attributes()
                .get("aria-hidden")
                .flatten()
                .is_some_and(|v| v.as_bytes() == b"true")
            && tag
                .children()
                .all(dom.parser())
                .iter()
                .all(|node| match node {
                    tl::Node::Tag(shape) => matches!(
                        shape.name().as_bytes(),
                        b"path"
                            | b"circle"
                            | b"rect"
                            | b"line"
                            | b"polygon"
                            | b"polyline"
                            | b"ellipse"
                            | b"g"
                    ),
                    tl::Node::Raw(text) => text.as_utf8_str().trim().is_empty(),
                    _ => false,
                });
        if !style && !geometry {
            continue;
        }
        let (start, end) = tag.boundaries(dom.parser());
        if start < cursor {
            continue;
        }
        if let Some(rest) = remaining.strip_prefix(&saved[cursor..=end]) {
            remaining = rest;
        } else {
            let prefix = &saved[cursor..start];
            // Ignore the extra line break and indentation of a stand-alone style block.
            let prefix = if style && saved[end + 1..].starts_with('\n') {
                prefix
                    .trim_end_matches([' ', '\t'])
                    .strip_suffix('\n')
                    .unwrap_or(prefix)
            } else {
                prefix
            };
            let Some(rest) = remaining.strip_prefix(prefix) else {
                return false;
            };
            remaining = rest;
        }
        cursor = end + 1;
    }
    remaining == &saved[cursor..]
}

// Only discount the URL tokens of a manuscript link when the HTML contains
// the same visible label and destination. Keep the label in the body check.
fn html_source_labels(body: &str, raw: &[u8]) -> Result<String> {
    // A manuscript may contain the complete saved HTML, including its links.
    // When the markup itself matches, compare the visible text on both sides.
    let trimmed = body.trim();
    let markup = trimmed
        .strip_prefix("```html\n")
        .and_then(|s| s.strip_suffix("\n```"))
        .unwrap_or(trimmed)
        .trim();
    if markup.starts_with('<') && matches_styled_html(markup, raw) {
        let visible = html(markup);
        if !visible.trim().is_empty() {
            return Ok(visible);
        }
    }
    let links = re(r"\[([^\]\n]+)\]\((https?://[^\s()]+)\)");
    if !links.is_match(body) {
        return Ok(body.to_owned());
    }
    let dom = tl::parse(std::str::from_utf8(raw)?, tl::ParserOptions::default())?;
    let mut anchors = Vec::new();
    for tag in dom.nodes().iter().filter_map(tl::Node::as_tag) {
        if !tag.name().as_bytes().eq_ignore_ascii_case(b"a") {
            continue;
        }
        if let Some(href) = tag.attributes().get("href").flatten() {
            let label = tag.inner_text(dom.parser());
            let label = html_escape::decode_html_entities(&label)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            anchors.push((
                label,
                html_escape::decode_html_entities(&href.as_utf8_str()).into_owned(),
            ));
        }
    }
    Ok(links
        .replace_all(body, |c: &regex::Captures<'_>| {
            let label = c[1].split_whitespace().collect::<Vec<_>>().join(" ");
            if let Some(i) = anchors
                .iter()
                .position(|(text, href)| *text == label && *href == c[2])
            {
                anchors.swap_remove(i);
                c[1].to_owned()
            } else {
                c[0].to_owned()
            }
        })
        .into_owned())
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
fn office_units(node: &xmltree::Element, presentation: bool, texts: &mut Vec<String>) {
    if presentation && node.name == "txBody" {
        // Keep paragraphs in the same text box together for trace matching.
        let mut paragraphs = Vec::new();
        office_units(node, false, &mut paragraphs);
        texts.push(paragraphs.join(" "));
        return;
    }
    if matches!(node.name.as_str(), "p" | "si" | "c") {
        texts.push(units(node));
    }
    for child in &node.children {
        if let xmltree::XMLNode::Element(child) = child {
            office_units(child, presentation, texts);
        }
    }
}
fn word_units(node: &xmltree::Element, starts_unit: &mut bool, texts: &mut Vec<String>) {
    let cell = node.name == "tc";
    *starts_unit |= cell;
    if matches!(node.name.as_str(), "p" | "si" | "c") {
        let text = units(node);
        if *starts_unit {
            texts.push(text);
            *starts_unit = false;
        } else if let Some(previous) = texts.last_mut() {
            previous.push(' ');
            previous.push_str(&text);
        }
    }
    for child in &node.children {
        if let xmltree::XMLNode::Element(child) = child {
            word_units(child, starts_unit, texts);
        }
    }
    *starts_unit |= cell;
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
            let before = texts.len();
            if ext == "docx" {
                let mut starts_unit = true;
                word_units(&e, &mut starts_unit, &mut texts);
            } else {
                office_units(&e, ext == "pptx", &mut texts);
            }
            if texts.len() == before {
                let t = units(&e);
                if !t.is_empty() {
                    texts.push(t)
                }
            }
        }
    }
    // XML 1.0 forbids the record separator inside a cell or text box.
    Ok(texts.join("\x1e"))
}
fn ocr(p: &Path, language: &str) -> Result<String> {
    Ok(
        String::from_utf8_lossy(&cmd(&["tesseract", &strpath(p), "stdout", "-l", language])?)
            .into_owned(),
    )
}
fn pdf_text_and_blank_pages(path: &str) -> Result<(String, Vec<usize>)> {
    let text = utf(&cmd(&["pdftotext", path, "-"])?)?;
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
        blanks.push(1);
    }
    Ok((text, blanks))
}
fn pdf_trace_layout(path: &str) -> Result<(usize, String, String)> {
    let native_len = pdf_text_and_blank_pages(path)?.0.len();
    let layout = xml(&cmd(&["pdftotext", "-bbox-layout", path, "-"])?)?;
    let mut blocks = Vec::new();
    visit(&layout, &mut |block| {
        if block.name == "block" {
            let mut words = Vec::new();
            visit(block, &mut |word| {
                if word.name == "word" {
                    words.push(xml_text(word));
                }
            });
            blocks.push(words.join(" "));
        }
    });
    let separated = blocks.join("\x1e");
    let joined = separated.replace('\x1e', " ");
    Ok((native_len, separated, joined))
}
fn pdf_scope_crosses_blocks(separated: &str, joined: &str, candidate: &str) -> bool {
    // Require a matching boundary in the visual layout, and retain an in-block occurrence.
    let mut crosses = false;
    for m in fancy(TRACE).find_iter(joined).flatten() {
        if m.as_str()
            .split_whitespace()
            .eq(candidate.split_whitespace())
        {
            if !separated[m.start()..m.end()].contains('\x1e') {
                return false;
            }
            crosses = true;
        }
    }
    crosses
}
fn ocr_pdf_pages(path: &str, pages: &[usize], language: &str, dpi: &str) -> Result<Vec<String>> {
    let work = tempfile::tempdir()?;
    pages
        .iter()
        .map(|i| {
            let prefix = work.path().join(format!("page-{i}"));
            cmd(&[
                "pdftoppm",
                "-f",
                &i.to_string(),
                "-l",
                &i.to_string(),
                "-singlefile",
                "-r",
                dpi,
                "-png",
                path,
                &strpath(&prefix),
            ])?;
            ocr(&prefix.with_extension("png"), language)
        })
        .collect()
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
pub(crate) fn extract(p: &Path, ext: &str, language: Option<&str>) -> Result<(String, bool, bool)> {
    let path = strpath(p);
    if ["docx", "xlsx", "pptx"].contains(&ext) {
        return Ok((office_text(p, ext)?, false, false));
    }
    if ext == "pdf" {
        cmd(&["pdfinfo", &path])?;
        let (mut text, blanks) = pdf_text_and_blank_pages(&path)?;
        let image_only = !blanks.is_empty() && text.trim().is_empty();
        if let Some(lang) = language
            && !blanks.is_empty()
        {
            for page in ocr_pdf_pages(&path, &blanks, lang, "150")? {
                text.push('\n');
                text.push_str(&page);
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
fn slides_without_notes(parent: &str, request: &str) -> bool {
    parent.ends_with(".pptx")
        && request.trim_start().lines().next().is_some_and(|first| {
            // A slide-only export describes the screen contents as the entire handout.
            // Match the complete instruction so a request for additional notes stays in scope.
            if re(r"^(?:違いなし。)?(?:[0-9]{1,2}時[0-9]{1,2}分提出版の)?全[0-9]+スライドを画面表示どおりに収録。(?:議事メモの追加なし。)?$")
                .is_match(request.trim())
            {
                return true;
            }
            if matches!(
                request.trim(),
                "通常のスライド表示で出力。スライドの順序と内容は元と同じ。話者ノートは出力しない。"
                    | "投影順のスライド面を書き出し、発表者ノートは配布面に含めない。その他の違いなし。"
            ) {
                return true;
            }
            if matches!(request.trim(), "違いなし" | "スライド面に違いなし。発表者メモは含めない。") || re(r"(?i)^(?:The [a-z0-9]+ slide faces are unchanged and in the same order\.\s*)?Export the slide faces only; omit speaker notes\.(?:\s*No post-meeting additions\.)?$").is_match(request.trim())
                || re(r"(?i)^All [a-z0-9]+ slides in the same order and with the same slide content as v[0-9]+(?:\.[0-9]+)*\.\s+Speaker notes are omitted from the distributed PDF\.$").is_match(request.trim()) {
                return true;
            }
            first.split_inclusive('。').any(|sentence| {
                matches!(
                    sentence.trim(),
                    "発表者ノートを含めない。"
                        | "発表者ノートは含めない。"
                        | "発表者ノートは配布 PDF に含めない。"
                )
            })
        })
}

// A handout manuscript may restate lines already required by the slide manuscript.
// Preserve the larger occurrence count and every additional handout line.
fn append_handout_lines(mut printed: String, handout: &str) -> String {
    let mut remaining = HashMap::new();
    for line in printed.lines() {
        *remaining.entry(line.trim().to_owned()).or_insert(0usize) += 1;
    }
    for line in handout.lines() {
        if let Some(count) = remaining.get_mut(line.trim()) {
            if *count > 0 {
                *count -= 1;
                continue;
            }
        }
        printed.push('\n');
        printed.push_str(line);
    }
    printed
}
// Compare the printed portion when the derivative explicitly excludes working material.
fn print_source(parent: &str, body: &str, request: &str) -> String {
    if slides_without_notes(parent, request) {
        return body
            .lines()
            .filter(|line| {
                !line.trim_start().starts_with("発表者メモ：")
                    && !line.trim_start().starts_with("発表者ノート：")
                    && !line.trim_start().starts_with("Speaker notes:")
            })
            .collect::<Vec<_>>()
            .join("\n");
    }
    if parent.ends_with(".xlsx") {
        let selector = re(r"^「([^「」\n]+)」シートの印刷範囲のみ");
        if let Some(name) = selector.captures(request.trim_start()) {
            let wanted = format!("## シート「{}」", &name[1]);
            if body.lines().filter(|line| line.trim() == wanted).count() == 1 {
                let mut include = true;
                return body
                    .lines()
                    .filter(|line| {
                        if line.trim_start().starts_with("## ") {
                            include = !line.trim_start().starts_with("## シート「")
                                || line.trim() == wanted;
                        }
                        include
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
        }
    }
    body.to_owned()
}

// A Word revision can supply complete replacement text under the draft's headings.
// Keep inherited text outside those sections, including the document control block.
fn revised_word_source(parent: &str, changes: &str) -> String {
    let mut headings = HashSet::new();
    let mut replacements = false;
    let mut current = None;
    for line in changes.lines().map(str::trim) {
        if line.contains("それぞれの本文を以下で差し替える。") {
            replacements = true;
        } else if line.starts_with('#') {
            current = replacements
                .then_some(line)
                .filter(|s| s.starts_with("## "));
        } else if !line.is_empty() {
            if let Some(heading) = current.take() {
                headings.insert(heading);
            }
        }
    }
    let mut replaced = false;
    let retained = parent
        .lines()
        .filter(|line| {
            if line.trim_start().starts_with('#') {
                replaced = headings.contains(line.trim());
            }
            !replaced
        })
        .collect::<Vec<_>>()
        .join("\n");
    retained + "\n" + changes
}
// An unchanged slide-face export maps consecutively numbered slides to PDF pages.
fn slide_pdf_excerpt(parent_body: &str, changes: &str) -> Option<String> {
    let export = "表示スライドは違いなし。発表者ノートは印刷対象に含めない。";
    let slides = parent_body.trim_end().strip_suffix(export)?.trim_end();
    let requested = re(r"^提出PDFの pp\.([0-9]+(?:[-–—][0-9]+)?) を本文と[0-9]+表を含めて抜粋し、見出しと原文を保持する。")
        .captures(changes.trim_start())?;
    let (first, last) = pdf_page_range(&requested[1])?;
    let mut page = 0usize;
    let mut excerpt = Vec::new();
    for line in slides.lines() {
        if let Some(heading) = line.strip_prefix("## スライド") {
            page += 1;
            if heading.parse::<usize>().ok()? != page {
                return None;
            }
        } else if line.starts_with('#') || (page == 0 && !line.trim().is_empty()) {
            return None;
        }
        if (first..=last).contains(&page)
            && !line.trim_start().starts_with("発表者メモ：")
            && !line.trim_start().starts_with("発表者ノート：")
            && !line.trim_start().starts_with("Speaker notes:")
        {
            excerpt.push(line);
        }
    }
    (last <= page).then(|| excerpt.join("\n") + "\n" + changes)
}
fn numbered_pdf_pages(body: &str) -> Option<Vec<(usize, usize)>> {
    let candidate = re(r"(?i)^## Page [0-9]");
    let heading = re(r"(?i)^## Page ([1-9][0-9]*)(?:\s+[—–:-]\s+.+)?$");
    body.lines()
        .enumerate()
        .filter(|(_, line)| candidate.is_match(line))
        .map(|(index, line)| Some((index, heading.captures(line.trim_end())?[1].parse().ok()?)))
        .collect()
}

fn pdf_page_range(cell: &str) -> Option<(usize, usize)> {
    let span = re(r"^([1-9][0-9]*)(?:\s*[-–—]\s*([1-9][0-9]*))?$").captures(cell)?;
    let first = span[1].parse().ok()?;
    let last = match span.get(2) {
        Some(last) => last.as_str().parse().ok()?,
        None => first,
    };
    (first <= last).then_some((first, last))
}

fn shortened_pdf_source(
    parent: &str,
    parent_body: &str,
    child: &str,
    changes: &str,
) -> Option<String> {
    // A fenced example is not a page map or a numbered manuscript section.
    if parent_body.lines().chain(changes.lines()).any(|line| {
        let line = line.trim_start();
        line.starts_with("```") || line.starts_with("~~~")
    }) {
        return None;
    }
    let child_lines: Vec<_> = changes.lines().collect();
    let mut headers = child_lines.iter().enumerate().filter_map(|(index, line)| {
        if !line.starts_with('|') {
            return None;
        }
        let columns = cells(line);
        let [destination, origin, purpose] = columns.as_slice() else {
            return None;
        };
        purpose.eq_ignore_ascii_case("Replacements").then_some(())?;
        Some((
            index,
            normalized(destination).strip_suffix(" pages")?.to_owned(),
            normalized(origin).strip_suffix(" pages")?.to_owned(),
        ))
    });
    let (index, destination, origin) = headers.next()?;
    if headers.next().is_some() {
        return None;
    }
    let names_edition = |edition: &str, path: &str| {
        let edition = tokens(edition);
        !edition.is_empty()
            && Path::new(path)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| {
                    tokens(&normalized(stem))
                        .windows(edition.len())
                        .any(|words| words == edition)
                })
    };
    if !(destination == "new" && origin == "old"
        || names_edition(&destination, child)
            && !names_edition(&destination, parent)
            && names_edition(&origin, parent)
            && !names_edition(&origin, child))
    {
        return None;
    }
    let delimiter = cells(child_lines.get(index + 1)?);
    if delimiter.len() != 3 || !separator(&delimiter) {
        return None;
    }
    let mut retained = Vec::new();
    let mut last_child = 0usize;
    let mut last_parent = 0usize;
    for row in child_lines[index + 2..]
        .iter()
        .take_while(|row| row.contains('|'))
    {
        let columns = cells(row);
        let [destination, origin, _] = columns.as_slice() else {
            return None;
        };
        let (first_child, end_child) = pdf_page_range(destination)?;
        let (first_parent, end_parent) = pdf_page_range(origin)?;
        if first_child != last_child.checked_add(1)?
            || first_parent <= last_parent
            || end_parent - first_parent != end_child - first_child
        {
            return None;
        }
        retained.push(first_parent..=end_parent);
        last_child = end_child;
        last_parent = end_parent;
    }

    let parent_pages = numbered_pdf_pages(parent_body)?;
    if retained.is_empty()
        || last_parent > parent_pages.len()
        || last_child >= parent_pages.len()
        || parent_pages
            .iter()
            .enumerate()
            .any(|(i, &(_, page))| page != i + 1)
    {
        return None;
    }
    let mut prior_child = 0;
    for (line, page) in numbered_pdf_pages(changes)? {
        if line <= index || page <= prior_child || page > last_child {
            return None;
        }
        prior_child = page;
    }

    let mut parent_pages = parent_pages.into_iter().peekable();
    let mut expected = String::new();
    let mut keep = true;
    for (index, line) in parent_body.lines().enumerate() {
        if let Some(&(line_index, page)) = parent_pages.peek()
            && line_index == index
        {
            keep = retained.iter().any(|range| range.contains(&page));
            parent_pages.next();
        } else if line.starts_with("# ") || line.starts_with("## ") {
            keep = true;
        }
        if keep {
            expected.push_str(line);
            expected.push('\n');
        }
    }
    expected.push('\n');
    expected.push_str(changes);
    Some(expected)
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
        let (parent_cfg, mut parent_body) = source(root, parent, chain)?;
        body = if rel.ends_with(".pdf")
            && cfg
                .get("kind")
                .is_some_and(|kind| kind.starts_with("office:"))
        {
            let printed = print_source(parent, &parent_body, &body);
            if slides_without_notes(parent, &body) {
                append_handout_lines(printed, &body)
            } else {
                printed + "\n" + &body
            }
        } else if rel.ends_with(".pdf") && parent.ends_with(".pdf") {
            // The original scan's paper description was never printed on the copy.
            // Keep its page headings, numbers and transcript, and all changes on the copy.
            if cfg.get("kind").and_then(|kind| kind.strip_prefix("scan:")) == Some(parent.as_str())
                && parent_cfg
                    .get("kind")
                    .is_some_and(|kind| kind.starts_with("scan:"))
                && parent_cfg.get("derived_from").is_none_or(String::is_empty)
                && scan_transcript(&parent_body).is_some()
                && let Some(appearance) = re(SCAN_APPEARANCE).find(&parent_body)
            {
                parent_body.truncate(appearance.start());
            }
            shortened_pdf_source(parent, &parent_body, rel, &body)
                .unwrap_or_else(|| parent_body + "\n" + &body)
        } else if rel.ends_with(".docx")
            && parent.ends_with(".pdf")
            && parent_cfg
                .get("kind")
                .is_some_and(|kind| kind.starts_with("office:") && kind.ends_with(".pptx"))
            && parent_cfg
                .get("derived_from")
                .is_some_and(|path| path.ends_with(".pptx"))
        {
            slide_pdf_excerpt(&parent_body, &body).unwrap_or_else(|| parent_body + "\n" + &body)
        } else if rel.ends_with(".docx") && parent.ends_with(".docx") {
            revised_word_source(&parent_body, &body)
        } else {
            parent_body + "\n" + &body
        };
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
fn data_lines(text: &str, ext: &str) -> String {
    let mut lines = Vec::new();
    let mut sample = Vec::new();
    let mut headers = Vec::new();
    let mut markdown = Vec::new();
    let mut fence = None;
    let mut read_header = false;
    for line in text.lines() {
        if let Some(label) = line.trim().strip_prefix("```") {
            fence = if fence.is_some() {
                None
            } else {
                Some(label.trim().eq_ignore_ascii_case(ext))
            };
            read_header = fence == Some(true);
            markdown.push("");
            continue;
        }
        if fence.is_some() || line.trim_start().starts_with('|') {
            lines.push(line);
        }
        if fence.is_none() {
            markdown.push(line);
        }
        if fence == Some(true) {
            sample.push(line);
            if read_header && !line.trim().is_empty() {
                read_header = false;
                let delimiter = if ext == "tsv" { b'\t' } else { b',' };
                let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .delimiter(delimiter)
                    .from_reader(line.as_bytes());
                if let Some(Ok(header)) = reader.records().next() {
                    headers.push(header);
                }
            }
        }
    }
    if !headers.is_empty() {
        // A labeled sample identifies the output columns. Other Markdown
        // tables can describe totals or rules for generating the data.
        for table in markdown.split(|line| !line.trim_start().starts_with('|')) {
            if table.len() < 2 || !separator(&cells(table[1])) {
                continue;
            }
            let columns = cells(table[0]);
            if headers.iter().any(|header| {
                header.len() == columns.len()
                    && header
                        .iter()
                        .zip(&columns)
                        .all(|(sample, column)| sample.trim() == column.trim_matches([' ', '`']))
            }) {
                sample.push("");
                sample.extend_from_slice(table);
            }
        }
        sample.join("\n")
    } else if lines.is_empty() {
        text.into()
    } else {
        lines.join("\n")
    }
}
fn revision_text(text: &str) -> &str {
    let text = text.trim();
    text.strip_prefix('“')
        .and_then(|text| text.strip_suffix('”'))
        .unwrap_or(text)
}

// Revision instructions use a local timestamp, while Word commonly stores the instant in UTC.
fn word_revision_source(body: &str, path: &Path, zone: chrono_tz::Tz) -> Result<String> {
    let instruction = re(
        r"(?m)^[-*] (削除|挿入|コメント) *\| *([^|\n]+?) *\| *(\d{4}-\d\d-\d\d \d\d:\d\d [+-]\d{4}) *\|",
    );
    let labeled = re(concat!(
        r"(?m)^[-*][ \t]+種類[:：][ \t]*(削除|挿入|コメント)[ \t]*(?:[;／|][ \t]*|\r?\n[ \t]+)",
        r"人物[:：][ \t]*([^\r\n]+?)[ \t]*(?:[;／|][ \t]*|\r?\n[ \t]+)",
        r"日時[:：][ \t]*(\d{4}-\d\d-\d\d \d\d:\d\d)(?: ([A-Z]{2,6}|[+-]\d{4}))?[ \t]*(?:[;／|][ \t]*|\r?\n[ \t]+)",
        r"対象(?:の文)?[:：][ \t]*([^\r\n]+?)[ \t]*(?:[;／|][ \t]*|\r?\n[ \t]+)",
        r"新しい文[:：][ \t]*([^\r\n]+)\r?$",
    ));
    let comment = re(concat!(
        r"(?m)^[-*] 種類：コメント。人物：([^。\n]+)。日時：(\d{4}-\d\d-\d\d \d\d:\d\d [A-Z]{2,6})。",
        r"対象：([^。\n]+)。内容：([^\r\n]+)\r?$",
    ));
    if !instruction.is_match(body) && !labeled.is_match(body) && !comment.is_match(body) {
        return Ok(body.to_string());
    }
    let parts = zip_parts(path)?;
    let mut revisions = BTreeSet::new();
    let mut labeled_records = Vec::new();
    for part in ["word/document.xml", "word/comments.xml"] {
        if let Some(raw) = parts.get(part) {
            visit(&xml(raw)?, &mut |node| {
                let kind = match (part, node.name.as_str()) {
                    ("word/document.xml", "del") => "削除",
                    ("word/document.xml", "ins") => "挿入",
                    ("word/comments.xml", "comment") => "コメント",
                    _ => return,
                };
                if let (Some(author), Some(date)) =
                    (node.attributes.get("author"), node.attributes.get("date"))
                    && let Ok(instant) = DateTime::parse_from_rfc3339(date)
                {
                    revisions.insert((kind, author.clone(), instant));
                    labeled_records.push((
                        kind,
                        author.clone(),
                        instant.with_timezone(&zone),
                        units(node),
                    ));
                }
            });
        }
    }
    let mut result = body.to_string();
    for captures in instruction
        .captures_iter(body)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        let date = captures.get(3).unwrap();
        if let Ok(instant) = DateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M %z")
            && revisions.contains(&(&captures[1], captures[2].to_string(), instant))
        {
            result.replace_range(date.range(), "");
        }
    }
    let result = comment.replace_all(&result, |row: &regex::Captures<'_>| {
        if let Some(index) = labeled_records
            .iter()
            .position(|(kind, author, date, text)| {
                *kind == "コメント"
                    && author == &row[1]
                    && date.format("%Y-%m-%d %H:%M %Z").to_string() == row[2]
                    && text == &row[4]
            })
        {
            labeled_records.swap_remove(index);
            format!("{} {}", &row[3], &row[4])
        } else {
            row[0].to_owned()
        }
    });
    // A tracked edit stores its payload once in OOXML. Keep the target in the
    // manuscript comparison after verifying the actual revision or comment.
    Ok(labeled
        .replace_all(&result, |row: &regex::Captures<'_>| {
            let deleted = &row[1] == "削除";
            let target = revision_text(&row[5]);
            let replacement = revision_text(&row[6]);
            let payload = if deleted { &row[5] } else { &row[6] };
            let unwrapped = revision_text(payload);
            let matched = labeled_records
                .iter()
                .enumerate()
                .filter(|(_, (kind, author, date, text))| {
                    *kind == &row[1]
                        && author == &row[2]
                        && date.format("%Y-%m-%d %H:%M").to_string() == row[3]
                        && row.get(4).is_none_or(|zone| {
                            let label = zone.as_str();
                            let format = if label.starts_with(['+', '-']) {
                                "%z"
                            } else {
                                "%Z"
                            };
                            date.format(format).to_string() == label
                        })
                        && (text == payload || text == unwrapped)
                })
                .min_by_key(|(_, (_, _, _, text))| text != payload)
                .map(|(index, _)| index);
            if let Some(index) =
                matched.filter(|_| !deleted || matches!(replacement, "なし" | "なし。"))
            {
                let (_, _, _, text) = labeled_records.swap_remove(index);
                if text == payload {
                    row[5].to_owned()
                } else {
                    target.to_owned()
                }
            } else {
                row[0].to_owned()
            }
        })
        .into_owned())
}
const SCAN_MIN_OVERLAP: f64 = 0.3;

fn body_token_overlap(expected: &str, actual: &str) -> f64 {
    let want = counts(&ungroup(expected));
    let have = counts(&ungroup(actual));
    let sum: usize = want.values().sum();
    if sum == 0 {
        return 1.0;
    }
    want.iter()
        .map(|(k, v)| (*v).min(*have.get(k).unwrap_or(&0)))
        .sum::<usize>() as f64
        / sum as f64
}

fn strip_transcript_list_labels(page: String) -> String {
    let blocks = re(r"(?m)(?:^[0-9]{1,9}\.[ \t]+[^\n]+\n)+");
    let label = re(r"^[0-9]{1,9}\.[ \t]+");
    blocks
        .replace_all(&page, |block: &regex::Captures<'_>| {
            let rows: Vec<_> = block[0].lines().collect();
            if rows.len() < 2
                || !rows
                    .iter()
                    .enumerate()
                    .all(|(i, line)| line.starts_with(&format!("{}.", i + 1)))
            {
                return block[0].to_owned();
            }
            rows.iter()
                .map(|line| label.replace(line, ""))
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        })
        .into_owned()
}

const SCAN_APPEARANCE: &str =
    r"(?im)^[ \t]*## (?:Paper and scan|Papier et (?:numérisation|copie))[ \t]*\r?$";

fn scan_transcript(expected: &str) -> Option<Vec<String>> {
    let heading = re(r"(?i)^## Page ([1-9][0-9]*)(?:\s+[—–:-]\s+.+)?$");
    let folio = re(r"^## [A-Z]+/[0-9]+/[A-Z]+ f\. [0-9]+[rv]$");
    let appearance = re(SCAN_APPEARANCE);
    let mut lines = expected.trim_start().lines().map(str::trim);
    let mut pages = Vec::<String>::new();
    let first = lines.clone().next()?;
    let implicit_page = !first.starts_with('#');
    if implicit_page {
        pages.push(String::new());
    }
    while let Some(line) = lines.next() {
        if let Some(page) = heading.captures(line) {
            if implicit_page || page[1].parse::<usize>().ok() != Some(pages.len() + 1) {
                return None;
            }
            pages.push(String::new());
        } else if folio.is_match(line) {
            if implicit_page {
                return None;
            }
            pages.push(String::new());
        } else if appearance.is_match(line) {
            return (!pages.is_empty()
                && pages.iter().all(|page| !counts(page).is_empty())
                && !lines.any(|line| line.starts_with("# ") || line.starts_with("## ")))
            .then(|| {
                pages
                    .into_iter()
                    .map(strip_transcript_list_labels)
                    .collect()
            });
        } else {
            if line.starts_with("# ") || line.starts_with("## ") {
                return None;
            }
            let page = pages.last_mut()?;
            page.push_str(line);
            page.push('\n');
        }
    }
    None
}

fn retry_scan_transcript(p: &Path, expected: &str, language: &str) -> Result<Option<String>> {
    let Some(transcript) = scan_transcript(expected) else {
        return Ok(None);
    };
    let path = strpath(p);
    let (layer, pages) = pdf_text_and_blank_pages(&path)?;
    if !layer.trim().is_empty() || pages.len() != transcript.len() {
        return Ok(None);
    }
    let recognized = ocr_pdf_pages(&path, &pages, language, "200")?;
    if transcript
        .iter()
        .zip(&recognized)
        .any(|(expected, actual)| body_token_overlap(expected, actual) == 0.0)
    {
        return Ok(None);
    }
    let recognized = recognized.join("\n");
    Ok(
        (body_token_overlap(&transcript.join("\n"), &recognized) >= SCAN_MIN_OVERLAP)
            .then_some(recognized),
    )
}

// Workbook manuscripts may embed literal CSV rows alongside formulas and prose.
// Separate fields before numeric comparison so date tails cannot absorb quantities.
fn workbook_source(text: &str) -> String {
    re(r"(?ims)^[ \t]*```csv[ \t]*\r?\n(.*?)^[ \t]*```[ \t]*\r?$")
        .replace_all(text, |block: &regex::Captures<'_>| {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(block[1].as_bytes());
            let mut rows = Vec::new();
            for row in reader.records() {
                let Ok(row) = row else {
                    return block[0].to_owned();
                };
                rows.push(row.iter().collect::<Vec<_>>().join("\t"));
            }
            rows.join("\n")
        })
        .into_owned()
}

fn source_reason(
    expected: &str,
    actual: &str,
    scan: bool,
    ext: &str,
    reviewed: bool,
    derived: bool,
) -> Option<String> {
    let delimited = matches!(ext, "csv" | "tsv");
    let table = delimited || ext == "xlsx";
    // Locate Markdown table rows before source_body removes their outer pipes.
    let mut expected = if delimited {
        source_body(&data_lines(expected, ext))
    } else if ext == "xlsx" {
        source_body(&workbook_source(expected))
    } else {
        source_body(expected)
    };
    if !table {
        let fig = re(r"(?i)図|グラフ|軸|凡例|目盛|chart|axis|axes|legend|graph");
        expected = expected
            .lines()
            .filter(|l| !fig.is_match(l))
            .collect::<Vec<_>>()
            .join("\n")
    }
    let overlap = body_token_overlap(&expected, actual);
    if !scan && !derived {
        let vals = numbers(&expected);
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
        SCAN_MIN_OVERLAP
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
pub(crate) fn attachment_declaration() -> Regex {
    re(
        r"(?i)^\s*(?:[-*]\s*)?(?:添付(?:ファイル)?(?:（実ファイル）)?|attachments?(?:[ \t]+\(actual files?\))?)\s*[:：](.*)$",
    )
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
    let declaration = attachment_declaration();
    let file = re(&format!(
        r"(?i)^[^<>,\n]+\.(?:{})$",
        EXT.split_whitespace().collect::<Vec<_>>().join("|")
    ));
    let mut lines = expected.lines().peekable();
    while let Some(l) = lines.next() {
        if l.trim()
            .eq_ignore_ascii_case("----- Original Message -----")
            && lines.peek().is_some_and(|line| {
                line.trim_start()
                    .get(..5)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("From:"))
            })
        {
            // Attachment names in the quoted header block belong to the earlier message.
            for header in lines.by_ref() {
                if header.trim().is_empty() {
                    break;
                }
            }
            listing = false;
            continue;
        }
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
        for item in value.split('、').map(str::trim) {
            let quoted = item
                .strip_prefix('`')
                .and_then(|text| text.strip_suffix('`'));
            let members = item.split("; ");
            // Split only complete filename lists and preserve a quoted literal name.
            let names: Vec<_> = if quoted.is_some_and(|text| !text.contains('`'))
                || members
                    .clone()
                    .any(|name| !file.is_match(name.trim().trim_matches('`')))
            {
                vec![item]
            } else {
                members.collect()
            };
            for name in names {
                let name = name.trim().trim_matches('`');
                if file.is_match(name) {
                    declared.insert(
                        Path::new(name)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
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
fn persona_timezone(cfg: &BTreeMap<String, String>) -> Result<chrono_tz::Tz> {
    Ok(cfg
        .get("timezone")
        .map(String::as_str)
        .unwrap_or("UTC")
        .parse()?)
}

fn outside_period(instant: DateTime<Utc>, zone: chrono_tz::Tz, period: &str) -> Result<bool> {
    let (start, end) = period
        .split_once("..")
        .ok_or_else(|| anyhow!("invalid period"))?;
    let day = instant.with_timezone(&zone).date_naive();
    Ok(day < NaiveDate::parse_from_str(start, "%Y-%m-%d")?
        || day > NaiveDate::parse_from_str(end, "%Y-%m-%d")?)
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
    let zone = persona_timezone(cfg)?;
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
                if let Some(period) = cfg.get("period")
                    && outside_period(t, zone, period)?
                {
                    return Ok(Some(format!("{key}: outside persona period: {value}")));
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
    measurements: Option<CsvMeasurements>,
    skipped: Vec<String>,
}

pub(crate) struct CsvMeasurements {
    header: csv::StringRecord,
    columns: Vec<Option<BTreeMap<u64, usize>>>,
}

impl CsvMeasurements {
    const LENGTH_UNIT: &str = r"(?:_(?:mm|cm|m)|\((?:mm|cm|m)\))$";
    const MASS_CONCENTRATION_UNIT: &str = r"(?:_[num]?g_m?l|\([num]?g/m?l\))$";

    pub(crate) fn from_draft(path: &str, body: &str) -> Option<Self> {
        if !path.ends_with(".csv") {
            return None;
        }
        let csv = body
            .strip_prefix("```csv\n")
            .and_then(|s| s.strip_suffix("\n```"))
            .unwrap_or(body);
        Self::parse(csv)
    }

    fn parse(text: &str) -> Option<Self> {
        let mut reader = csv::Reader::from_reader(text.as_bytes());
        let header = reader.headers().ok()?.clone();
        let length = re(Self::LENGTH_UNIT);
        let concentration = re(Self::MASS_CONCENTRATION_UNIT);
        let mut columns = header
            .iter()
            .map(|name| {
                let name = normalized(name.trim());
                (length.is_match(&name) || concentration.is_match(&name)).then(BTreeMap::new)
            })
            .collect::<Vec<_>>();
        if columns.iter().all(Option::is_none) {
            return None;
        }
        // A qualifying measurement column contains decimal readings in every row.
        let decimal = re(r"^[+-]?[0-9]+\.[0-9]+$");
        for row in reader.records() {
            let row = row.ok()?;
            for (column, cell) in columns.iter_mut().zip(row.iter()) {
                let Some(readings) = column else {
                    continue;
                };
                let cell = cell.trim();
                if let Some(value) = cell
                    .parse::<f64>()
                    .ok()
                    .filter(|v| v.is_finite() && decimal.is_match(cell))
                {
                    // Match signed zero and alternate decimal spellings.
                    *readings.entry((value + 0.0).to_bits()).or_default() += 1;
                } else {
                    *column = None;
                }
            }
        }
        Some(Self { header, columns })
    }

    pub(crate) fn differs_from(&self, other: &Self) -> bool {
        self.header == other.header
            && self
                .header
                .iter()
                .zip(self.columns.iter().zip(&other.columns))
                .any(|(name, (a, b))| {
                    let (Some(a), Some(b)) = (a, b) else {
                        return false;
                    };
                    let rows_a = a.values().sum::<usize>();
                    let rows_b = b.values().sum::<usize>();
                    let concentration =
                        re(Self::MASS_CONCENTRATION_UNIT).is_match(&normalized(name.trim()));
                    let min_distinct = if concentration { 4 } else { 5 };
                    if a.len().min(b.len()) < min_distinct || (!concentration && rows_a != rows_b) {
                        return false;
                    }
                    // Require changed frequencies for several values on both sides.
                    // Replacing a single repeated value can change most rows at once.
                    let mut common = 0;
                    let mut excess_a = 0;
                    for (value, &count) in a {
                        let other_count = b.get(value).copied().unwrap_or(0);
                        common += count.min(other_count);
                        excess_a += usize::from(count > other_count);
                    }
                    let excess_b = b
                        .iter()
                        .filter(|(value, count)| a.get(value).is_none_or(|other| other < *count))
                        .count();
                    // Measure overlap against the shorter export so a filtered copy still matches.
                    5 * common < 4 * rows_a.min(rows_b) && excess_a >= 2 && excess_b >= 2
                })
    }
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
                    let office_units = ["docx", "xlsx", "pptx"]
                        .contains(&ext.as_str())
                        .then_some(&body);
                    text = if office_units.is_some() {
                        body.replace('\x1e', " ")
                    } else {
                        body.clone()
                    };
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
                            let body = if ext == "html" {
                                html_source_labels(&body, &raw)?
                            } else if ext == "docx" {
                                word_revision_source(&body, p, persona_timezone(cfg)?)?
                            } else {
                                body
                            };
                            let visual = ["png", "jpg"].contains(&ext.as_str())
                                || scfg.get("kind").is_some_and(|s| {
                                    s.split(':').any(|s| {
                                        ["photo", "handwriting", "chart", "diagram"].contains(&s)
                                    })
                                });
                            // Audio decoding produces frames, without a text transcription.
                            let audio = matches!(ext.as_str(), "m4a" | "mp3" | "wav");
                            if !visual && !audio {
                                let derived =
                                    scfg.get("derived_from").is_some_and(|s| !s.is_empty());
                                if missing {
                                    r.skipped.push("source(OCR)".into())
                                } else if let Some(reason) = source_reason(
                                    &body,
                                    &text,
                                    scan,
                                    &ext,
                                    ext == "docx" && reviewed(p),
                                    derived,
                                ) {
                                    let retry = if let Some(lang) = language
                                        && scan
                                        && ext == "pdf"
                                        && !derived
                                        && scfg.get("kind").is_some_and(|k| k.starts_with("scan:"))
                                    {
                                        retry_scan_transcript(p, &body, lang)
                                    } else {
                                        Ok(None)
                                    };
                                    match retry {
                                        Ok(Some(recognized)) => {
                                            // Retain both OCR reads for the trace and secret checks.
                                            text.push('\n');
                                            text.push_str(&recognized);
                                        }
                                        Ok(None) => fail(&mut r, "source", reason),
                                        Err(e) => {
                                            fail(&mut r, "source", reason);
                                            fail(&mut r, "opens", format!("OCR retry: {e}"));
                                        }
                                    }
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
                    let mut found = false;
                    let pdf_layout = std::cell::OnceCell::new();
                    for m in traces(&traced, cfg.get("allowed_terms").map(String::as_str)) {
                        // A cell or text box ending in "scope" does not qualify its neighbor.
                        if m.as_str().to_ascii_lowercase().starts_with("scope")
                            && let Some(units) = office_units
                            && let Some(start) = m.start().checked_sub(rel.len() + 1)
                            && units[start..start + m.as_str().len()].contains('\x1e')
                        {
                            continue;
                        }
                        // Plain PDF extraction can place separate cover and control blocks
                        // on consecutive lines. Confirm their layout for native text only.
                        if ext == "pdf"
                            && m.as_str().to_ascii_lowercase().starts_with("scope")
                            && m.as_str().contains(['\n', '\x0c'])
                            && m.start() > rel.len()
                            && let Some((native_len, separated, joined)) =
                                pdf_layout.get_or_init(|| pdf_trace_layout(&strpath(p)).ok())
                            && m.end() - (rel.len() + 1) <= *native_len
                            && pdf_scope_crosses_blocks(separated, joined, m.as_str())
                        {
                            continue;
                        }
                        fail(&mut r, "trace", format!("production trace: {}", m.as_str()));
                        found = true;
                        break;
                    }
                    if !found {
                        if let Some(m)=re(r"本文への埋め込み|【[^】\n]{0,60}(?:削除・取り消し線|赤字の挿入|赤字追記)[^】\n]{0,20}】").find(&traced){fail(&mut r,"trace",format!("production trace: {}",m.as_str()));}
                    }
                }
            }
        }
        if let Some(period) = cfg.get("period")
            && outside_period(st.modified()?.into(), persona_timezone(cfg)?, period)?
        {
            fail(&mut r, "mtime", "outside persona period (local date)")
        }
        if ext == "csv" {
            r.measurements = CsvMeasurements::parse(&text);
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
                                    .is_some_and(|v| v.contains(&results[i].rel))
                                || results[i]
                                    .measurements
                                    .as_ref()
                                    .zip(results[j].measurements.as_ref())
                                    .is_some_and(|(a, b)| a.differs_from(b))))
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
    /// Inspect only deliverables committed in Git HEAD.
    #[arg(long, requires = "all")]
    accepted: bool,
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
pub(crate) fn ocr_language() -> Result<Option<&'static str>> {
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
    Ok(language)
}
fn run_inner(a: &Args, selected: Option<&BTreeMap<String, BTreeSet<String>>>) -> Result<i32> {
    let mut accepted = BTreeMap::<String, BTreeSet<String>>::new();
    let selected = if a.accepted {
        for path in crate::manifest::accepted(&a.root)? {
            let (name, rel) = path.split_once('/').context("invalid accepted file path")?;
            accepted.entry(name.into()).or_default().insert(rel.into());
        }
        if accepted.is_empty() {
            bail!("no accepted files")
        }
        Some(&accepted)
    } else {
        selected
    };
    let names = match selected {
        Some(s) => s.keys().cloned().collect(),
        None => names(a)?,
    };
    let started = Instant::now();
    let mut failures = Vec::<[String; 4]>::new();
    let (mut passed, mut pending) = (0, 0);
    let mut failed_files = BTreeSet::new();
    let mut skips = BTreeMap::<String, usize>::new();
    let language = ocr_language()?;
    for name in names {
        let folder = a.root.join(&name);
        let cfg_path = a.personas.join(format!("{name}.md"));
        let cfg = if cfg_path.exists() {
            config_text(&read(&cfg_path)?, false)?
        } else {
            eprintln!("定義なし：{}（期間検査・許可語なし）", cfg_path.display());
            BTreeMap::new()
        };
        let name_rules = selected
            .map(|_| crate::rename::NameRules::new(cfg.get("date_prefix").map(String::as_str)))
            .transpose()
            .with_context(|| format!("{}: 命名規則", cfg_path.display()))?;
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
                    let versions = related.entry(r.rel.clone()).or_default();
                    versions.insert(parent.clone());
                    // A scan of an Office printout also reproduces the Office original.
                    if r.rel.ends_with(".pdf")
                        && parent.ends_with(".pdf")
                        && cfg.get("kind").and_then(|k| k.strip_prefix("scan:"))
                            == Some(parent.as_str())
                        && let Ok((print, _)) =
                            source(&a.sources.join(&name), parent, &mut Vec::new())
                        && let Some(template) =
                            print.get("kind").and_then(|k| k.strip_prefix("office:"))
                        && let Some(original) = print.get("derived_from")
                        && ["docx", "xlsx", "pptx"].iter().any(|ext| {
                            Path::new(template).extension() == Some(ext.as_ref())
                                && Path::new(original).extension() == Some(ext.as_ref())
                        })
                    {
                        versions.insert(original.clone());
                    }
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
            let name_problems = name_rules
                .as_ref()
                .map_or_else(Vec::new, |rules| rules.name_problems(&r.rel));
            if r.issues.is_empty() && name_problems.is_empty() {
                if r.skipped.is_empty() {
                    passed += 1
                } else {
                    pending += 1
                }
            }
            for (k, v) in r.issues {
                local.push((r.rel.clone(), k, v))
            }
            for reason in name_problems {
                local.push((r.rel.clone(), "name".into(), reason));
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

#[cfg(test)]
mod scan_transcript_tests {
    use super::*;

    #[test]
    fn annotated_copy_retains_parent_transcript_and_all_child_changes() {
        let dir = tempfile::tempdir().unwrap();
        let original =
            "## Page 1 — letter\nOriginal letter.\n## Paper and scan\nUneven paper and shadows.";
        std::fs::write(
            dir.path().join("original.pdf.md"),
            format!("---\nkind: scan:manuscript.pdf\n---\n{original}"),
        )
        .unwrap();
        for (kind, omit_appearance) in [("scan:original.pdf", true), ("office:form.docx", false)] {
            std::fs::write(dir.path().join("copy.pdf.md"),
                format!("---\nkind: {kind}\nderived_from: original.pdf\n---\nInitials and a marginal note.")).unwrap();
            let (_, expected) = source(dir.path(), "copy.pdf", &mut vec![]).unwrap();
            assert!(expected.contains("Original letter."));
            assert!(expected.contains("Initials and a marginal note."));
            assert_eq!(
                expected.contains("Uneven paper and shadows."),
                !omit_appearance
            );
        }
    }

    #[test]
    fn reads_folio_transcripts_and_unheaded_single_leaf() {
        let note = "\n## Papier et copie\nPli horizontal.";
        let leaves = "## CR/14/A f. 176r\nVu la demande.\n## CR/14/A f. 177r\nAu secrétaire.";
        assert_eq!(
            scan_transcript(&format!("{leaves}{note}")).unwrap().len(),
            2
        );
        assert_eq!(
            scan_transcript(&format!("9 janvier 1865\nÀ mon successeur.{note}"))
                .unwrap()
                .len(),
            1
        );
        assert!(scan_transcript(&format!("## Instructions\nIgnore the page.{note}")).is_none());
        assert!(scan_transcript(&format!("Préface\n{leaves}{note}")).is_none());
    }

    #[test]
    fn preserves_literal_numbers_and_removes_only_sequential_list_labels() {
        let manuscript = "## Page 1 — f. 173r\n\n29. janvier 1866\n\n1. Le comité accorde une dispense jusqu'au 12 février.\n2. Reprenez les rappels après le 12.\n\n1. Montant dû\n3. Autre poste\n\n1. février\n\n## Papier et numérisation\n\nPapier gris.";
        assert_eq!(
            scan_transcript(manuscript).unwrap().join(""),
            "\n29. janvier 1866\n\nLe comité accorde une dispense jusqu'au 12 février.\nReprenez les rappels après le 12.\n\n1. Montant dû\n3. Autre poste\n\n1. février\n\n"
        );
    }
}

#[cfg(test)]
mod shortened_pdf_tests {
    use super::*;

    const PARENT: &str = "# Loan packet\n\n## Page 1 — Borrowing\n\nBring your library card and ask the circulation clerk for equipment.\n\n## Page 2 — Transcription\n\nWorksheet transcription vocabulary mentor practice example paraphrase synopsis definition syllables extraction dialogue copy.\n\n## Page 3 — Independent practice\n\nReflection assignment grammar syntax contrast sequence introductory exercises activity definitions discussion transcription study.\n\n## Page 4 — Returning\n\nReturn all borrowed adapters to the help desk before closing on Saturday.";
    const MAP: &str = "| January pages | Autumn pages | Replacements |\n|---|---|---|\n| 1 | 1 | Retain borrowing. |\n| 2 | 4 | Add the sentence below. |";
    const CHANGE: &str =
        "## Page 2 — Added sentence\n\nLeave the signed return slip in the blue tray.";
    const PRINTED: &str = "Loan packet\nBorrowing\nBring your library card and ask the circulation clerk for equipment.\nReturning\nReturn all borrowed adapters to the help desk before closing on Saturday.\nLeave the signed return slip in the blue tray.";

    fn composed_source(parent: &str, changes: &str) -> String {
        let root = tempfile::tempdir().unwrap();
        for (path, origin, body) in [
            ("Autumn packet.pdf", "", parent),
            ("January packet.pdf", "Autumn packet.pdf", changes),
        ] {
            fs::write(
                root.path().join(format!("{path}.md")),
                format!("---\npath: {path}\nkind: tex:loans.cls\nauthor: Reader\nderived_from: {origin}\n---\n{body}"),
            )
            .unwrap();
        }
        source(root.path(), "January packet.pdf", &mut Vec::new())
            .unwrap()
            .1
    }

    #[test]
    fn explicit_page_map_removes_only_unretained_sections() {
        let changes = format!("# Changes\n\n{MAP}\n\n{CHANGE}");
        let expected = composed_source(PARENT, &changes);
        assert!(expected.contains("circulation clerk"));
        assert!(expected.contains("borrowed adapters"));
        assert!(expected.contains("signed return slip"));
        assert!(!expected.contains("mentor practice"));
        assert!(!expected.contains("Reflection assignment"));
        assert_eq!(
            source_reason(&expected, PRINTED, false, "pdf", false, true),
            None
        );
        assert!(
            source_reason(
                &expected,
                "Bring your library card",
                false,
                "pdf",
                false,
                true
            )
            .is_some()
        );
        let deleted = PARENT
            .split_once("## Page 2")
            .unwrap()
            .1
            .split_once("## Page 4")
            .unwrap()
            .0;
        assert!(source_reason(&expected, deleted, false, "pdf", false, true).is_some());
    }

    #[test]
    fn page_map_keeps_all_pages_in_retained_ranges() {
        let parent = format!("{PARENT}\n\n## Page 5 — Receipt\n\nKeep a copy of the receipt.");
        let map = MAP.replace("| 2 | 4 |", "| 2–3 | 4–5 |");
        let expected = composed_source(&parent, &map);
        assert!(expected.contains("borrowed adapters"));
        assert!(expected.contains("copy of the receipt"));
        assert!(!expected.contains("Reflection assignment"));
    }

    #[test]
    fn invalid_page_maps_keep_the_full_parent_evidence() {
        for map in [
            MAP.replace("| 2 | 4 |", "| 3 | 4 |"),
            MAP.replace("| 2 | 4 |", "| 2 | 5 |"),
            MAP.replace("| 2 | 4 |", "| 2 | 1 |"),
            MAP.replace("| 2 | 4 |", "| 2 | 4–3 |"),
            MAP.replace("| 2 | 4 |", "| 2 | 3–4 |"),
            MAP.replace(
                "January pages | Autumn pages",
                "Autumn pages | January pages",
            ),
            MAP.replace("January pages", "Revised pages"),
            format!("{MAP}\n\n{MAP}"),
            format!("```md\n{MAP}\n```"),
        ] {
            let changes = format!("# Changes\n\n{map}\n\n{CHANGE}");
            assert_eq!(
                composed_source(PARENT, &changes),
                format!("{PARENT}\n{changes}")
            );
        }
        let changes = format!(
            "# Changes\n\n{MAP}\n\n{}",
            CHANGE.replace("Page 2", "Page 3")
        );
        assert_eq!(
            composed_source(PARENT, &changes),
            format!("{PARENT}\n{changes}")
        );
        let parent = PARENT.replace("## Page 3", "## Page 2");
        let changes = format!("# Changes\n\n{MAP}\n\n{CHANGE}");
        assert_eq!(
            composed_source(&parent, &changes),
            format!("{parent}\n{changes}")
        );
    }
}

#[cfg(test)]
mod print_scope_tests {
    use super::*;

    #[test]
    fn derived_pdf_uses_only_the_requested_print_content() {
        let slides = "## 1枚目\n本文：見出し\n発表者メモ：社内用\n## 2枚目\n本文：数字";
        assert_eq!(
            print_source(
                "a.pptx",
                slides,
                "スライド面に違いなし。発表者メモは含めない。"
            ),
            "## 1枚目\n本文：見出し\n## 2枚目\n本文：数字"
        );
        assert_eq!(
            print_source(
                "a.pptx",
                slides,
                "発表者ノートを含めない。スライド面は違いなし。"
            ),
            "## 1枚目\n本文：見出し\n## 2枚目\n本文：数字"
        );
        assert_eq!(
            print_source(
                "a.pptx",
                slides,
                "違いなし。11時34分提出版の全3スライドを画面表示どおりに収録。議事メモの追加なし。"
            ),
            "## 1枚目\n本文：見出し\n## 2枚目\n本文：数字"
        );
        for request in [
            "全3スライドを画面表示どおりに収録し、発表者メモを添える。",
            "全3スライドを画面表示どおりに収録。発表者メモも掲載する。",
        ] {
            assert_eq!(print_source("a.pptx", slides, request), slides);
        }
        let sheets = "共通欄\n## シート「配布用」\n売上高\n## シート「根拠」\n内訳";
        assert_eq!(
            print_source(
                "a.xlsx",
                sheets,
                "「配布用」シートの印刷範囲のみをPDF出力する。"
            ),
            "共通欄\n## シート「配布用」\n売上高"
        );
        for request in [
            "全シートを配布",
            "「不存在」シートの印刷範囲のみをPDF出力する。",
        ] {
            assert_eq!(print_source("a.xlsx", sheets, request), sheets);
        }
        assert_eq!(
            print_source("a.pptx", slides, "発表者メモを付ける。"),
            slides
        );
    }
}

#[cfg(test)]
mod handout_and_revision_tests {
    use super::*;

    #[test]
    fn slide_face_exports_exclude_only_marked_note_lines() {
        let body = "Sales\nSpeaker notes: private\n発表者メモ：補足\nCosts";
        for request in [
            "違いなし",
            "全スライドのスライド面は元の v1.0 と同じ。発表者ノートは配布 PDF に含めない。",
            "通常のスライド表示で出力。スライドの順序と内容は元と同じ。話者ノートは出力しない。",
            "投影順のスライド面を書き出し、発表者ノートは配布面に含めない。その他の違いなし。",
            "The eight slide faces are unchanged and in the same order. Export the slide faces only; omit speaker notes. No post-meeting additions.",
        ] {
            assert_eq!(print_source("deck.pptx", body, request), "Sales\nCosts");
        }
        assert_eq!(
            print_source("deck.pptx", body, "Include speaker notes."),
            body
        );
    }

    #[test]
    fn labeled_revision_keeps_the_target_and_checks_word_evidence() {
        use std::io::Write;
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut zip = zip::ZipWriter::new(file.reopen().unwrap());
        zip.start_file(
            "word/document.xml",
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
        zip.write_all(r#"<document><ins author="石井" date="2024-07-24T06:21:00Z"><t>新しい見出し</t></ins></document>"#.as_bytes()).unwrap();
        zip.finish().unwrap();
        let line = "- 種類：挿入／人物：石井／日時：2024-07-24 15:21／対象の文：旧見出し／新しい文：新しい見出し";
        assert_eq!(
            word_revision_source(line, file.path(), chrono_tz::Asia::Tokyo).unwrap(),
            "旧見出し"
        );
        for wrong in ["別の人", "15:22", "違う見出し"] {
            let bad = line.replace(
                match wrong {
                    "別の人" => "石井",
                    "15:22" => "15:21",
                    _ => "新しい見出し",
                },
                wrong,
            );
            assert_eq!(
                word_revision_source(&bad, file.path(), chrono_tz::Asia::Tokyo).unwrap(),
                bad
            );
        }
    }

    #[test]
    fn semicolon_revisions_require_matching_word_payload_and_local_zone() {
        use std::io::Write;
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut zip = zip::ZipWriter::new(file.reopen().unwrap());
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("word/document.xml", options).unwrap();
        zip.write_all(r#"<document>
            <del author="Editor Name" date="2025-03-18T19:17:00Z"><r><delText>Remove this sentence.</delText></r></del>
            <ins author="Editor Name" date="2025-03-18T19:17:00Z"><r><t>Send the roster; request a reply.</t></r></ins>
            <del author="Editor Name" date="2025-03-18T19:17:00Z"><r><delText>“Filing held.”</delText></r></del>
        </document>"#.as_bytes()).unwrap();
        zip.start_file("word/comments.xml", options).unwrap();
        zip.write_all(br#"<comments>
            <comment author="Editor Name" date="2025-03-18T19:17:00Z"><p><r><t>Add the reply date.</t></r></p></comment>
        </comments>"#).unwrap();
        zip.finish().unwrap();
        let zone = chrono_tz::America::Denver;
        let delete = "- 種類: 削除; 人物: Editor Name; 日時: 2025-03-18 13:17 MDT; 対象の文: “Remove this sentence.”; 新しい文: なし。";
        let insert = "- 種類: 挿入; 人物: Editor Name; 日時: 2025-03-18 13:17 MDT; 対象の文: Activation; 新しい文: “Send the roster; request a reply.”";
        let comment = "- 種類: コメント; 人物: Editor Name; 日時: 2025-03-18 13:17; 対象の文: “reply”; 新しい文: “Add the reply date.”";
        let body = format!("{delete}\n{insert}\n{comment}\n{comment}");
        assert_eq!(
            word_revision_source(&body, file.path(), zone).unwrap(),
            format!("Remove this sentence.\nActivation\nreply\n{comment}")
        );
        let localized = "- 種類：コメント。人物：Editor Name。日時：2025-03-18 13:17 MDT。対象：Q24 の修正文。内容：Add the reply date.";
        assert_eq!(
            word_revision_source(localized, file.path(), zone).unwrap(),
            "Q24 の修正文 Add the reply date."
        );
        for wrong in ["MST", "Add the date."] {
            let unmatched = localized.replace(
                if wrong == "MST" {
                    "MDT"
                } else {
                    "Add the reply date."
                },
                wrong,
            );
            assert_eq!(
                word_revision_source(&unmatched, file.path(), zone).unwrap(),
                unmatched
            );
        }
        let literal = "- 種類：削除／人物：Editor Name／日時：2025-03-18 13:17／対象の文：“Filing held.”／新しい文：なし";
        assert_eq!(
            word_revision_source(literal, file.path(), zone).unwrap(),
            "“Filing held.”"
        );
        let pipes = insert
            .replace("; 人物", " | 人物")
            .replace("; 日時", " | 日時")
            .replace("; 対象の文", " | 対象")
            .replace("; 新しい文", " | 新しい文");
        assert_eq!(
            word_revision_source(&pipes, file.path(), zone).unwrap(),
            "Activation"
        );
        let wrong = pipes.replace("Send the roster", "Send the contract");
        assert_eq!(
            word_revision_source(&wrong, file.path(), zone).unwrap(),
            wrong
        );
        let multiline = insert.replacen("; ", "\n  ", 4);
        assert_eq!(
            word_revision_source(&multiline, file.path(), zone).unwrap(),
            "Activation"
        );
        let wrong = multiline.replace("Editor Name", "Other Editor");
        assert_eq!(
            word_revision_source(&wrong, file.path(), zone).unwrap(),
            wrong
        );
        let offset = insert.replace("MDT", "-0600");
        assert_eq!(
            word_revision_source(&offset, file.path(), zone).unwrap(),
            "Activation"
        );
        for wrong in [
            insert.replace("MDT", "-0700"),
            insert.replace("MDT", "MST"),
            insert.replace("13:17", "13:18"),
            insert.replace("Editor Name", "Other Name"),
            insert.replace("挿入", "コメント"),
            insert.replace("a reply", "an approval"),
            delete.replace("なし。", "別の文"),
        ] {
            assert_eq!(
                word_revision_source(&wrong, file.path(), zone).unwrap(),
                wrong
            );
        }
    }
}

#[cfg(test)]
mod workbook_manuscript_tests {
    use super::*;

    #[test]
    fn workbook_csv_cells_are_separate_from_prose_and_grouped_amounts() {
        let manuscript = "Total 8,123,456\n```csv\ndate,qty,cost\n2025-04-18,130,950\n2025-04-24,135,950\n2025-04-26,161,950\n2025-04-28,125,950\n2025-04-30,134,950\n2025-05-01,12,\"4,200\"\n```";
        let actual = "8123456 2025-04-18 130 950 2025-04-24 135 950 2025-04-26 161 950 2025-04-28 125 950 2025-04-30 134 950 2025-05-01 12 4200";
        assert_eq!(
            source_reason(manuscript, actual, false, "xlsx", false, false),
            None
        );
        for missing in ["8123456", "4200"] {
            assert!(
                source_reason(
                    manuscript,
                    &actual.replace(missing, ""),
                    false,
                    "xlsx",
                    false,
                    false
                )
                .unwrap()
                .contains(missing)
            );
        }
    }
}

#[cfg(test)]
mod derived_word_tests {
    use super::*;

    #[test]
    fn explicit_section_replacement_excludes_only_superseded_prose() {
        let dir = tempfile::tempdir().unwrap();
        let parent = "# Draft\nIssuer: BRT\n## Scope\nRetired draft wording.\n## Contacts\nContact Operations.";
        let child = "既存の節の見出しを使い、それぞれの本文を以下で差し替える。\n\n## Scope\nOnly the confirmed point is covered.";
        for (name, from, body) in [("draft", "", parent), ("issue", "draft.docx", child)] {
            std::fs::write(dir.path().join(format!("{name}.docx.md")),
                format!("---\npath: {name}.docx\nkind: sample.docx\nauthor: Editor\nderived_from: {from}\n---\n{body}")).unwrap();
        }
        let resolve = || source(dir.path(), "issue.docx", &mut Vec::new()).unwrap().1;
        let expected = resolve();
        assert!(!expected.contains("Retired draft wording."));
        for kept in [
            "Issuer: BRT",
            "Contact Operations.",
            "Only the confirmed point is covered.",
        ] {
            assert!(expected.contains(kept));
        }
        let file = dir.path().join("issue.docx.md");
        std::fs::write(
            &file,
            read(&file)
                .unwrap()
                .replace("それぞれの本文を以下で差し替える", "コメントを追加する"),
        )
        .unwrap();
        assert!(resolve().contains("Retired draft wording."));
    }
}

#[cfg(test)]
mod word_excerpt_tests {
    use super::*;

    #[test]
    fn revised_excerpt_uses_only_explicit_pages_of_slide_only_pdf() {
        let dir = tempfile::tempdir().unwrap();
        let write = |name: &str, parent: &str, kind: &str, body: &str| {
            fs::write(dir.path().join(format!("{name}.md")), format!(
                "---\npath: {name}\nkind: {kind}\nauthor: Editor\nderived_from: {parent}\n---\n{body}"
            )).unwrap();
        };
        write(
            "slides.pptx",
            "",
            "deck.pptx",
            "## スライド1\nCover\n## スライド2\nFirst table\n発表者メモ：Private\n## スライド3\nSecond table\n## スライド4\nAppendix",
        );
        write(
            "sent.pdf",
            "slides.pptx",
            "office:deck.pptx",
            "表示スライドは違いなし。発表者ノートは印刷対象に含めない。",
        );
        let resolve = |pages: &str| {
            write(
                "revision.docx",
                "sent.pdf",
                "form.docx",
                &format!(
                    "提出PDFの pp.{pages} を本文と2表を含めて抜粋し、見出しと原文を保持する。\nAdded text"
                ),
            );
            source(dir.path(), "revision.docx", &mut Vec::new())
                .unwrap()
                .1
        };
        let body = resolve("2–3");
        for retained in ["First table", "Second table", "Added text"] {
            assert!(body.contains(retained));
        }
        for excluded in ["Cover", "Private", "Appendix"] {
            assert!(!body.contains(excluded), "{excluded}");
        }
        assert!(resolve("2–9").contains("Cover"));
    }
}
