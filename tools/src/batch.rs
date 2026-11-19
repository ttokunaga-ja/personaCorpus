//! Persona batch orchestration, journals, review gates, and scoped acceptance.
use crate::check::CsvMeasurements;
use anyhow::{Result, anyhow, bail};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use fs2::FileExt;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{fd::AsRawFd, unix::process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use unicode_casefold::UnicodeCaseFold;
use walkdir::WalkDir;

const COLUMNS: [&str; 6] = ["パス", "日付", "作り方", "内容", "関連", "先に要るもの"];
const EXT: &str = "docx pdf xlsx csv tsv pptx txt md log eml html png jpg json xml wav mp3 m4a zip";
const BAD_PATH: &str = r"foundation|visual|source-art|full|(?<![a-z0-9])m1(?![a-z0-9])|primary|secondary|pdf_scan|pdf_text|txt_log|csv_tsv|html_eml|structured_text|domain_binary|imagegen";
const SOL_REBUILD: &str = "- 再制作の担当：gpt-6.2-sol";
const TRACE: &str = r"(?<!\w)(?:kio(?:-eval)?|personacorpus|persona|corpus|fixture|content\s+spine|synthetic|fictional|not\s+a\s+real|artifact\s+id|source\s+id|batch|scope\s+(?=[\w-]*\d)[\w-]+|p\d{2}-[\w-]+|lorem\s+ipsum|placeholder)(?!\w)|/Users/|scratch/";
fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
fn read(p: &Path) -> Result<String> {
    Ok(fs::read_to_string(p).map_err(|e| anyhow!("{}: {e}", p.display()))?)
}
fn write(p: &Path, s: impl AsRef<[u8]>) -> Result<()> {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(p, s)?;
    Ok(())
}
fn save(p: &Path, v: &Value) -> Result<()> {
    let temp = p.with_file_name(format!("{}.tmp", p.file_name().unwrap().to_string_lossy()));
    write(&temp, serde_json::to_vec_pretty(v)?)?;
    fs::rename(temp, p)?;
    Ok(())
}
fn matches(p: &str, s: &str) -> bool {
    fancy_regex::Regex::new(&format!("(?i){p}"))
        .unwrap()
        .is_match(s)
        .unwrap_or(false)
}
fn re(p: &str) -> regex::Regex {
    regex::Regex::new(p).unwrap()
}
fn map(v: &Value) -> Map<String, Value> {
    v.as_object().cloned().unwrap_or_default()
}
fn array(v: &Value) -> Vec<Value> {
    v.as_array().cloned().unwrap_or_default()
}
fn value_map(v: &Value) -> BTreeMap<String, String> {
    map(v)
        .iter()
        .map(|(k, v)| (k.clone(), text(v).to_string()))
        .collect()
}
fn remove(p: &Path) -> Result<()> {
    match fs::remove_file(p) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}
fn files(p: &Path) -> Vec<PathBuf> {
    if !p.exists() {
        return vec![];
    }
    let mut entries: Vec<_> = WalkDir::new(p)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();
    entries.sort();
    entries
}
fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .to_string()
}
fn cmd(root: &Path, args: &[String], check: bool) -> Result<(i32, String)> {
    if args.is_empty() {
        bail!("empty command");
    }
    let out = Command::new(&args[0])
        .args(&args[1..])
        .current_dir(root)
        .output()?;
    let code = out.status.code().unwrap_or(1);
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    if check && code != 0 {
        bail!(
            "{} exit {}: {}",
            args[0],
            code,
            output
                .lines()
                .rev()
                .take(20)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    Ok((code, output))
}
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let mut a = vec!["git".to_string()];
    a.extend(args.iter().map(|s| s.to_string()));
    Ok(cmd(root, &a, true)?.1)
}
fn bin(root: &Path, name: &str, args: &[String], check: bool) -> Result<(i32, String)> {
    let mut a = vec![root.join("bin").join(name).to_string_lossy().to_string()];
    a.extend_from_slice(args);
    cmd(root, &a, check)
}
pub fn config(p: &Path) -> HashMap<String, String> {
    config_text(&fs::read_to_string(p).unwrap_or_default())
}
fn config_text(s: &str) -> HashMap<String, String> {
    if !s.starts_with("---\n") {
        return HashMap::new();
    }
    s[4..]
        .split("\n---")
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|l| {
            l.split_once(':')
                .map(|(k, v)| (k.trim().into(), v.trim().into()))
        })
        .collect()
}
pub fn persona_name(root: &Path, s: &str) -> Result<String> {
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir(root.join("personas")) {
        for e in entries.flatten() {
            let n = e
                .path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if re(r"^p\d\d-.*$").is_match(&n)
                && (n == s || re(r"^p\d\d$").is_match(s) && n.starts_with(&format!("{s}-")))
            {
                names.push(n);
            }
        }
    }
    if names.len() != 1 {
        bail!("persona が見つからないか曖昧: {s}");
    }
    Ok(names.remove(0))
}
fn safe_file(folder: &Path, relative: &str) -> Result<PathBuf> {
    if relative.is_empty()
        || relative.starts_with('/')
        || relative.contains('\\')
        || relative
            .split('/')
            .any(|p| p.is_empty() || p.starts_with('.'))
    {
        bail!("安全でないパス: {relative}");
    }
    let target = folder.join(relative);
    let mut p = target.as_path();
    loop {
        if fs::symlink_metadata(p)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            bail!("リンクを含むパス: {relative}");
        }
        if p == folder {
            break;
        }
        match p.parent() {
            Some(x) if x.starts_with(folder) => p = x,
            _ => break,
        }
    }
    Ok(target)
}
fn split_pipe(s: &str) -> Vec<String> {
    let mut cells = vec![];
    let mut current = String::new();
    let mut previous = '\0';
    for c in s.chars() {
        if c == '|' && previous != '\\' {
            cells.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
        previous = c;
    }
    cells.push(current);
    cells
}
pub fn plan_rows(p: &Path) -> Result<Vec<Value>> {
    parse_plan(&read(p)?)
}
fn parse_plan(plan: &str) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut indices: Option<Vec<Option<usize>>> = None;
    for (number, line) in plan.lines().enumerate() {
        if !line.trim().starts_with('|') {
            continue;
        }
        let cells: Vec<_> = split_pipe(line.trim().trim_matches('|'))
            .into_iter()
            .map(|s| s.trim().trim_matches('`').to_string())
            .collect();
        if COLUMNS[..5].iter().all(|c| cells.iter().any(|s| s == c)) {
            indices = Some(
                COLUMNS
                    .iter()
                    .map(|c| cells.iter().position(|s| s == c))
                    .collect(),
            );
            continue;
        }
        let Some(ix) = &indices else { continue };
        if cells.iter().all(|s| re(r"^:?-+:?$").is_match(s)) {
            continue;
        }
        if ix.iter().flatten().any(|i| *i >= cells.len()) {
            bail!("計画の列が不足");
        }
        let mut row = Map::new();
        for (c, i) in COLUMNS.iter().zip(ix) {
            row.insert(
                (*c).into(),
                json!(i.map(|i| cells[i].clone()).unwrap_or_default()),
            );
        }
        row.insert("_line".into(), json!(number));
        row.insert("_content".into(), json!(ix[3]));
        rows.push(Value::Object(row));
    }
    if indices.is_none() {
        bail!("計画に必要な列: {}", COLUMNS.join("・"));
    }
    Ok(rows)
}
fn prerequisites(row: &Value) -> BTreeSet<String> {
    re(r"[,、;；\n]+")
        .split(text(&row["先に要るもの"]))
        .map(|s| s.trim().trim_matches('`').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
fn mark_plan(p: &Path, reasons: &BTreeMap<String, String>, marker: &str) -> Result<()> {
    if reasons.is_empty() {
        return Ok(());
    }
    let rows = plan_rows(p)?;
    let mut lines: Vec<String> = read(p)?.lines().map(str::to_string).collect();
    for row in rows {
        let Some(reason) = reasons.get(text(&row["パス"])) else {
            continue;
        };
        let reason = re(r"[\t\r\n|]")
            .replace_all(reason, " ")
            .trim_end_matches('。')
            .replace('。', "；");
        let old = text(&row["内容"]);
        let prefix = format!("{marker}：{reason}。");
        if old.starts_with(&prefix) {
            continue;
        }
        let body = if old.starts_with("破棄：") || old.starts_with("成形待ち：") {
            old.split_once('。').map(|s| s.1).unwrap_or("")
        } else {
            old
        };
        let i = row["_line"].as_u64().unwrap() as usize;
        let c = row["_content"].as_u64().unwrap() as usize + 1;
        let mut cells = split_pipe(&lines[i]);
        cells[c] = format!(" {prefix}{body} ");
        lines[i] = cells.join("|");
    }
    write(p, lines.join("\n") + "\n")
}
fn restore_plan(p: &Path, paths: &HashSet<String>, marker: &str) -> Result<()> {
    let rows = plan_rows(p)?;
    let mut lines: Vec<_> = read(p)?.lines().map(str::to_string).collect();
    for row in rows {
        if !paths.contains(text(&row["パス"])) {
            continue;
        }
        let i = row["_line"].as_u64().unwrap() as usize;
        let c = row["_content"].as_u64().unwrap() as usize + 1;
        let mut cells = split_pipe(&lines[i]);
        let body = text(&row["内容"]);
        let body = if body.starts_with(marker) {
            body.split_once('。').map(|x| x.1).unwrap_or("")
        } else {
            body
        };
        cells[c] = format!(" {body} ");
        lines[i] = cells.join("|");
    }
    write(p, lines.join("\n") + "\n")
}
thread_local! {static HELD:RefCell<HashSet<PathBuf>>=RefCell::new(HashSet::new());}
struct Lock {
    file: Option<File>,
    path: PathBuf,
}
impl Lock {
    fn new(path: PathBuf, shared: bool, wait: bool, reentrant: bool) -> Result<Self> {
        if reentrant && HELD.with(|h| h.borrow().contains(&path)) {
            return Ok(Self { file: None, path });
        }
        if let Some(p) = path.parent() {
            fs::create_dir_all(p)?;
        }
        let f = OpenOptions::new().create(true).append(true).open(&path)?;
        let result = match (shared, wait) {
            (true, true) => FileExt::lock_shared(&f),
            (true, false) => FileExt::try_lock_shared(&f),
            (false, true) => FileExt::lock_exclusive(&f),
            (false, false) => FileExt::try_lock_exclusive(&f),
        };
        result?;
        HELD.with(|h| h.borrow_mut().insert(path.clone()));
        Ok(Self {
            file: Some(f),
            path,
        })
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        if self.file.is_some() {
            HELD.with(|h| h.borrow_mut().remove(&self.path));
        }
    }
}
fn repository_lock(root: &Path, name: &str) -> Result<Lock> {
    Lock::new(
        root.join(format!("scratch/.locks/{name}.lock")),
        false,
        true,
        true,
    )
}
fn persona_lock(root: &Path, p: &str, shared: bool, wait: bool) -> Result<Lock> {
    Lock::new(root.join(format!("scratch/{p}/.lock")), shared, wait, false)
        .map_err(|_| anyhow!("同じ persona の batch が進行中: {p}"))
}
fn batch_lock(root: &Path, p: &str, b: &str, wait: bool) -> Result<Lock> {
    Lock::new(
        root.join(format!("scratch/{p}/.runtime-locks/{b}.lock")),
        false,
        wait,
        false,
    )
    .map_err(|_| anyhow!("同じ batch が進行中: {p} {b}"))
}
fn limits(root: &Path) -> Result<Value> {
    let p = root.join("config/limits.json");
    if p.exists() {
        Ok(serde_json::from_str(&read(&p)?)?)
    } else {
        Ok(json!({}))
    }
}
fn limit(root: &Path, key: &str, env: &str, default: usize) -> Result<usize> {
    let cfg = limits(root)?;
    let value = std::env::var(env).ok().unwrap_or_else(|| {
        cfg.get(key)
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .unwrap_or_else(|| v.to_string())
            })
            .unwrap_or(default.to_string())
    });
    let n = if value.chars().all(|c| c.is_ascii_digit()) {
        value.parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    if n < 1 {
        bail!("{key}（config/limits.json）は正の整数");
    }
    Ok(n)
}
fn runtime_batches(root: &Path, persona: &str) -> HashSet<String> {
    let folder = root.join(format!("scratch/{persona}/.runtime-locks"));
    let mut result = HashSet::new();
    if let Ok(es) = fs::read_dir(folder) {
        for e in es.flatten() {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("lock") {
                if let Ok(f) = OpenOptions::new().create(true).append(true).open(&p) {
                    if FileExt::try_lock_exclusive(&f).is_err() {
                        result.insert(p.file_stem().unwrap().to_string_lossy().into());
                    }
                }
            }
        }
    }
    result
}
pub fn job_batches(persona: &str, output: &str) -> HashSet<String> {
    let pattern = re(&format!(
        r"^{}-(b\d{{3,}})(?:-r\d+)?$",
        regex::escape(persona.split('-').next().unwrap_or(persona))
    ));
    output
        .lines()
        .filter_map(|l| {
            let p: Vec<_> = l.split('\t').collect();
            if p.len() >= 3 && ["running", "limit", "stdin-wait", "stalled"].contains(&p[2]) {
                pattern.captures(p[1]).map(|c| c[1].to_string())
            } else {
                None
            }
        })
        .collect()
}
fn admit(root: &Path, persona: &str, batch: &str, jobs: &str, allow_current: bool) -> Result<()> {
    let mut active = runtime_batches(root, persona);
    active.extend(job_batches(persona, jobs));
    if active.contains(batch) && !allow_current {
        bail!("同じ batch が進行中: {batch}");
    }
    active.remove(batch);
    let maximum = limit(
        root,
        "max_batches_per_persona",
        "CORPUS_MAX_BATCHES_PER_PERSONA",
        3,
    )?;
    if active.len() >= maximum {
        bail!("同じ persona の batch 上限: {maximum}");
    }
    if !active.is_empty() {
        let (code, out) = cmd(
            root,
            &[
                "git".into(),
                "show".into(),
                "HEAD:progress/batches.tsv".into(),
            ],
            false,
        )?;
        let accepted = code == 0
            && tsv_dict(&out)?.iter().any(|r| {
                r.get("persona").map(String::as_str) == Some(persona)
                    && r.get("status").map(String::as_str) == Some("accepted")
            });
        if !accepted {
            bail!("同じ persona の batch が進行中。並列には受入済み batch が必要");
        }
    }
    Ok(())
}
fn tsv(s: &str) -> Result<Vec<Vec<String>>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b'\t')
        .from_reader(s.as_bytes());
    Ok(reader
        .records()
        .map(|r| r.map(|r| r.iter().map(str::to_string).collect()))
        .collect::<std::result::Result<_, _>>()?)
}
fn tsv_dict(s: &str) -> Result<Vec<HashMap<String, String>>> {
    let rows = tsv(s)?;
    if rows.is_empty() {
        return Ok(vec![]);
    }
    Ok(rows[1..]
        .iter()
        .map(|r| rows[0].iter().cloned().zip(r.iter().cloned()).collect())
        .collect())
}
fn write_tsv(p: &Path, rows: &[Vec<String>]) -> Result<()> {
    let mut w = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_writer(vec![]);
    for row in rows {
        w.write_record(row)?;
    }
    write(p, w.into_inner()?)?;
    Ok(())
}
fn slot(root: &Path) -> Result<File> {
    let n = limit(root, "max_codex", "CORPUS_MAX_CODEX", 32)?;
    let folder = root.join("scratch/.slots");
    fs::create_dir_all(&folder)?;
    let handles: Vec<File> = (0..n)
        .map(|i| {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(folder.join(i.to_string()))
        })
        .collect::<std::io::Result<_>>()?;
    loop {
        for (i, f) in handles.iter().enumerate() {
            if FileExt::try_lock_exclusive(f).is_ok() {
                let f = handles.into_iter().nth(i).unwrap();
                return Ok(f);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
pub fn slot_usage(root: &Path) -> Result<usize> {
    let n = limit(root, "max_codex", "CORPUS_MAX_CODEX", 32)?;
    let folder = root.join("scratch/.slots");
    fs::create_dir_all(&folder)?;
    let mut used = 0;
    for i in 0..n {
        let f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(folder.join(i.to_string()))?;
        if FileExt::try_lock_exclusive(&f).is_err() {
            used += 1;
        }
    }
    Ok(used)
}
fn build_groups(root: &Path, rows: &[Value]) -> Result<Vec<Vec<Value>>> {
    let defaults = json!({"pptx":3.,"xlsx":2.,"photo":2.,"handwriting":2.,"chart":2.,"diagram":2.,"screenshot":2.,"scan":2.,"tex":1.5,"docx":1.5,"other":1.});
    let cfg = limits(root)?;
    let mut weights = map(&defaults);
    weights.extend(map(&cfg["build_weights"]));
    let maximum = match cfg.get("max_build_weight") {
        Some(v) => v
            .as_f64()
            .ok_or_else(|| anyhow!("max_build_weight は正の数"))?,
        None => 6.,
    };
    if !maximum.is_finite()
        || maximum <= 0.
        || weights.values().any(|w| {
            w.as_f64()
                .map(|w| !w.is_finite() || w <= 0.)
                .unwrap_or(true)
        })
    {
        bail!("成形の重みは正の数");
    }
    let mut result = vec![];
    let (mut group, mut total) = (vec![], 0.);
    for row in rows {
        let ext = Path::new(text(&row["パス"]))
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let method = text(&row["作り方"]).split(':').next().unwrap_or("");
        let key = if [
            "photo",
            "handwriting",
            "chart",
            "diagram",
            "screenshot",
            "scan",
        ]
        .contains(&method)
        {
            method
        } else if ["pptx", "xlsx", "docx"].contains(&ext) {
            ext
        } else if ext == "pdf" && method == "tex" {
            "tex"
        } else {
            "other"
        };
        let w = weights[key].as_f64().unwrap();
        if !group.is_empty() && total + w > maximum {
            result.push(std::mem::take(&mut group));
            total = 0.;
        }
        group.push(row.clone());
        total += w;
    }
    if !group.is_empty() {
        result.push(group);
    }
    Ok(result)
}
fn missing_explanation(output: &str, relative: &str, rows: &[Value]) -> String {
    let lines: Vec<_> = output.lines().collect();
    let name = Path::new(relative)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let starts: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.contains(relative) || l.contains(name.as_ref()))
        .map(|(i, _)| i)
        .collect();
    if starts.is_empty() {
        return output.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    let mut sections = vec![];
    for start in starts {
        let mut end = start + 1;
        while end < lines.len() {
            if lines[end].trim_start().starts_with('#')
                || rows.iter().any(|r| {
                    let p = text(&r["パス"]);
                    p != relative
                        && (lines[end].contains(p)
                            || lines[end].contains(
                                Path::new(p)
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .as_ref(),
                            ))
                })
            {
                break;
            }
            end += 1;
        }
        sections.extend_from_slice(&lines[start..end]);
    }
    sections
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn root_untracked(root: &Path) -> Result<HashSet<String>> {
    let tracked: HashSet<String> = git(root, &["ls-files", "-z"])?
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(|s| s.split('/').next().unwrap().to_string())
        .collect();
    let mut candidates: HashSet<String> =
        git(root, &["ls-files", "--others", "--exclude-standard", "-z"])?
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.split('/').next().unwrap().into())
            .collect();
    for e in fs::read_dir(root)?.flatten() {
        if e.file_type()?.is_dir() && fs::read_dir(e.path())?.next().is_none() {
            candidates.insert(e.file_name().to_string_lossy().into());
        }
    }
    let allowed:HashSet<&str>="people sources progress templates personas manifests config ops bin prompts runbooks tests tools .git".split_whitespace().collect();
    let mut result = HashSet::new();
    for name in candidates {
        if tracked.contains(&name) || allowed.contains(name.as_str()) {
            continue;
        }
        if cmd(
            root,
            &[
                "git".into(),
                "check-ignore".into(),
                "-q".into(),
                "--".into(),
                name.clone(),
            ],
            false,
        )?
        .0 != 0
        {
            result.insert(name);
        }
    }
    Ok(result)
}
struct Watch {
    root: PathBuf,
    work: PathBuf,
    stage: String,
    identity: String,
    before: HashSet<String>,
}
impl Watch {
    fn begin(root: &Path, work: &Path, stage: &str, execution: usize) -> Result<Self> {
        let _lock = Lock::new(root.join("scratch/.root-watch.lock"), false, true, false)?;
        let p = root.join("scratch/.root-watch.json");
        let mut registry = if p.exists() {
            serde_json::from_str::<Value>(&read(&p)?)?
        } else {
            json!({})
        };
        registry
            .as_object_mut()
            .unwrap()
            .retain(|_, v| unsafe { libc::kill(v["pid"].as_i64().unwrap_or(0) as i32, 0) == 0 });
        let identity = format!("{}/a{execution}", rel(root, work));
        let label = format!("{stage}、{identity}");
        let mut peers = vec![];
        for (_, v) in registry.as_object_mut().unwrap() {
            peers.push(v["label"].clone());
            v["candidates"].as_array_mut().unwrap().push(json!(label));
        }
        peers.push(json!(label));
        registry[&identity] = json!({"pid":std::process::id(),"label":label,"candidates":peers});
        let before = root_untracked(root)?;
        save(&p, &registry)?;
        Ok(Self {
            root: root.into(),
            work: work.into(),
            stage: stage.into(),
            identity,
            before,
        })
    }
    fn finish(self) -> Result<Vec<Value>> {
        let _lock = Lock::new(
            self.root.join("scratch/.root-watch.lock"),
            false,
            true,
            false,
        )?;
        let p = self.root.join("scratch/.root-watch.json");
        let mut registry: Value = serde_json::from_str(&read(&p)?)?;
        let record = registry
            .as_object_mut()
            .unwrap()
            .remove(&self.identity)
            .unwrap_or(json!({"candidates":[]}));
        let mut names: Vec<_> = root_untracked(&self.root)?
            .difference(&self.before)
            .cloned()
            .collect();
        names.sort();
        let mut records = vec![];
        for name in names {
            let source = self.root.join(&name);
            let dest = self.work.join("stray").join(&name);
            fs::create_dir_all(dest.parent().unwrap())?;
            let ignored = git(
                &self.root,
                &[
                    "ls-files",
                    "--others",
                    "--ignored",
                    "--exclude-standard",
                    "-z",
                    "--",
                    &name,
                ],
            )?;
            let outcome = if source.is_dir() && !source.is_symlink() && !ignored.is_empty() {
                for file in git(
                    &self.root,
                    &[
                        "ls-files",
                        "--others",
                        "--exclude-standard",
                        "-z",
                        "--",
                        &name,
                    ],
                )?
                .split('\0')
                .filter(|s| !s.is_empty())
                {
                    let dest = self.work.join("stray").join(file);
                    fs::create_dir_all(dest.parent().unwrap())?;
                    match fs::rename(self.root.join(file), dest) {
                        Ok(()) => (),
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                        Err(e) => return Err(e.into()),
                    }
                }
                Ok(())
            } else {
                fs::rename(source, &dest)
            };
            match outcome {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => return Err(e.into()),
            }
            records.push(json!({"name":name,"stage":self.stage,"execution":self.identity,"candidates":record["candidates"],"destination":rel(&self.root,&dest)}));
        }
        save(&p, &registry)?;
        Ok(records)
    }
}
fn stray_message(v: &Value) -> String {
    format!(
        "repo の直下に作られたファイル：{}（{}、{}）。候補の実行：{}。移動先：{}",
        text(&v["name"]),
        text(&v["stage"]),
        text(&v["execution"]),
        array(&v["candidates"])
            .iter()
            .map(text)
            .collect::<Vec<_>>()
            .join(" / "),
        text(&v["destination"])
    )
}
#[derive(Clone, Default)]
struct Args {
    persona: String,
    batch: String,
    producer: Option<String>,
    plan_only: bool,
    review_again: bool,
    draft_review_again: bool,
    rebuild: bool,
    redraft: bool,
    wait_lock: bool,
    drop: Vec<String>,
    reason: String,
    fix: String,
}
#[derive(Clone)]
struct Batch {
    root: PathBuf,
    args: Args,
    persona: String,
    batch: String,
    folder: PathBuf,
    sources: PathBuf,
    scratch: PathBuf,
    plan: PathBuf,
    request: PathBuf,
    result: PathBuf,
    definition: PathBuf,
    templates: PathBuf,
    state_path: PathBuf,
    state: Arc<Mutex<Value>>,
    cfg: HashMap<String, String>,
    prefix: String,
    planning: bool,
}
impl Batch {
    fn new(root: &Path, args: Args) -> Result<Self> {
        let persona = persona_name(root, &args.persona)?;
        if !re(r"^b\d{3,}$").is_match(&args.batch) {
            bail!("batch 名は b001 からの連番");
        }
        let batch = args.batch.clone();
        let scratch = root.join(format!("scratch/{persona}/{batch}"));
        let plan = root.join(format!("progress/{persona}/{batch}.md"));
        let definition = root.join(format!("personas/{persona}.md"));
        let state_path = scratch.join("state.json");
        let mut state = if state_path.exists() {
            serde_json::from_str(&read(&state_path)?)?
        } else {
            json!({"completed":{},"timings":{},"status":"new","stage":"new","rebuilds":0})
        };
        if !state_path.exists() {
            let ledger = root.join("progress/batches.tsv");
            if ledger.exists() {
                for r in tsv_dict(&read(&ledger)?)? {
                    if r.get("persona") == Some(&persona) && r.get("batch") == Some(&batch) {
                        state["elapsed_base"] = json!(
                            r.get("seconds")
                                .and_then(|s| s.parse::<f64>().ok())
                                .unwrap_or(0.)
                        );
                        break;
                    }
                }
            }
        }
        Ok(Self {
            root: root.into(),
            args,
            folder: root.join(format!("people/{persona}")),
            sources: root.join(format!("sources/{persona}")),
            scratch,
            plan: plan.clone(),
            request: plan.with_file_name(format!("{batch}.request.md")),
            result: plan.with_file_name(format!("{batch}.result.md")),
            definition: definition.clone(),
            templates: root.join(format!("templates/{persona}/README.md")),
            state_path,
            state: Arc::new(Mutex::new(state)),
            cfg: config(&definition),
            prefix: String::new(),
            planning: false,
            persona,
            batch,
        })
    }
    fn state(&self) -> Value {
        self.state.lock().unwrap().clone()
    }
    fn reload_state(&self) -> Result<()> {
        if self.state_path.is_file() {
            *self.state.lock().unwrap() = serde_json::from_str(&read(&self.state_path)?)?;
        }
        Ok(())
    }
    fn update<T>(&self, action: impl FnOnce(&mut Value) -> T) -> Result<T> {
        let mut state = self.state.lock().unwrap();
        let result = action(&mut state);
        save(&self.state_path, &state)?;
        Ok(result)
    }
    fn set(&self, key: &str, v: Value) -> Result<()> {
        self.update(|s| s[key] = v)
    }
    fn source(&self, p: &str) -> Result<PathBuf> {
        safe_file(&self.sources, &format!("{p}.md"))
    }
    fn scope(&self) -> Result<String> {
        let t = read(&self.request)?;
        Ok(re(r"範囲\s*[：:]\s*(.+)")
            .captures(&t)
            .map(|c| c[1].trim().to_string())
            .unwrap_or_else(|| {
                t.lines()
                    .next()
                    .unwrap_or("")
                    .trim_start_matches(['#', ' '])
                    .to_string()
            }))
    }
    fn step(
        &self,
        key: &str,
        count: usize,
        action: impl FnOnce() -> Result<Value>,
    ) -> Result<Value> {
        let key = format!("{}{key}", self.prefix);
        {
            let mut s = self.state.lock().unwrap();
            if let Some(value) = s["completed"].get(&key) {
                return Ok(value.clone());
            }
            s["stage"] = json!(key);
            s["status"] = json!("running");
            s["reason"] = json!("");
            if !s["active"].is_object() {
                s["active"] = json!({});
            }
            s["active"][&key] = json!(count);
            save(&self.state_path, &s)?;
        }
        println!("{key} 開始 {count} 件");
        let started = Instant::now();
        let result = action();
        self.update(|s| {
            if let Ok(v) = &result {
                s["completed"][&key] = v.clone();
            }
            if !s["timings"].is_object() {
                s["timings"] = json!({});
            }
            s["timings"][&key] =
                json!(s["timings"][&key].as_f64().unwrap_or(0.) + started.elapsed().as_secs_f64());
            s["active"].as_object_mut().unwrap().remove(&key);
        })?;
        if result.is_ok() {
            println!("{key} 終了 {count} 件");
        }
        result
    }
    fn shared_hashes(&self) -> Result<BTreeMap<String, String>> {
        let mut paths = files(self.templates.parent().unwrap());
        paths.push(self.definition.clone());
        paths.push(self.plan.parent().unwrap().join("facts.md"));
        hash_paths(paths, false)
    }
    fn hashes(&self, rows: &[Value], sources: bool) -> Result<BTreeMap<String, String>> {
        let rows = if sources {
            plan_rows(&self.plan)?
        } else {
            rows.to_vec()
        };
        let paths: Vec<_> = rows
            .iter()
            .map(|r| {
                if sources {
                    self.source(text(&r["パス"]))
                } else {
                    safe_file(&self.folder, text(&r["パス"]))
                }
            })
            .collect::<Result<_>>()?;
        hash_paths(paths, !sources)
    }
    fn codex(
        &self,
        stage: &str,
        rows: &[Value],
        round: usize,
        reasons: &Value,
        review: Option<&Path>,
        continuation: usize,
        continuation_output: &Value,
    ) -> Result<String> {
        let _lock = if self.planning {
            None
        } else {
            Some(persona_lock(&self.root, &self.persona, true, true)?)
        };
        let before = if self.planning {
            BTreeMap::new()
        } else {
            self.shared_hashes()?
        };
        let output = self.codex_inner(
            stage,
            rows,
            round,
            reasons,
            review,
            continuation,
            continuation_output,
        );
        if !self.planning && before != self.shared_hashes()? {
            bail!("計画の段階のあとに型・台帳・persona が変更された");
        }
        output
    }
    fn codex_inner(
        &self,
        stage: &str,
        rows: &[Value],
        round: usize,
        reasons: &Value,
        review: Option<&Path>,
        continuation: usize,
        continuation_output: &Value,
    ) -> Result<String> {
        let logs = self.scratch.join("logs");
        fs::create_dir_all(&logs)?;
        let number = self.update(|s| {
            let n = s["execution_sequence"].as_u64().unwrap_or(0) + 1;
            s["execution_sequence"] = json!(n);
            n
        })?;
        let work = self.scratch.join(format!("work/{stage}-{number:04}"));
        fs::create_dir_all(&work)?;
        let mut context = json!({"work":rel(&self.root,&work),"render":rel(&self.root,&work.join("render")),"stage":stage,"persona":self.persona,"batch":self.batch,"scratch":rel(&self.root,&self.scratch),"plan":rel(&self.root,&self.plan),"rows":rows,"round":round,"reasons":reasons,"continuation":continuation,"continuation_output":continuation_output,"review_output":review.map(|p|rel(&self.root,p)).unwrap_or_default()});
        let s = self.state();
        if stage == "draft" {
            let mut missing = Map::new();
            for r in rows {
                let p = text(&r["パス"]);
                missing.insert(
                    p.into(),
                    s["draft_missing_prerequisites"][p]
                        .as_array()
                        .map(|v| json!(v))
                        .unwrap_or(json!([])),
                );
            }
            context["missing_prerequisites"] = Value::Object(missing);
        }
        let mut previous = Map::new();
        for r in rows {
            let p = text(&r["パス"]);
            if let Some(v) = s[format!("{stage}_verdicts")].get(p) {
                previous.insert(p.into(), v.clone());
            }
        }
        context["previous_verdicts"] = json!(previous);
        if ["draft-review", "build-review"].contains(&stage) {
            context["reasons"] = Value::Object(
                previous
                    .iter()
                    .filter(|(_, v)| v[0] == "reject")
                    .map(|(p, v)| (p.clone(), v[1].clone()))
                    .collect(),
            );
        }
        let mut references = vec![
            (self.request.clone(), "依頼の範囲と受入条件を確認する"),
            (
                self.definition.clone(),
                "人物・組織・仕事の流れ・言語と日時の設定を確認する",
            ),
            (self.templates.clone(), "使う型・見本と用途を確認する"),
            (
                self.plan.parent().unwrap().join("facts.md"),
                "対象の行や依頼に関係する案件・出来事の見出しを検索して読み、共通の事実を照合する",
            ),
            (
                self.plan.clone(),
                "計画全体の文脈・関連と依存関係を確認する",
            ),
        ];
        let judgment = self.judgment_path();
        if (self.args.rebuild || self.args.redraft) && judgment.is_file() {
            references.push((judgment, "再制作の対象・判断理由・照合する資料を確認する"));
        }
        if stage == "plan" {
            for p in files(self.plan.parent().unwrap()) {
                let name = p.file_name().unwrap().to_string_lossy();
                if name.ends_with(".request.md") && p != self.request {
                    let other = name.trim_end_matches(".request.md");
                    if self
                        .root
                        .join(format!("scratch/{}/{other}/state.json", self.persona))
                        .exists()
                    {
                        references.push((p.clone(), "同じ persona の進行中の依頼と範囲を重ねない"));
                        references.push((
                            p.with_file_name(format!("{other}.md")),
                            "同じ persona の進行中の計画とパス・内容を重ねない",
                        ));
                    }
                }
            }
        }
        let mut refs = "\n## 読むファイル（パスはリポジトリ直下からの相対パス）\n".to_string();
        for (p, purpose) in references {
            refs += &format!(
                "- `{}`：{purpose}。{}\n",
                rel(&self.root, &p),
                if p.exists() {
                    ""
                } else {
                    "未作成なら読む必要はありません。"
                }
            );
        }
        let template = read(&self.root.join(format!("prompts/{stage}.md")))?;
        let request = format!("\n## 依頼\n{}", read(&self.request)?);
        let mut final_tail = String::new();
        let mut final_code = 1;
        for attempt in 0..2 {
            let execution_work = if attempt == 0 {
                work.clone()
            } else {
                work.with_file_name(format!(
                    "{}-retry{attempt}",
                    work.file_name().unwrap().to_string_lossy()
                ))
            };
            fs::create_dir_all(&execution_work)?;
            context["work"] = json!(rel(&self.root, &execution_work));
            context["render"] = json!(rel(&self.root, &execution_work.join("render")));
            let ctx = format!(
                "\nBATCH_CONTEXT\n{}\nEND_BATCH_CONTEXT\n",
                serde_json::to_string(&context)?
            );
            let parts = [
                ("ひな形", template.as_str()),
                ("参照ファイル", refs.as_str()),
                ("依頼本文", request.as_str()),
                ("BATCH_CONTEXT", ctx.as_str()),
            ];
            let total: usize = parts.iter().map(|(_, s)| s.len()).sum();
            if total > 60 * 1024 {
                bail!(
                    "指示が 60 KiB を超えています: 合計={total} bytes ({})",
                    parts
                        .iter()
                        .map(|(n, v)| format!("{n}={} bytes", v.len()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            if let Some(p) = review {
                remove(p)?;
            }
            let stem = format!("{stage}-{number:04}-a{attempt}");
            let out = logs.join(format!("{stem}.out.md"));
            write(&out, "")?;
            let logpath = logs.join(format!("{stem}.log"));
            let mut log = File::create(&logpath)?;
            let model = if ["draft", "build"].contains(&stage) {
                s["producer"].as_str().unwrap_or("gpt-6.1-terra")
            } else {
                "gpt-6.2-sol"
            };
            let slot = slot(&self.root)?;
            let watcher = Watch::begin(&self.root, &execution_work, stage, attempt)?;
            let mut command = Command::new(std::env::var("CODEX_BIN").unwrap_or("codex".into()));
            command
                .args([
                    "exec",
                    "-m",
                    model,
                    "-c",
                    "model_reasoning_effort=\"medium\"",
                    "-s",
                    "workspace-write",
                    "-c",
                    "sandbox_workspace_write.network_access=true",
                    "-C",
                ])
                .arg(&self.root)
                .arg("-o")
                .arg(&out)
                .arg("-")
                .current_dir(&self.root)
                .stdin(Stdio::piped())
                .stdout(Stdio::from(log.try_clone()?))
                .stderr(Stdio::from(log.try_clone()?));
            inherit_slot(&mut command, slot.as_raw_fd());
            let mut child = command.spawn();
            let status = (|| -> Result<i32> {
                let c = child.as_mut().map_err(|e| anyhow!(e.to_string()))?;
                c.stdin
                    .take()
                    .unwrap()
                    .write_all(parts.iter().map(|(_, v)| *v).collect::<String>().as_bytes())?;
                Ok(c.wait()?.code().unwrap_or(1))
            })();
            let records = watcher.finish()?;
            self.update(|s| {
                if !s["strays"].is_array() {
                    s["strays"] = json!([]);
                }
                s["strays"].as_array_mut().unwrap().extend(records.clone());
            })?;
            for record in &records {
                let message = stray_message(record);
                writeln!(log, "{message}")?;
                println!("{message}");
            }
            drop(slot);
            final_code = status?;
            if final_code == 0 {
                return if out.exists() {
                    read(&out)
                } else {
                    Ok(String::new())
                };
            }
            final_tail = read(&logpath)?
                .lines()
                .rev()
                .take(20)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
        }
        self.set("failure_tail", json!(final_tail))?;
        if matches("usage limit|rate limit", &final_tail) {
            bail!("利用上限");
        }
        bail!("codex exit {final_code}\n{final_tail}")
    }
    fn timestamp(&self, row: &Value) -> Result<DateTime<Utc>> {
        let raw = text(&row["日付"]);
        if let Ok(t) = DateTime::parse_from_rfc3339(raw) {
            return Ok(t.with_timezone(&Utc));
        }
        let date = if raw.contains('T') || raw.contains(' ') {
            // Plan dates are written to the minute ("2026-03-06 18:42") or the second.
            [
                "%Y-%m-%dT%H:%M:%S",
                "%Y-%m-%d %H:%M:%S",
                "%Y-%m-%dT%H:%M",
                "%Y-%m-%d %H:%M",
            ]
            .iter()
            .find_map(|f| NaiveDateTime::parse_from_str(raw, f).ok())
            .ok_or_else(|| anyhow!("日付を読めない: {raw}"))?
        } else {
            NaiveDate::parse_from_str(raw, "%Y-%m-%d")?
                .and_hms_opt(12, 0, 0)
                .unwrap()
        };
        let zone = self
            .cfg
            .get("timezone")
            .map(String::as_str)
            .unwrap_or("UTC")
            .parse::<chrono_tz::Tz>()?;
        Ok(zone
            .from_local_datetime(&date)
            .single()
            .ok_or_else(|| anyhow!("日時が曖昧"))?
            .with_timezone(&Utc))
    }
    fn trace(&self, body: &str) -> bool {
        let allowed = self
            .cfg
            .get("allowed_terms")
            .map(String::as_str)
            .unwrap_or("");
        let ranges: Vec<_> = allowed
            .split(',')
            .filter(|s| s.split_whitespace().count() >= 2)
            .flat_map(|term| {
                let pat = format!(
                    r"(?i)(?<!\w){}(?!\w)",
                    term.split_whitespace()
                        .map(regex::escape)
                        .collect::<Vec<_>>()
                        .join(r"\s+")
                );
                fancy_regex::Regex::new(&pat)
                    .unwrap()
                    .find_iter(body)
                    .filter_map(|r| r.ok())
                    .map(|m| (m.start(), m.end()))
                    .collect::<Vec<_>>()
            })
            .collect();
        fancy_regex::Regex::new(&format!("(?i){TRACE}"))
            .unwrap()
            .find_iter(body)
            .filter_map(|r| r.ok())
            .any(|m| !ranges.iter().any(|(a, b)| *a <= m.start() && m.end() <= *b))
    }
    fn validate_plan(&self) -> Result<Vec<Value>> {
        let rows = plan_rows(&self.plan)?;
        let request = read(&self.request)?;
        let capture = re(r"件数の目安\s*[：:]\s*(\d+)")
            .captures(&request)
            .ok_or_else(|| anyhow!("依頼に件数の目安がない"))?;
        let target = capture[1].parse::<usize>()?;
        if target == 0 {
            bail!("件数の目安は正の整数");
        }
        let mut errors = vec![];
        if (rows.len() as f64) < 0.5 * target as f64
            || rows.len() as f64 > (1.2 * target as f64).min(60.)
        {
            errors.push(format!(
                "件数 {} は目安 {target} の 0.5〜1.2 倍かつ 60 件以下ではない",
                rows.len()
            ));
        }
        let period = self
            .cfg
            .get("period")
            .ok_or_else(|| anyhow!("period がない"))?;
        let (start, end) = period
            .split_once("..")
            .ok_or_else(|| anyhow!("period 不正"))?;
        let mut seen = HashSet::new();
        for row in &rows {
            let relative = text(&row["パス"]);
            let result = (|| -> Result<()> {
                let p = safe_file(&self.folder, relative)?;
                if p.exists()
                    || self.source(relative)?.exists()
                    || !seen.insert(relative.to_string())
                {
                    errors.push(format!("既存または重複パス: {relative}"));
                }
                let ext = p
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();
                if !EXT.split_whitespace().any(|s| s == ext) {
                    errors.push(format!("対象外の拡張子: {relative}"));
                }
                if matches(BAD_PATH, relative)
                    || matches(
                        r"(?<![a-z0-9])p\d{2}(?!\d)|-s\d{6}(?!\d)|(?<!\d)\d{5,}(?!\d)",
                        p.file_name().unwrap_or_default().to_string_lossy().as_ref(),
                    )
                {
                    errors.push(format!("制作区分または ID: {relative}"));
                }
                if self.trace(relative) {
                    errors.push(format!("制作の痕跡: {relative}"));
                }
                let stamp = self.timestamp(row)?;
                let local = if let Ok(explicit) = DateTime::parse_from_rfc3339(text(&row["日付"]))
                {
                    explicit.date_naive()
                } else {
                    stamp
                        .with_timezone(
                            &self
                                .cfg
                                .get("timezone")
                                .map(String::as_str)
                                .unwrap_or("UTC")
                                .parse::<chrono_tz::Tz>()?,
                        )
                        .date_naive()
                };
                if local < NaiveDate::parse_from_str(start, "%Y-%m-%d")?
                    || local > NaiveDate::parse_from_str(end, "%Y-%m-%d")?
                {
                    errors.push(format!("期間外: {relative}"));
                }
                let method = text(&row["作り方"]);
                let mut template = None;
                if ["docx", "xlsx", "pptx"].contains(&ext.as_str()) {
                    template = Some(method);
                } else if ext == "pdf" {
                    if !["office:", "tex:", "scan:"]
                        .iter()
                        .any(|s| method.starts_with(s))
                    {
                        errors.push(format!("PDF の作り方が不正: {relative}"));
                    } else if method.starts_with("office:") || method.starts_with("tex:") {
                        template = method.split_once(':').map(|x| x.1);
                    } else if method.starts_with("scan:office:") || method.starts_with("scan:tex:")
                    {
                        template = method.splitn(3, ':').nth(2);
                    }
                } else if ["png", "jpg"].contains(&ext.as_str()) {
                    if !["photo", "screenshot", "chart", "diagram"].contains(&method) {
                        errors.push(format!("画像の作り方が不正: {relative}"));
                    }
                } else if method != "text" {
                    errors.push(format!("作り方は text: {relative}"));
                }
                if let Some(t) = template {
                    let inventory = fs::read_to_string(&self.templates).unwrap_or_default();
                    if t.is_empty()
                        || !matches(
                            &format!(r"(?<![\w.-]){}(?![\w.-])", regex::escape(t)),
                            &inventory,
                        )
                    {
                        errors.push(format!("一覧にないテンプレート: {t}"));
                    }
                }
                if text(&row["内容"]).is_empty() || text(&row["内容"]).starts_with("破棄：")
                {
                    errors.push(format!("制作前の内容が空か破棄: {relative}"));
                }
                Ok(())
            })();
            if let Err(e) = result {
                errors.push(format!("{relative}: {e}"));
            }
        }
        let mut graph: BTreeMap<String, BTreeSet<String>> = rows
            .iter()
            .map(|r| (text(&r["パス"]).into(), prerequisites(r)))
            .collect();
        let known: HashSet<_> = graph.keys().cloned().collect();
        for (p, dependencies) in &graph {
            if dependencies.iter().any(|d| !known.contains(d)) {
                errors.push(format!("先に要るものは同じ batch の行: {p}"));
            }
        }
        while !graph.is_empty() {
            let ready: Vec<_> = graph
                .iter()
                .filter(|(_, deps)| !deps.iter().any(|d| graph.contains_key(d)))
                .map(|(p, _)| p.clone())
                .collect();
            if ready.is_empty() {
                errors.push("先に要るものが循環している".into());
                break;
            }
            for p in ready {
                graph.remove(&p);
            }
        }
        for other in files(self.plan.parent().unwrap()) {
            if other != self.plan
                && re(r"^b\d+\.md$").is_match(
                    other
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .as_ref(),
                )
                && self
                    .root
                    .join(format!(
                        "scratch/{}/{}/state.json",
                        self.persona,
                        other.file_stem().unwrap().to_string_lossy()
                    ))
                    .exists()
            {
                let mut overlap: Vec<_> = plan_rows(&other)?
                    .into_iter()
                    .filter(|row| known.contains(text(&row["パス"])))
                    .collect();
                if overlap
                    .iter()
                    .any(|row| text(&row["内容"]).starts_with("破棄："))
                {
                    let prior = Batch::new(
                        &self.root,
                        Args {
                            persona: self.persona.clone(),
                            batch: other.file_stem().unwrap().to_string_lossy().into_owned(),
                            ..Args::default()
                        },
                    )?;
                    if matches!(text(&prior.state()["status"]), "accepted" | "discarded") {
                        let (redraft, _) = prior.redraft_candidates()?;
                        overlap.retain(|row| {
                            !text(&row["内容"]).starts_with("破棄：")
                                || redraft.iter().any(|pending| pending["パス"] == row["パス"])
                        });
                    }
                }
                let overlap: Vec<_> = overlap.iter().map(|row| text(&row["パス"])).collect();
                if !overlap.is_empty() {
                    errors.push(format!("進行中の batch と重複パス: {}", overlap.join(", ")));
                }
            }
        }
        if !errors.is_empty() {
            bail!("{}", errors.join("\n"));
        }
        Ok(rows)
    }
    fn make_plan(&self) -> Result<Value> {
        let mut reasons = json!({});
        for _ in 0..2 {
            self.codex("plan", &[], 0, &reasons, None, 0, &json!({}))?;
            match self.validate_plan() {
                Ok(rows) => return Ok(json!(rows)),
                Err(e) => reasons = json!({"計画":e.to_string()}),
            }
        }
        bail!("計画不合格: {}", text(&reasons["計画"]))
    }
    fn event(
        &self,
        stage: &str,
        round: usize,
        criterion: &str,
        path: &str,
        reason: &str,
    ) -> Result<()> {
        let item = json!([
            Utc::now().date_naive().to_string(),
            self.persona,
            self.batch,
            stage,
            round.to_string(),
            if criterion.is_empty() {
                "other"
            } else {
                criterion
            },
            path,
            re(r"[\t\r\n]+").replace_all(reason, " ").trim()
        ]);
        self.update(|s| {
            if !s["events"].is_array() {
                s["events"] = json!([]);
            }
            let events = s["events"].as_array_mut().unwrap();
            if !events
                .iter()
                .any(|e| e.as_array().unwrap()[1..] == item.as_array().unwrap()[1..])
            {
                events.push(item);
            }
        })
    }
    fn record_tables(
        &self,
        status: Option<&str>,
        coordinator: Option<usize>,
        templates_only: bool,
    ) -> Result<()> {
        let _lock = repository_lock(&self.root, "records")?;
        let progress = self.root.join("progress");
        fs::create_dir_all(&progress)?;
        let directory = File::open(&progress)?;
        FileExt::lock_exclusive(&directory)?;
        let event_header: Vec<String> = [
            "date",
            "persona",
            "batch",
            "stage",
            "round",
            "criterion",
            "path",
            "reason",
        ]
        .iter()
        .map(|s| (*s).into())
        .collect();
        let mut events = load_table(&progress.join("findings.tsv"), &event_header)?;
        let s = self.state();
        for item in array(&s["events"]) {
            let row: Vec<String> = array(&item).iter().map(|v| text(v).to_string()).collect();
            if row.len() != 8 {
                continue;
            }
            if row[3] == "check" && row[5] == "dependency" {
                continue;
            }
            if !events.iter().any(|e| e[1..] == row[1..]) {
                events.push(row);
            }
        }
        let mut all = vec![event_header];
        all.extend(events.clone());
        atomic_tsv(&progress.join("findings.tsv"), &all)?;
        if templates_only {
            return Ok(());
        }
        let header: Vec<String> = [
            "persona",
            "batch",
            "date",
            "planned",
            "draft_initial_rejects",
            "build_initial_rejects",
            "final_rejects",
            "coordinator_rejects",
            "status",
            "seconds",
        ]
        .iter()
        .map(|s| (*s).into())
        .collect();
        let mut batches = load_table(&progress.join("batches.tsv"), &header)?;
        let old = batches
            .iter()
            .find(|r| r[0] == self.persona && r[1] == self.batch)
            .cloned();
        let rows = if self.plan.exists() {
            plan_rows(&self.plan)?
        } else {
            vec![]
        };
        let initial = |stages: &[&str]| -> usize {
            events
                .iter()
                .filter(|e| {
                    e[1] == self.persona
                        && e[2] == self.batch
                        && stages.contains(&e[3].as_str())
                        && e[4] == "0"
                })
                .map(|e| e[6].clone())
                .collect::<HashSet<_>>()
                .len()
        };
        let keep = old
            .as_ref()
            .map(|r| ["done", "accepted", "discarded"].contains(&r[8].as_str()))
            .unwrap_or(false);
        let elapsed = s["elapsed_base"].as_f64().unwrap_or(0.)
            + map(&s["timings"])
                .iter()
                .filter(|(k, _)| !["draft-complete", "build-complete"].contains(&k.as_str()))
                .map(|(_, v)| v.as_f64().unwrap_or(0.))
                .sum::<f64>();
        let row = vec![
            self.persona.clone(),
            self.batch.clone(),
            old.as_ref()
                .map(|r| r[2].clone())
                .unwrap_or(Utc::now().date_naive().to_string()),
            rows.len().to_string(),
            if keep {
                old.as_ref().unwrap()[4].clone()
            } else {
                initial(&["draft-check", "draft-review"]).to_string()
            },
            if keep {
                old.as_ref().unwrap()[5].clone()
            } else {
                initial(&["check", "build-review"]).to_string()
            },
            rows.iter()
                .filter(|r| marked(text(&r["内容"])))
                .count()
                .to_string(),
            coordinator
                .map(|n| n.to_string())
                .unwrap_or_else(|| old.as_ref().map(|r| r[7].clone()).unwrap_or("0".into())),
            status
                .unwrap_or(if s["status"] == "failed" {
                    "failed"
                } else {
                    "done"
                })
                .to_string(),
            format!("{elapsed:.2}"),
        ];
        if let Some(i) = batches
            .iter()
            .position(|r| r[0] == self.persona && r[1] == self.batch)
        {
            batches[i] = row;
        } else {
            batches.push(row);
        }
        let mut all = vec![header];
        all.extend(batches);
        atomic_tsv(&progress.join("batches.tsv"), &all)
    }
    fn review_templates(&self, fix: Option<&str>) -> Result<Value> {
        let mut reasons = if let Some(f) = fix {
            json!({"型":f})
        } else {
            json!({})
        };
        for round in 0..3 {
            if !map(&reasons).is_empty() {
                self.codex("templates", &[], round, &reasons, None, 0, &json!({}))?;
            }
            let out = self.scratch.join(format!("templates-review-r{round}.tsv"));
            let paths = files(self.templates.parent().unwrap());
            let before = hash_paths(paths.clone(), false)?;
            let inventory: Vec<_> = paths
                .iter()
                .map(|p| json!({"パス":rel(self.templates.parent().unwrap(),p)}))
                .collect();
            self.codex(
                "templates-review",
                &inventory,
                round,
                &json!({}),
                Some(&out),
                0,
                &json!({}),
            )?;
            if before != hash_paths(files(self.templates.parent().unwrap()), false)? {
                bail!("テンプレートのレビュー中に型が変更された");
            }
            let entries = if out.exists() {
                tsv(&read(&out)?)?
            } else {
                vec![]
            };
            let expected: HashSet<_> = paths
                .iter()
                .map(|p| rel(self.templates.parent().unwrap(), p))
                .collect();
            let mut seen = HashSet::new();
            let mut rejected = BTreeMap::new();
            for e in &entries {
                if ![3, 4].contains(&e.len())
                    || !["pass", "reject"].contains(&e[1].as_str())
                    || !expected.contains(&e[0])
                {
                    continue;
                }
                if !seen.insert(e[0].clone()) {
                    rejected.insert(e[0].clone(), "判定が重複".to_string());
                }
                if e[1] == "reject" {
                    rejected.insert(
                        e[0].clone(),
                        if e.last().unwrap().is_empty() {
                            "理由なし".into()
                        } else {
                            e.last().unwrap().clone()
                        },
                    );
                }
            }
            for p in expected.difference(&seen) {
                if p != "README.md" {
                    rejected.insert(p.clone(), "判定なし".into());
                }
            }
            if entries.is_empty() || expected.is_empty() {
                rejected.insert("README.md".into(), "判定なし、または型がない".into());
            }
            for (p, r) in &rejected {
                self.event("templates-review", round, "other", p, r)?;
            }
            if rejected.is_empty() {
                return Ok(Value::Null);
            }
            reasons = json!(rejected);
        }
        self.record_tables(None, None, true)?;
        bail!("テンプレートのレビュー不合格: {reasons}")
    }
    fn dependencies(&self, rows: &[Value]) -> Result<BTreeMap<String, BTreeSet<String>>> {
        let candidates: BTreeSet<String> = plan_rows(&self.plan)?
            .iter()
            .map(|r| text(&r["パス"]).into())
            .collect();
        let declaration = re(r"(?i)^\s*(?:[-*]\s*)?(?:添付(?:ファイル)?|attachments?)\s*[:：](.*)");
        let heading = re(
            r"(?i)^[ \t]{0,3}#{1,6}[ \t]+(?:添付(?:ファイル)?|attachments?)(?:[ \t]+#+)?[ \t]*$",
        );
        let bullet = re(r"^\s*[-*]\s+");
        let attachments = re(&format!(
            r#"(?:[\w. -]+/)*[^/`\[\],;:：\n]+?\.(?:{})(?:$|[\s`\],;（(])"#,
            EXT.split_whitespace().collect::<Vec<_>>().join("|")
        ));
        let mut result = BTreeMap::new();
        for row in rows {
            let relative = text(&row["パス"]);
            let source = self.source(relative)?;
            let cfg = config(&source);
            let mut dependencies = BTreeSet::new();
            if let Some(p) = cfg.get("derived_from").filter(|p| !p.is_empty()) {
                dependencies.insert(p.clone());
            }
            let mut lines = vec![];
            let mut in_attachment = false;
            for line in fs::read_to_string(&source).unwrap_or_default().lines() {
                if let Some(c) = declaration.captures(line) {
                    in_attachment = true;
                    lines.push(c[1].to_string());
                } else if heading.is_match(line) {
                    in_attachment = true;
                } else if in_attachment && bullet.is_match(line) {
                    lines.push(line.to_string());
                } else if !line.trim().is_empty() {
                    in_attachment = false;
                }
            }
            let all = lines.join("\n");
            for c in &candidates {
                if c != relative && all.contains(c) {
                    dependencies.insert(c.clone());
                }
            }
            if relative.to_lowercase().ends_with(".eml") {
                for line in &lines {
                    for matched in attachments.find_iter(line) {
                        let mut candidate = matched
                            .as_str()
                            .trim_matches(|c: char| c.is_whitespace() || "-\"'`],;（(".contains(c))
                            .to_string();
                        if !candidate.contains('/') {
                            let nearby = Path::new(relative)
                                .parent()
                                .unwrap_or(Path::new(""))
                                .join(&candidate)
                                .to_string_lossy()
                                .to_string();
                            let matching: Vec<_> = candidates
                                .iter()
                                .filter(|p| {
                                    Path::new(p).file_name()
                                        == Some(std::ffi::OsStr::new(&candidate))
                                })
                                .collect();
                            if candidates.contains(&nearby)
                                || safe_file(&self.folder, &nearby)?.is_file()
                            {
                                candidate = nearby;
                            } else if matching.len() == 1 {
                                candidate = matching[0].clone();
                            } else {
                                let found: Vec<_> = files(&self.folder)
                                    .into_iter()
                                    .filter(|p| {
                                        p.file_name() == Some(std::ffi::OsStr::new(&candidate))
                                    })
                                    .collect();
                                candidate = if found.len() == 1 {
                                    rel(&self.folder, &found[0])
                                } else {
                                    nearby
                                };
                            }
                        }
                        if candidates.contains(&candidate)
                            || safe_file(&self.folder, &candidate)?.is_file()
                        {
                            dependencies.insert(candidate);
                        }
                    }
                }
            }
            dependencies.remove(relative);
            result.insert(relative.into(), dependencies);
        }
        Ok(result)
    }
    fn draft_check(&self, rows: &[Value]) -> Result<Value> {
        let mut failures = BTreeMap::<String, String>::new();
        let mut comparisons = vec![];
        let mut measurements = HashMap::new();
        let mut related = HashSet::<(String, String)>::new();
        for p in files(self.plan.parent().unwrap()) {
            if !re(r"^b\d+\.md$")
                .is_match(p.file_name().unwrap_or_default().to_string_lossy().as_ref())
            {
                continue;
            }
            for row in plan_rows(&p)? {
                let mut refs = prerequisites(&row);
                refs.extend(
                    re(r"[,、;；\n]+")
                        .split(text(&row["関連"]))
                        .map(|s| s.trim().trim_matches('`').to_string())
                        .filter(|s| !s.is_empty()),
                );
                for other in refs {
                    let a = text(&row["パス"]).to_string();
                    related.insert(if a < other { (a, other) } else { (other, a) });
                }
            }
        }
        let expected: HashMap<String, Value> = rows
            .iter()
            .map(|r| (text(&r["パス"]).to_string(), r.clone()))
            .collect();
        let mut paths: BTreeSet<PathBuf> = files(&self.sources)
            .into_iter()
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
            .collect();
        for p in expected.keys() {
            paths.insert(self.source(p)?);
        }
        for path in paths {
            let relative = rel(&self.sources, &path)
                .trim_end_matches(".md")
                .to_string();
            let row = expected.get(&relative);
            let check = (|| -> Result<Option<(String, [u8; 32], BTreeSet<[u8; 8]>)>> {
                let text = read(&path)?;
                let body = text
                    .strip_prefix("---\n")
                    .and_then(|s| s.split_once("\n---"))
                    .and_then(|(_, b)| b.strip_prefix('\n').or_else(|| b.is_empty().then_some("")))
                    .ok_or_else(|| anyhow!("frontmatter がない"))?
                    .trim();
                let cfg = config(&path);
                if cfg.get("path").map(String::as_str) != Some(relative.as_str())
                    || row
                        .map(|r| {
                            cfg.get("kind").map(String::as_str) != Some(text_value(&r["作り方"]))
                        })
                        .unwrap_or(false)
                {
                    bail!("frontmatter path/kind が計画と違う");
                }
                if body.is_empty() {
                    bail!("本文が空");
                }
                if self.trace(body) {
                    bail!("制作の痕跡");
                }
                if let Some(m)=re(r"本文への埋め込み|【[^】\n]{0,60}(?:削除・取り消し線|赤字の挿入|赤字追記)[^】\n]{0,20}】").find(body){bail!("制作の痕跡（変更履歴・添付の扱いを本文に説明で書いている）: {}",m.as_str());}
                if let Some(parent) = cfg.get("derived_from").filter(|s| !s.is_empty()) {
                    let missing = array(&self.state()["draft_missing_prerequisites"][&relative]);
                    if !self.source(parent)?.is_file() && !missing.iter().any(|p| p == parent) {
                        bail!(
                            "derived_from の元の原稿がない（元が成果物でないなら derived_from を空にして本文に全文を書く）: {parent}"
                        );
                    }
                    return Ok(None);
                }
                let digest: [u8; 32] = Sha256::digest(body.as_bytes()).into();
                let normalized = re(r"\d+")
                    .replace_all(&body.case_fold().collect::<String>(), "0")
                    .to_string();
                let words: Vec<String> =
                    re(r"[\u3040-\u30ff\u3400-\u9fff]|[^\W_\u3040-\u30ff\u3400-\u9fff]+")
                        .find_iter(&normalized)
                        .map(|m| m.as_str().to_string())
                        .collect();
                let grams = words
                    .windows(5)
                    .map(|w| {
                        use blake2::digest::{Update, VariableOutput};
                        let mut digest = blake2::Blake2bVar::new(8).unwrap();
                        digest.update(w.join(" ").as_bytes());
                        let mut g = [0; 8];
                        digest.finalize_variable(&mut g).unwrap();
                        g
                    })
                    .collect();
                if let Some(values) = CsvMeasurements::from_draft(&relative, body) {
                    measurements.insert(relative.clone(), values);
                }
                Ok(Some((relative.clone(), digest, grams)))
            })();
            match check {
                Ok(Some(c)) => comparisons.push(c),
                Ok(None) => (),
                Err(e) => {
                    if row.is_some() {
                        failures.insert(relative, e.to_string());
                    }
                }
            }
        }
        let reject_pair =
            |a: &str, b: &str, why: &str, exact: bool, failures: &mut BTreeMap<String, String>| {
                let pair = if a < b {
                    (a.to_string(), b.to_string())
                } else {
                    (b.to_string(), a.to_string())
                };
                let distinct_measurements = measurements
                    .get(a)
                    .zip(measurements.get(b))
                    .is_some_and(|(a, b)| a.differs_from(b));
                if a != b && (exact || !(related.contains(&pair) || distinct_measurements)) {
                    if expected.contains_key(a) {
                        failures
                            .entry(a.into())
                            .or_insert_with(|| format!("duplicate: {why}: {b}"));
                    }
                    if expected.contains_key(b) {
                        failures
                            .entry(b.into())
                            .or_insert_with(|| format!("duplicate: {why}: {a}"));
                    }
                }
            };
        let mut hashes: BTreeMap<[u8; 32], Vec<String>> = BTreeMap::new();
        let mut groups: BTreeMap<BTreeSet<[u8; 8]>, Vec<String>> = BTreeMap::new();
        for (p, d, g) in comparisons {
            hashes.entry(d).or_default().push(p.clone());
            if !g.is_empty() {
                groups.entry(g).or_default().push(p);
            }
        }
        for paths in hashes.values() {
            for i in 0..paths.len() {
                for j in i + 1..paths.len() {
                    reject_pair(&paths[i], &paths[j], "identical bytes", true, &mut failures);
                }
            }
        }
        for paths in groups.values() {
            for i in 0..paths.len() {
                for j in i + 1..paths.len() {
                    reject_pair(
                        &paths[i],
                        &paths[j],
                        "identical normalized shingles",
                        false,
                        &mut failures,
                    );
                }
            }
        }
        let mut frequency = HashMap::new();
        for g in groups.keys() {
            for gram in g {
                *frequency.entry(*gram).or_insert(0usize) += 1;
            }
        }
        let mut sorted: Vec<_> = groups.iter().collect();
        sorted.sort_by_key(|(g, _)| g.len());
        let mut index: HashMap<[u8; 8], Vec<usize>> = HashMap::new();
        for (i, (grams, paths)) in sorted.iter().enumerate() {
            let n = grams.len();
            let mut ordered: Vec<_> = grams.iter().copied().collect();
            ordered.sort_by_key(|g| (frequency[g], *g));
            let prefix = n - (4 * n).div_ceil(5) + 1;
            let candidates: HashSet<usize> = ordered[..prefix]
                .iter()
                .flat_map(|g| index.get(g).cloned().unwrap_or_default())
                .filter(|j| 5 * sorted[*j].0.len() >= 4 * n)
                .collect();
            for j in candidates {
                let (other, op) = sorted[j];
                let intersection = grams.intersection(other).count();
                if 5 * intersection >= 4 * (n + other.len() - intersection) {
                    for a in *paths {
                        for b in op {
                            reject_pair(
                                a,
                                b,
                                "normalized 5-word Jaccard >= 0.8",
                                false,
                                &mut failures,
                            );
                        }
                    }
                }
            }
            for g in &ordered[..prefix] {
                index.entry(*g).or_default().push(i);
            }
        }
        Ok(json!(failures))
    }
    fn check(&self, round: usize, rows: &[Value]) -> Result<Value> {
        let report = self.scratch.join(format!(
            "{}check-r{round}.tsv",
            self.prefix.replace('/', "-")
        ));
        remove(&report)?;
        let (code, out) = bin(
            &self.root,
            "check",
            &[
                self.persona.clone(),
                "--batch".into(),
                self.batch.clone(),
                "--report".into(),
                report.to_string_lossy().to_string(),
            ],
            false,
        )?;
        write(
            &self.scratch.join(format!(
                "logs/{}check-r{round}.log",
                self.prefix.replace('/', "-")
            )),
            &out,
        )?;
        if !report.exists() {
            bail!("bin/check が報告を書かなかった: {}", tail_chars(&out, 2000));
        }
        let known: HashSet<_> = rows.iter().map(|r| text(&r["パス"]).to_string()).collect();
        let mut failures = BTreeMap::<String, String>::new();
        for e in tsv(&read(&report)?)? {
            if e.len() != 4 || e[0] != self.persona {
                bail!("bin/check の報告形式が不正");
            }
            if known.contains(&e[1]) && !(e[2] == "plan" && e[3] == "planned file missing") {
                self.event("check", round, &e[2], &e[1], &e[3])?;
            }
            *failures.entry(e[1].clone()).or_default() += &format!("{}: {} ", e[2], e[3]);
        }
        let outside: HashSet<_> = failures
            .keys()
            .filter(|p| !known.contains(*p))
            .cloned()
            .collect();
        let planned: HashSet<_> = plan_rows(&self.plan)?
            .iter()
            .map(|r| text(&r["パス"]).to_string())
            .collect();
        if outside.difference(&planned).next().is_some() {
            bail!(
                "batch 外の検査不合格: {:?}",
                outside.difference(&planned).collect::<Vec<_>>()
            );
        }
        failures.retain(|p, _| known.contains(p));
        if code != 0 && failures.is_empty() && outside.is_empty() {
            bail!(
                "bin/check 未完了（検査不能または OCR 未実行）: {}",
                tail_chars(&out, 2000)
            );
        }
        Ok(json!(failures))
    }
    fn review(
        &self,
        stage: &str,
        rows: &[Value],
        all: &[Value],
        round: usize,
        group: usize,
    ) -> Result<Value> {
        let out = self.scratch.join(format!(
            "{}{stage}-r{round}-g{group}.tsv",
            self.prefix.replace('/', "-")
        ));
        remove(&out)?;
        let sources = self.hashes(&[], true)?;
        let artifacts = self.hashes(all, false)?;
        let notes = self.codex(stage, rows, round, &json!({}), Some(&out), 0, &json!({}));
        if sources != self.hashes(&[], true)? || artifacts != self.hashes(all, false)? {
            bail!("レビュー中に原稿または成果物が変更された");
        }
        let notes = notes?;
        self.update(|s| {
            let key = format!("{stage}_notes");
            if !s[&key].is_array() {
                s[&key] = json!([]);
            }
            s[&key].as_array_mut().unwrap().push(json!(format!(
                "{}round {round} / 組 {}\n\n{notes}",
                self.prefix,
                group + 1
            )));
        })?;
        let mut verdicts = Map::new();
        for e in if out.exists() {
            tsv(&read(&out)?)?
        } else {
            vec![]
        } {
            if ![3, 4].contains(&e.len()) || !["pass", "reject"].contains(&e[1].as_str()) {
                continue;
            }
            let (criterion, reason) = if e.len() == 4 {
                (e[2].clone(), e[3].clone())
            } else {
                ("other".into(), e[2].clone())
            };
            let valid = if stage == "draft-review" {
                re(r"^D(?:10|[1-9])$").is_match(&criterion)
            } else {
                re(r"^V[1-6]$").is_match(&criterion)
            };
            let criterion = if valid { criterion } else { "other".into() };
            let v = if verdicts.contains_key(&e[0]) {
                json!(["reject", "判定が重複", "other"])
            } else {
                json!([
                    e[1],
                    if reason.is_empty() {
                        "理由なし"
                    } else {
                        &reason
                    },
                    if e[1] == "reject" {
                        criterion.as_str()
                    } else {
                        ""
                    }
                ])
            };
            verdicts.insert(e[0].clone(), v);
        }
        let mut result = Map::new();
        for r in rows {
            let p = text(&r["パス"]);
            let v = verdicts
                .get(p)
                .cloned()
                .unwrap_or(json!(["reject", "判定なし", "other"]));
            if v[0] == "reject" {
                self.event(stage, round, text(&v[2]), p, text(&v[1]))?;
            }
            result.insert(p.into(), v);
        }
        self.update(|s| {
            let key = format!("{stage}_verdicts");
            let mut values = map(&s[&key]);
            values.extend(result.clone());
            s[&key] = json!(values);
        })?;
        Ok(json!(result))
    }
    fn protected(&self, rows: &[Value], artifacts_only: bool) -> Result<()> {
        let tracked: HashSet<_> = cmd(
            &self.root,
            &[
                "git".into(),
                "ls-tree".into(),
                "-rz".into(),
                "--name-only".into(),
                "HEAD".into(),
            ],
            false,
        )?
        .1
        .split('\0')
        .map(str::to_string)
        .collect();
        for r in rows {
            let relative = text(&r["パス"]);
            let mut paths = vec![safe_file(&self.folder, relative)?];
            if !artifacts_only {
                paths.push(self.source(relative)?);
            }
            if paths.iter().any(|p| tracked.contains(&rel(&self.root, p))) {
                bail!("受入済みのファイルには操作しない: {relative}");
            }
        }
        Ok(())
    }
    fn produce(
        &self,
        stage: &str,
        group: &[Value],
        round: usize,
        key: &str,
        failures: &Value,
    ) -> Result<Value> {
        let attempt = format!("{}{key}", self.prefix);
        let s = self.state();
        let attempted = array(&s["attempted"]);
        let mut pending = if attempted.iter().any(|v| v == &attempt) {
            group
                .iter()
                .filter(|r| {
                    let path = if stage == "draft" {
                        self.source(text(&r["パス"]))
                    } else {
                        safe_file(&self.folder, text(&r["パス"]))
                    };
                    !path.map(|p| p.is_file()).unwrap_or(false)
                })
                .cloned()
                .collect::<Vec<_>>()
        } else {
            self.update(|s| {
                if !s["attempted"].is_array() {
                    s["attempted"] = json!([]);
                }
                s["attempted"].as_array_mut().unwrap().push(json!(attempt));
            })?;
            group.to_vec()
        };
        if stage == "build"
            && round == 0
            && (self.args.review_again || self.args.draft_review_again)
        {
            pending.retain(|r| !self.folder.join(text(&r["パス"])).is_file());
        }
        let before = self.hashes(&[], true)?;
        let result = (|| -> Result<Value> {
            if stage == "draft" {
                if !pending.is_empty() {
                    self.codex(stage, &pending, round, failures, None, 0, &json!({}))?;
                }
            } else {
                self.update(|s| {
                    if !s["build_runs"].is_object() {
                        s["build_runs"] = json!({});
                    }
                    if s["build_runs"].get(&attempt).is_none() {
                        s["build_runs"][&attempt] = json!({"completed":0,"output":{}});
                    }
                })?;
                let completed = self.state()["build_runs"][&attempt]["completed"]
                    .as_u64()
                    .unwrap_or(0) as usize;
                for execution in completed..3 {
                    if pending.is_empty() {
                        break;
                    }
                    if execution > 0 {
                        self.update(|s| {
                            if !s["build_continuations"].is_object() {
                                s["build_continuations"] = json!({});
                            }
                            for r in &pending {
                                let p = text(&r["パス"]);
                                if !s["build_continuations"][p].is_array() {
                                    s["build_continuations"][p] = json!([]);
                                }
                                let marker = json!(format!("{attempt}/continue{execution}"));
                                let uses = s["build_continuations"][p].as_array_mut().unwrap();
                                if !uses.contains(&marker) {
                                    uses.push(marker);
                                }
                            }
                        })?;
                    }
                    let prior = self.state()["build_runs"][&attempt]["output"].clone();
                    let mut continuation_output = Map::new();
                    if execution > 0 {
                        for r in &pending {
                            let p = text(&r["パス"]);
                            continuation_output
                                .insert(p.into(), prior.get(p).cloned().unwrap_or(json!("")));
                        }
                    }
                    let output = self.codex(
                        stage,
                        &pending,
                        round,
                        failures,
                        None,
                        execution,
                        &json!(continuation_output),
                    )?;
                    pending.retain(|r| !self.folder.join(text(&r["パス"])).is_file());
                    let output: Map<String, Value> = pending
                        .iter()
                        .map(|r| {
                            let p = text(&r["パス"]);
                            (p.to_string(), json!(missing_explanation(&output, p, group)))
                        })
                        .collect();
                    self.update(|s| {
                        s["build_runs"][&attempt] =
                            json!({"completed":execution+1,"output":output});
                        let mut missing = map(&s["build_missing_output"]);
                        missing.extend(output);
                        s["build_missing_output"] = json!(missing);
                    })?;
                }
            }
            Ok(Value::Null)
        })();
        if stage == "build" && before != self.hashes(&[], true)? {
            bail!("成形中に原稿が変更された");
        }
        result
    }
    fn dates(&self, rows: &[Value], round: usize) -> Result<Value> {
        let mut errors = BTreeMap::new();
        for row in rows {
            let relative = text(&row["パス"]);
            let path = safe_file(&self.folder, relative)?;
            if !path.exists() {
                continue;
            }
            let cfg = config(&self.source(relative)?);
            let mut args = vec![
                path.to_string_lossy().to_string(),
                "--author".into(),
                cfg.get("author").cloned().unwrap_or_default(),
                "--date".into(),
                text(&row["日付"]).into(),
                "--timezone".into(),
                self.cfg.get("timezone").cloned().unwrap_or("UTC".into()),
            ];
            for (key, flag) in [
                ("modified_by", "--modified-by"),
                ("created", "--created"),
                ("device", "--device"),
            ] {
                if let Some(v) = cfg.get(key).filter(|s| !s.is_empty()) {
                    args.push(flag.into());
                    args.push(v.trim_matches(['\'', '"']).into());
                }
            }
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            if ext == "pdf" && text(&row["作り方"]).starts_with("office:") {
                let parent = cfg
                    .get("derived_from")
                    .filter(|s| !s.is_empty())
                    .map(String::as_str)
                    .unwrap_or(text(&row["作り方"]));
                let app = match Path::new(parent).extension().and_then(|s| s.to_str()) {
                    Some("xlsx") => "Excel",
                    Some("pptx") => "PowerPoint",
                    _ => "Word",
                };
                args.extend([
                    "--producer".into(),
                    format!("Microsoft® {app} for Microsoft 365"),
                ]);
            }
            let mut criterion = "metadata";
            let result = (|| -> Result<()> {
                if ext == "xlsx" {
                    if has_formula(&path)? {
                        criterion = "formula";
                        bin(&self.root, "recalc", &[path.to_string_lossy().into()], true)?;
                    }
                }
                criterion = "metadata";
                bin(&self.root, "stamp", &args, true)?;
                let stamp = filetime::FileTime::from_unix_time(self.timestamp(row)?.timestamp(), 0);
                filetime::set_file_times(&path, stamp, stamp)?;
                Ok(())
            })();
            if let Err(e) = result {
                errors.insert(relative.to_string(), format!("{criterion}: {e}"));
                self.event("check", round, criterion, relative, &e.to_string())?;
            }
        }
        Ok(json!(errors))
    }
    fn parallel<T: Send>(
        &self,
        tasks: Vec<(String, usize, T)>,
        action: impl Fn(T) -> Result<Value> + Sync,
    ) -> Result<Vec<Value>> {
        thread::scope(|scope| {
            let handles: Vec<_> = tasks
                .into_iter()
                .map(|(key, count, value)| {
                    let action = &action;
                    scope.spawn(move || self.step(&key, count, || action(value)))
                })
                .collect();
            let mut results = vec![];
            let mut failure = None;
            for h in handles {
                match h.join() {
                    Ok(Ok(value)) => results.push(value),
                    Ok(Err(e)) => {
                        if failure.is_none() {
                            failure = Some(e);
                        }
                    }
                    Err(_) => {
                        if failure.is_none() {
                            failure = Some(anyhow!("制作スレッドが異常終了"));
                        }
                    }
                }
            }
            if let Some(e) = failure {
                Err(e)
            } else {
                Ok(results)
            }
        })
    }
    fn phase(&self, stage: &str, rows: &[Value], all: &[Value]) -> Result<Value> {
        let mut failures = json!({});
        let mut history = vec![];
        for round in 0..3 {
            let targets: Vec<_> = rows
                .iter()
                .filter(|r| round == 0 || failures.get(text(&r["パス"])).is_some())
                .cloned()
                .collect();
            if targets.is_empty() {
                break;
            }
            if round > 0 {
                self.step(&format!("{stage}-delete-r{round}"), targets.len(), || {
                    self.protected(&targets, stage == "build")?;
                    for r in &targets {
                        let p = if stage == "draft" {
                            self.source(text(&r["パス"]))?
                        } else {
                            safe_file(&self.folder, text(&r["パス"]))?
                        };
                        remove(&p)?;
                    }
                    Ok(Value::Null)
                })?;
            }
            let groups = if stage == "build" {
                build_groups(&self.root, &targets)?
            } else {
                targets.chunks(10).map(|s| s.to_vec()).collect()
            };
            let mut tasks = vec![];
            for (i, group) in groups.into_iter().enumerate() {
                let key = format!("{stage}-r{round}-g{i}");
                if stage == "build" {
                    self.update(|s| {
                        if !s["build_groups"].is_object() {
                            s["build_groups"] = json!({});
                        }
                        s["build_groups"][format!("{}{key}", self.prefix)] =
                            json!(group.iter().map(|r| text(&r["パス"])).collect::<Vec<_>>());
                    })?;
                }
                tasks.push((key.clone(), group.len(), (key, group)));
            }
            self.parallel(tasks, |(key, group)| {
                self.produce(stage, &group, round, &key, &failures)
            })?;
            let stamp_failures = if stage == "build" {
                self.step(&format!("dates-r{round}"), targets.len(), || {
                    self.dates(&targets, round)
                })?
            } else {
                json!({})
            };
            let automatic = self.step(&format!("{stage}-check-r{round}"), targets.len(), || {
                if stage == "draft" {
                    self.draft_check(rows)
                } else {
                    self.check(round, rows)
                }
            })?;
            if stage == "draft" {
                for (p, v) in map(&automatic) {
                    let reason = text(&v);
                    let criterion = if reason.starts_with("duplicate:") {
                        "duplicate"
                    } else if reason.contains("frontmatter") {
                        "frontmatter"
                    } else if reason == "制作の痕跡" {
                        "trace"
                    } else if reason.starts_with("derived_from") {
                        "derived_from"
                    } else {
                        "source"
                    };
                    self.event("draft-check", round, criterion, &p, reason)?;
                }
            }
            let mut rejected = map(&automatic);
            rejected.extend(map(&stamp_failures));
            let review_rows: Vec<_> = targets
                .iter()
                .filter(|r| !rejected.contains_key(text(&r["パス"])))
                .cloned()
                .collect();
            let review_stage = format!("{stage}-review");
            let tasks = review_rows
                .chunks(10)
                .enumerate()
                .map(|(i, g)| {
                    (
                        format!("{review_stage}-r{round}-g{i}"),
                        g.len(),
                        (i, g.to_vec()),
                    )
                })
                .collect();
            let mut verdicts = Map::new();
            for values in self.parallel(tasks, |(i, g)| {
                self.review(&review_stage, &g, all, round, i)
            })? {
                verdicts.extend(map(&values));
            }
            let report_stage = if stage == "build" {
                "review"
            } else {
                &review_stage
            };
            let report: Vec<Vec<String>> = verdicts
                .iter()
                .map(|(p, v)| {
                    vec![
                        p.clone(),
                        text(&v[0]).into(),
                        text(&v[2]).into(),
                        text(&v[1]).into(),
                    ]
                })
                .collect();
            write_tsv(
                &self.scratch.join(format!(
                    "{}{report_stage}-r{round}.tsv",
                    self.prefix.replace('/', "-")
                )),
                &report,
            )?;
            for (p, v) in verdicts {
                if v[0] != "pass" {
                    rejected.insert(p, v[1].clone());
                }
            }
            failures = json!(rejected);
            history.push(json!({"round":round,"failures":failures}));
            self.update(|s| {
                s[format!("{stage}_failures")] = failures.clone();
                s[format!("{stage}_history")] = json!(history);
                s[format!("{stage}_retries")] = json!(round);
                s[format!("{stage}_pass")] = json!(rows.len().saturating_sub(rejected.len()));
                s[format!("{stage}_reject")] = json!(rejected.len());
                if stage == "build" {
                    s["rebuilds"] = json!(round);
                }
            })?;
            if rejected.is_empty() {
                break;
            }
        }
        Ok(failures)
    }
    fn draft_phase(&self, rows: &[Value], all: &[Value]) -> Result<Value> {
        let mut remaining = rows.to_vec();
        let mut failed = Map::new();
        let mut history = vec![];
        let mut wave = 0;
        while !remaining.is_empty() {
            let known: HashSet<_> = remaining
                .iter()
                .map(|r| text(&r["パス"]).to_string())
                .collect();
            let ready: Vec<_> = remaining
                .iter()
                .filter(|r| !prerequisites(r).iter().any(|p| known.contains(p)))
                .cloned()
                .collect();
            if ready.is_empty() {
                bail!("先に要るものが循環している");
            }
            self.update(|s| {
                if !s["draft_missing_prerequisites"].is_object() {
                    s["draft_missing_prerequisites"] = json!({});
                }
                for r in &ready {
                    let missing: Vec<_> = prerequisites(r)
                        .into_iter()
                        .filter(|p| {
                            failed.contains_key(p)
                                || !self.sources.join(format!("{p}.md")).is_file()
                        })
                        .collect();
                    s["draft_missing_prerequisites"][text(&r["パス"])] = json!(missing);
                }
            })?;
            let mut batch = self.clone();
            if wave > 0 || ready.len() != rows.len() {
                batch.prefix += &format!("draft-wave{wave}/");
            }
            failed.extend(map(&batch.phase("draft", &ready, all)?));
            history.extend(array(&self.state()["draft_history"]));
            let done: HashSet<_> = ready.iter().map(|r| text(&r["パス"]).to_string()).collect();
            remaining.retain(|r| !done.contains(text(&r["パス"])));
            wave += 1;
        }
        self.update(|s| {
            s["draft_failures"] = json!(failed);
            s["draft_history"] = json!(history);
            s["draft_pass"] = json!(rows.len().saturating_sub(failed.len()));
            s["draft_reject"] = json!(failed.len());
        })?;
        Ok(json!(failed))
    }
    fn build_phase(&self, rows: &[Value], all: &[Value]) -> Result<Value> {
        let mut dependencies = self.dependencies(rows)?;
        let mut remaining = rows.to_vec();
        let mut failed = Map::new();
        let mut history = vec![];
        let mut wave = 0;
        while !remaining.is_empty() {
            let known: HashSet<_> = remaining
                .iter()
                .map(|r| text(&r["パス"]).to_string())
                .collect();
            let mut ready: Vec<_> = remaining
                .iter()
                .filter(|r| {
                    !dependencies[text(&r["パス"])]
                        .iter()
                        .any(|p| known.contains(p))
                })
                .cloned()
                .collect();
            if ready.is_empty() {
                ready = remaining.clone();
                for r in &remaining {
                    dependencies
                        .get_mut(text(&r["パス"]))
                        .unwrap()
                        .retain(|p| !known.contains(p));
                }
            }
            let mut targets = vec![];
            let mut blocked = Map::new();
            for r in &ready {
                let p = text(&r["パス"]);
                if let Some(missing) = dependencies[p]
                    .iter()
                    .find(|p| failed.contains_key(p.as_str()) || !self.folder.join(p).is_file())
                {
                    blocked.insert(
                        p.into(),
                        json!(format!("元のファイル {missing} が未完成。")),
                    );
                } else {
                    targets.push(r.clone());
                }
            }
            let mut batch = self.clone();
            if wave > 0 || ready.len() != rows.len() {
                batch.prefix += &format!("build-wave{wave}/");
            }
            if !targets.is_empty() {
                failed.extend(map(&batch.phase("build", &targets, all)?));
                history.extend(array(&self.state()["build_history"]));
                for r in &targets {
                    let p = text(&r["パス"]);
                    if failed.contains_key(p) {
                        remove(&safe_file(&self.folder, p)?)?;
                    }
                }
            }
            failed.extend(blocked);
            let done: HashSet<_> = ready.iter().map(|r| text(&r["パス"]).to_string()).collect();
            remaining.retain(|r| !done.contains(text(&r["パス"])));
            wave += 1;
        }
        self.update(|s| {
            s["build_failures"] = json!(failed);
            s["build_history"] = json!(history);
            s["build_pass"] = json!(rows.len().saturating_sub(failed.len()));
            s["build_reject"] = json!(failed.len());
        })?;
        Ok(json!(failed))
    }
    fn judgment_path(&self) -> PathBuf {
        self.plan
            .with_file_name(format!("{}.judgment.md", self.batch))
    }

    fn request_redraft(&self) -> Result<()> {
        let state = self.state();
        let status = text(&state["status"]);
        if state["redraft_requested_from"].is_string()
            || state["redraft_active"] == true
            || ["accepted", "discarded"].contains(&status)
            || !git(
                &self.root,
                &["ls-tree", "HEAD", "--", &rel(&self.root, &self.plan)],
            )?
            .is_empty()
        {
            return Ok(());
        }
        self.update(|s| {
            s["redraft_requested_from"] = json!(status);
            s["status"] = json!("running");
            s["stage"] = json!("redraft-requested");
            s["reason"] = json!("");
        })
    }

    fn prepare_replan(&self) -> Result<Option<String>> {
        let relative = format!("progress/{}/{}.replan.patch", self.persona, self.batch);
        let path = safe_file(&self.root, &relative)?;
        if !path.try_exists()? {
            if !git(&self.root, &["ls-tree", "HEAD", "--", &relative])?.is_empty() {
                bail!("HEAD にある再計画の差分が作業ツリーにない: {relative}");
            }
            return Ok(None);
        }
        let _persona = persona_lock(&self.root, &self.persona, false, true)?;
        let patch = read(&path)?;
        let digest = format!("{:x}", Sha256::digest(patch.as_bytes()));
        let state = self.state();
        if array(&state["replan_digests"]).iter().any(|v| v == &digest) {
            return Ok(None);
        }
        let tracked = git(&self.root, &["show", &format!("HEAD:{relative}")])
            .map_err(|_| anyhow!("再計画の差分を HEAD で確認できない: {relative}"))?;
        if tracked != patch {
            bail!("再計画の差分が HEAD の内容と一致しない: {relative}");
        }
        let plan_path = rel(&self.root, &self.plan);
        if matches!(text(&state["status"]), "accepted" | "discarded")
            || !git(&self.root, &["ls-tree", "HEAD", "--", &plan_path])?.is_empty()
        {
            bail!("受入または破棄済みの batch には再計画を適用しない");
        }
        if state["redraft_active"] == true {
            bail!("進行中の原稿やり直しには新しい再計画を適用しない");
        }
        let progress = format!("progress/{}", self.persona);
        let allowed = [
            plan_path.clone(),
            rel(&self.root, &self.request),
            rel(&self.root, &self.definition),
            format!("{progress}/facts.md"),
            format!("{progress}/roadmap.md"),
        ];
        let header = re(
            r"^diff --git a/([^ \n]+) b/([^ \n]+)\nindex ([0-9a-f]{40})\.\.([0-9a-f]{40}) 100644\n--- a/([^ \n]+)\n\+\+\+ b/([^ \n]+)\n@@ ",
        );
        let plan = safe_file(&self.root, &plan_path)?;
        let mut before = read(&plan)?;
        let mut after = before.clone();
        fs::create_dir_all(&self.scratch)?;
        let staging = tempfile::tempdir_in(&self.scratch)?;
        let diff_file = staging.path().join("replan.patch");
        let mut seen = HashSet::new();
        let mut replacements = Vec::new();
        let mut parts = patch.split("\ndiff --git ");
        let first = parts.next().unwrap_or_default().to_owned();
        for section in std::iter::once(first).chain(parts.map(|p| format!("diff --git {p}"))) {
            let caps = header.captures(&section).ok_or_else(|| {
                anyhow!("再計画には既存 Markdown の完全な Git blob ID を含む差分が必要")
            })?;
            let relative = &caps[1];
            if !allowed.iter().any(|p| p == relative)
                || [2, 5, 6].iter().any(|i| &caps[*i] != relative)
                || !seen.insert(relative.to_owned())
            {
                bail!("再計画の対象外または重複: {relative}");
            }
            let target = safe_file(&self.root, relative)?;
            let current = read(&target)?;
            let projected = staging.path().join(relative);
            write(&projected, &current)?;
            let blob = |path: &Path| -> Result<String> {
                git(
                    &self.root,
                    &["hash-object", "--no-filters", "--", path.to_str().unwrap()],
                )
                .map(|s| s.trim().to_owned())
            };
            let hash = blob(&projected)?;
            let reverse = if hash == caps[3] {
                false
            } else if hash == caps[4] {
                true
            } else {
                bail!("再計画の差分と現在の内容が一致しない: {relative}");
            };
            write(&diff_file, format!("{}\n", section.trim_end_matches('\n')))?;
            let mut apply = Command::new("git");
            apply
                .current_dir(staging.path())
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .env_remove("GIT_COMMON_DIR")
                .env("GIT_CEILING_DIRECTORIES", &self.scratch)
                .args([
                    "apply",
                    "--whitespace=nowarn",
                    &format!("--include={relative}"),
                ]);
            if reverse {
                apply.arg("--reverse");
            }
            let out = apply.arg(&diff_file).output()?;
            if !out.status.success() || blob(&projected)? != caps[if reverse { 3 } else { 4 }] {
                bail!(
                    "再計画の差分を適用できない: {relative} {}",
                    String::from_utf8_lossy(&out.stderr)
                );
            }
            let changed = read(&projected)?;
            if relative == plan_path {
                if reverse {
                    before = changed.clone();
                } else {
                    after = changed.clone();
                }
            }
            if !reverse {
                let mut temp = tempfile::NamedTempFile::new_in(&self.scratch)?;
                temp.as_file()
                    .set_permissions(fs::metadata(&target)?.permissions())?;
                temp.write_all(changed.as_bytes())?;
                replacements.push((target, current, temp));
            }
        }
        let before = parse_plan(&before)?;
        let after = parse_plan(&after)?;
        if before.len() != after.len()
            || before.iter().zip(&after).any(|(old, new)| {
                old["パス"] != new["パス"]
                    || (text(&old["内容"]).starts_with("破棄：")
                        || text(&new["内容"]).starts_with("破棄："))
                        && COLUMNS.iter().any(|c| old[*c] != new[*c])
            })
        {
            bail!("再計画は計画のパス、順番、破棄済みの行を変更できない");
        }
        let pending: Vec<_> = after
            .into_iter()
            .filter(|row| !text(&row["内容"]).starts_with("破棄："))
            .collect();
        if pending.is_empty() {
            bail!("再計画で作り直す行がない");
        }
        self.protected(&pending, false)?;
        // A retry recognizes any files already replaced before an interruption.
        for (target, before, _) in &replacements {
            if read(target)? != *before {
                bail!("再計画の適用中に内容が変更された: {}", target.display());
            }
        }
        for (target, _, temp) in replacements {
            temp.persist(target)?;
        }
        Ok(Some(digest))
    }

    fn redraft_candidates(&self) -> Result<(Vec<Value>, Vec<String>)> {
        let s = self.state();
        let mut events = array(&s["events"]);
        let ledger = self.root.join("progress/findings.tsv");
        if ledger.exists() {
            for r in tsv_dict(&read(&ledger)?)? {
                if r.get("persona") == Some(&self.persona) && r.get("batch") == Some(&self.batch) {
                    events.push(json!(
                        [
                            "date",
                            "persona",
                            "batch",
                            "stage",
                            "round",
                            "criterion",
                            "path",
                            "reason"
                        ]
                        .map(|k| r.get(k).cloned().unwrap_or_default())
                    ));
                }
            }
        }
        let mut selected = vec![];
        let mut unknown = vec![];
        for row in plan_rows(&self.plan)? {
            let p = text(&row["パス"]);
            if s["redraft_active"] == true && array(&s["redraft_paths"]).iter().any(|v| v == p) {
                selected.push(row);
                continue;
            }
            let body = text(&row["内容"]);
            if !body.starts_with("破棄：") {
                continue;
            }
            let reason = body
                .trim_start_matches("破棄：")
                .split('。')
                .next()
                .unwrap_or("");
            let related: Vec<_> = events.iter().filter(|e| e[6] == p).collect();
            if reason == "統括の抜き取りで不合格" || related.iter().any(|e| e[3] == "coordinator")
            {
                continue;
            }
            if s["draft_failures"].get(p).is_some()
                || reason == "先に要る原稿が不合格"
                || related.iter().any(|e| {
                    ["draft-check", "draft-review"].contains(&text(&e[3]))
                        && text(&e[7]).trim_end_matches('。').replace('。', "；") == reason
                })
            {
                selected.push(row);
            } else {
                unknown.push(p.to_string());
            }
        }
        Ok((selected, unknown))
    }
    fn redraft(&self, replan: Option<String>) -> Result<()> {
        let (pending, unknown) = if replan.is_some() {
            (
                plan_rows(&self.plan)?
                    .into_iter()
                    .filter(|row| !text(&row["内容"]).starts_with("破棄："))
                    .collect(),
                vec![],
            )
        } else {
            self.redraft_candidates()?
        };
        for p in unknown {
            println!("原稿段階の破棄か判断できないため対象外: {p}");
        }
        if pending.is_empty() {
            if let Some(previous) = self.state()["redraft_requested_from"].as_str() {
                self.update(|s| {
                    s["status"] = json!(previous);
                    s["stage"] = json!(previous);
                    s["redraft_requested_from"] = Value::Null;
                })?;
            }
            return Ok(());
        }
        self.protected(&pending, false)?;
        let paths: HashSet<String> = pending.iter().map(|r| text(&r["パス"]).into()).collect();
        let generation = self.update(|s| {
            let active = s["redraft_active"] == true;
            if !active {
                s["redraft_replanned"] = json!(replan.is_some());
                if let Some(digest) = replan {
                    let mut consumed = array(&s["replan_digests"]);
                    consumed.push(json!(digest));
                    s["replan_digests"] = json!(consumed);
                }
                s["redraft_previous_draft_failures"] = json!(
                    map(&s["draft_failures"])
                        .into_iter()
                        .filter(|(p, _)| !paths.contains(p))
                        .collect::<Map<_, _>>()
                );
                s["redraft_previous_failures"] = json!(
                    map(&s["failures"])
                        .into_iter()
                        .filter(|(p, _)| !paths.contains(p))
                        .collect::<Map<_, _>>()
                );
            }
            let mut sorted: Vec<_> = paths.iter().cloned().collect();
            sorted.sort();
            s["redraft_paths"] = json!(sorted);
            let mut rebuild: BTreeSet<String> = array(&s["rebuild_paths"])
                .iter()
                .map(|v| text(v).into())
                .collect();
            rebuild.extend(paths.clone());
            s["rebuild_paths"] = json!(rebuild);
            let generation = s["redraft_generation"].as_u64().unwrap_or(0) + u64::from(!active);
            s["redraft_generation"] = json!(generation);
            s["redraft_active"] = json!(true);
            generation
        })?;
        let mut batch = self.clone();
        batch.prefix = format!("redraft{generation}/");
        let replanned = self.state()["redraft_replanned"] == true;
        if replanned {
            batch.step("replan-backup", pending.len(), || {
                for row in &pending {
                    let relative = text(&row["パス"]);
                    for original in [safe_file(&self.folder, relative)?, self.source(relative)?] {
                        if original.is_file() {
                            let backup = self
                                .scratch
                                .join(format!("redraft{generation}/previous"))
                                .join(rel(&self.root, &original));
                            fs::create_dir_all(backup.parent().unwrap())?;
                            fs::rename(original, backup)?;
                        }
                    }
                }
                Ok(Value::Null)
            })?;
            restore_plan(&self.plan, &paths, "成形待ち：")?;
        }
        restore_plan(&self.plan, &paths, "破棄：")?;
        self.update(|s| {
            s["draft_failures"] = json!(
                map(&s["draft_failures"])
                    .into_iter()
                    .filter(|(p, _)| !paths.contains(p))
                    .collect::<Map<_, _>>()
            );
            s["failures"] = json!(
                map(&s["failures"])
                    .into_iter()
                    .filter(|(p, _)| !paths.contains(p))
                    .collect::<Map<_, _>>()
            );
            s["status"] = json!("running");
            s["stage"] = json!("redraft");
        })?;
        let all = plan_rows(&self.plan)?;
        let draft_failures = batch.step("draft-complete", pending.len(), || {
            batch.draft_phase(&pending, &all)
        })?;
        let mut draft = map(&self.state()["redraft_previous_draft_failures"]);
        draft.extend(map(&draft_failures));
        self.set("draft_failures", json!(draft))?;
        self.discard_rows(&value_map(&draft_failures))?;
        let targets: Vec<_> = pending
            .iter()
            .filter(|r| draft_failures.get(text(&r["パス"])).is_none())
            .cloned()
            .collect();
        let build_failures = batch.step("build-complete", targets.len(), || {
            batch.build_phase(&targets, &all)
        })?;
        let mut failures = map(&self.state()["redraft_previous_failures"]);
        failures.extend(map(&draft_failures));
        failures.extend(map(&build_failures));
        self.set("failures", json!(failures))?;
        self.set(
            "count",
            json!(
                plan_rows(&self.plan)?
                    .iter()
                    .filter(|r| !text(&r["内容"]).starts_with("破棄：")
                        && !failures.contains_key(text(&r["パス"])))
                    .count()
            ),
        )?;
        self.write_report(&plan_rows(&self.plan)?, &mut failures, Some(&paths))?;
        self.final_check()?;
        self.update(|s| {
            s["status"] = json!("done");
            s["stage"] = json!("done");
            s["reason"] = json!("");
            s["redraft_active"] = json!(false);
            s["redraft_requested_from"] = Value::Null;
        })?;
        Ok(())
    }
    fn rebuild_pending(&self) -> Result<()> {
        let all = plan_rows(&self.plan)?;
        let pending: Vec<_> = all
            .iter()
            .filter(|r| text(&r["内容"]).starts_with("成形待ち："))
            .cloned()
            .collect();
        if pending.is_empty() {
            return Ok(());
        }
        self.protected(&pending, true)?;
        let paths: HashSet<String> = pending
            .iter()
            .map(|r| text(&r["パス"]).to_string())
            .collect();
        self.set(
            "rebuild_paths",
            json!(pending.iter().map(|r| text(&r["パス"])).collect::<Vec<_>>()),
        )?;
        for p in &paths {
            if !self.source(p)?.is_file() {
                bail!("成形待ちの原稿がない: {p}");
            }
        }
        let generation = self.update(|s| {
            let n = (s["rebuild_generation"].as_u64().unwrap_or(0)
                + u64::from(["done", "accepted"].contains(&text(&s["status"]))))
            .max(1);
            s["rebuild_generation"] = json!(n);
            s["status"] = json!("running");
            s["stage"] = json!("rebuild");
            n
        })?;
        let mut batch = self.clone();
        batch.prefix = format!("rebuild{generation}/");
        restore_plan(&self.plan, &paths, "成形待ち：")?;
        let result = (|| -> Result<()> {
            let result = batch.build_phase(&pending, &all)?;
            let mut failures: Map<String, Value> = map(&self.state()["failures"])
                .into_iter()
                .filter(|(p, _)| !paths.contains(p))
                .collect();
            failures.extend(map(&result));
            self.set("failures", json!(failures))?;
            self.set(
                "count",
                json!(
                    all.iter()
                        .filter(|r| !text(&r["内容"]).starts_with("破棄：")
                            && !failures.contains_key(text(&r["パス"])))
                        .count()
                ),
            )?;
            self.write_report(&all, &mut failures, None)?;
            self.final_check()
        })();
        if result.is_err() {
            mark_plan(
                &self.plan,
                &paths
                    .iter()
                    .map(|p| (p.clone(), "再成形が未完了".into()))
                    .collect(),
                "成形待ち",
            )?;
        }
        result?;
        self.update(|s| {
            s["status"] = json!("done");
            s["stage"] = json!("done");
        })
    }
    fn run(&mut self) -> Result<()> {
        self.reload_state()?;
        if self.args.redraft {
            self.request_redraft()?;
        }
        let state = self.state();
        self.args.redraft |=
            state["redraft_requested_from"].is_string() || state["redraft_active"] == true;
        let result = self.run_attempt();
        if let Err(error) = &result
            && self.state()["redraft_requested_from"].is_string()
        {
            self.update(|s| {
                s["status"] = json!("failed");
                s["reason"] = json!(error.to_string());
            })?;
        }
        result
    }
    fn run_attempt(&mut self) -> Result<()> {
        let replan = if self.args.redraft {
            self.prepare_replan()?
        } else {
            None
        };
        if replan.is_some() {
            self.cfg = config(&self.definition);
        }
        if !self.request.is_file() {
            bail!("依頼なし: {}", self.request.display());
        }
        let again = self.args.review_again || self.args.draft_review_again;
        let state = self.state();
        if ["done", "accepted"].contains(&text(&state["status"]))
            && !again
            && !self.args.rebuild
            && !self.args.redraft
        {
            println!("done 終了 {} 件", state["count"].as_u64().unwrap_or(0));
            return Ok(());
        }
        let mut coordinator_producer = None;
        if self.args.rebuild || self.args.redraft {
            let judgment = self.judgment_path();
            for path in
                std::iter::once(&self.request).chain(judgment.is_file().then_some(&judgment))
            {
                if read(path)?.lines().any(|line| line.trim() == SOL_REBUILD) {
                    coordinator_producer = Some("gpt-6.2-sol".to_owned());
                    break;
                }
            }
        }
        self.update(|s| {
            s["active"] = json!({});
            s["producer"] = json!(
                self.args
                    .producer
                    .clone()
                    .or(coordinator_producer)
                    .unwrap_or_else(|| {
                        s["producer"]
                            .as_str()
                            .unwrap_or("gpt-6.1-terra")
                            .to_string()
                    })
            );
        })?;
        fs::create_dir_all(self.scratch.join("logs"))?;
        let result = self.run_inner(again, replan);
        if let Err(e) = &result {
            self.update(|s| {
                s["status"] = json!("failed");
                s["reason"] = json!(e.to_string());
            })?;
            self.record_tables(None, None, false)?;
        }
        result
    }
    fn run_inner(&self, again: bool, replan: Option<String>) -> Result<()> {
        if self.args.redraft {
            return self.redraft(replan);
        }
        if again {
            self.update(|s| {
                let keep = |key: &str| {
                    if ["templates", "templates-reviewed", "plan"].contains(&key) {
                        return true;
                    }
                    let leaf = key.rsplit('/').next().unwrap_or(key);
                    if self.args.draft_review_again {
                        return re(r"^draft-r0-g\d+$").is_match(leaf);
                    }
                    key.starts_with("draft-")
                        || leaf.starts_with("build-r0-")
                        || leaf.starts_with("dates-r0")
                };
                s["completed"] = json!(
                    map(&s["completed"])
                        .into_iter()
                        .filter(|(k, _)| keep(k))
                        .collect::<Map<_, _>>()
                );
                s["attempted"] = json!(
                    array(&s["attempted"])
                        .into_iter()
                        .filter(|v| keep(text(v)))
                        .collect::<Vec<_>>()
                );
                if self.args.draft_review_again {
                    s["draft-review_notes"] = json!([]);
                }
                s["build-review_notes"] = json!([]);
            })?;
        }
        if self.args.rebuild {
            return self.rebuild_pending();
        }
        let planned = {
            let _lock = persona_lock(&self.root, &self.persona, false, true)?;
            let mut planning = self.clone();
            planning.planning = true;
            planning.step("templates", 0, || {
                if self.templates.exists() {
                    Ok(json!(""))
                } else {
                    Ok(json!(planning.codex(
                        "templates",
                        &[],
                        0,
                        &json!({}),
                        None,
                        0,
                        &json!({})
                    )?))
                }
            })?;
            planning.step("templates-reviewed", 0, || planning.review_templates(None))?;
            let before = hash_paths(files(self.templates.parent().unwrap()), false)?;
            let rows = planning.step("plan", 0, || planning.make_plan())?;
            if before != hash_paths(files(self.templates.parent().unwrap()), false)? {
                planning.step("plan-templates-reviewed", 0, || {
                    planning.review_templates(None)
                })?;
            }
            array(&rows)
        };
        let current: HashMap<String, Value> = plan_rows(&self.plan)?
            .into_iter()
            .map(|r| (text(&r["パス"]).into(), r))
            .collect();
        let expected: HashSet<_> = planned
            .iter()
            .map(|r| text(&r["パス"]).to_string())
            .collect();
        if expected != current.keys().cloned().collect() {
            bail!("検査済み計画のパスが変更された");
        }
        for row in &planned {
            let p = text(&row["パス"]);
            let new = &current[p];
            if COLUMNS
                .iter()
                .filter(|c| **c != "内容")
                .any(|c| new[*c] != row[*c])
                || (new["内容"] != row["内容"] && !marked(text(&new["内容"])))
            {
                bail!("検査済み計画が変更された: {p}");
            }
        }
        let rows: Vec<_> = planned
            .iter()
            .filter_map(|r| {
                let new = &current[text(&r["パス"])];
                (!again || !marked(text(&new["内容"]))).then(|| new.clone())
            })
            .collect();
        if self.args.plan_only {
            self.update(|s| {
                s["status"] = json!("planned");
                s["stage"] = json!("plan");
                s["count"] = json!(rows.len());
            })?;
            return Ok(());
        }
        let draft = self.step("draft-complete", rows.len(), || {
            self.draft_phase(&rows, &rows)
        })?;
        let draft: Map<String, Value> = map(&draft)
            .into_iter()
            .filter(|(p, _)| rows.iter().any(|r| text(&r["パス"]) == p))
            .collect();
        let build_rows: Vec<_> = rows
            .iter()
            .filter(|r| !draft.contains_key(text(&r["パス"])))
            .cloned()
            .collect();
        self.step("draft-discard", draft.len(), || {
            self.discard_rows(&value_map(&json!(draft)))?;
            Ok(Value::Null)
        })?;
        let build = self.step("build-complete", build_rows.len(), || {
            self.build_phase(&build_rows, &rows)
        })?;
        let mut failures: Map<String, Value> = plan_rows(&self.plan)?
            .iter()
            .filter_map(|r| {
                text(&r["内容"]).strip_prefix("成形待ち：").map(|v| {
                    (
                        text(&r["パス"]).to_string(),
                        json!(v.split('。').next().unwrap_or("")),
                    )
                })
            })
            .collect();
        failures.extend(draft);
        failures.extend(map(&build));
        self.update(|s| {
            s["failures"] = json!(failures);
            s["count"] = json!(
                current
                    .values()
                    .filter(|r| !text(&r["内容"]).starts_with("破棄：")
                        && !failures.contains_key(text(&r["パス"])))
                    .count()
            );
        })?;
        self.step("report", rows.len().saturating_sub(failures.len()), || {
            self.write_report(&rows, &mut failures, None)?;
            Ok(Value::Null)
        })?;
        self.step(
            "final-check",
            rows.len().saturating_sub(failures.len()),
            || {
                self.final_check()?;
                Ok(Value::Null)
            },
        )?;
        self.write_report(&rows, &mut failures, None)?;
        self.update(|s| {
            s["status"] = json!("done");
            s["stage"] = json!("done");
            s["reason"] = json!("");
        })?;
        Ok(())
    }
    fn discard_rows(&self, failures: &BTreeMap<String, String>) -> Result<()> {
        let rows: Vec<_> = failures.keys().map(|p| json!({"パス":p})).collect();
        self.protected(&rows, false)?;
        for p in failures.keys() {
            remove(&safe_file(&self.folder, p)?)?;
            remove(&self.source(p)?)?;
        }
        mark_plan(&self.plan, failures, "破棄")
    }
    fn final_check(&self) -> Result<()> {
        let (code, out) = bin(
            &self.root,
            "check",
            &[self.persona.clone(), "--batch".into(), self.batch.clone()],
            false,
        )?;
        write(&self.scratch.join("logs/final-check.log"), &out)?;
        if code != 0 {
            bail!("最終検査不合格: {}", tail_chars(&out, 2000));
        }
        Ok(())
    }
    fn write_report(
        &self,
        rows: &[Value],
        failures: &mut Map<String, Value>,
        mutation: Option<&HashSet<String>>,
    ) -> Result<()> {
        let s = self.state();
        let draft = map(&s["draft_failures"]);
        let discarded: BTreeMap<_, _> = failures
            .iter()
            .filter(|(p, _)| {
                draft.contains_key(*p) && mutation.map(|m| m.contains(*p)).unwrap_or(true)
            })
            .map(|(p, v)| (p.clone(), text(v).to_string()))
            .collect();
        self.discard_rows(&discarded)?;
        let mut waiting: BTreeMap<String, String> = failures
            .iter()
            .filter(|(p, _)| !draft.contains_key(*p))
            .map(|(p, v)| (p.clone(), text(v).to_string()))
            .collect();
        for (p, reason) in waiting.iter_mut() {
            if reason.contains("plan: planned file missing") {
                let output = text(&s["build_missing_output"][p.as_str()])
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                let output = if output.is_empty() {
                    "成形の最終出力に作れなかった理由の記載なし"
                } else {
                    &output
                };
                *reason = format!(
                    "{}。成形の最終出力：{output}",
                    reason
                        .split("。成形の最終出力：")
                        .next()
                        .unwrap_or("")
                        .trim()
                );
                failures.insert(p.clone(), json!(reason));
                self.event("check", 2, "plan", p, reason)?;
            }
        }
        let changed: BTreeMap<_, _> = waiting
            .iter()
            .filter(|(p, _)| mutation.map(|m| m.contains(*p)).unwrap_or(true))
            .map(|(p, r)| (p.clone(), r.clone()))
            .collect();
        self.protected(
            &changed
                .keys()
                .map(|p| json!({"パス":p}))
                .collect::<Vec<_>>(),
            true,
        )?;
        for p in changed.keys() {
            remove(&safe_file(&self.folder, p)?)?;
        }
        mark_plan(&self.plan, &changed, "成形待ち")?;
        self.update(|s| {
            s["pending"] = json!(waiting.len());
            s["failures"] = json!(failures);
        })?;
        self.record_tables(None, None, false)?;
        let mut discarded: BTreeMap<String, String> = plan_rows(&self.plan)?
            .iter()
            .filter_map(|r| {
                text(&r["内容"]).strip_prefix("破棄：").map(|v| {
                    (
                        text(&r["パス"]).to_string(),
                        v.split('。').next().unwrap_or("").to_string(),
                    )
                })
            })
            .collect();
        for (p, v) in failures.iter().filter(|(p, _)| draft.contains_key(*p)) {
            discarded.insert(p.clone(), text(v).into());
        }
        let mut counts = BTreeMap::<String, usize>::new();
        for row in rows {
            if !failures.contains_key(text(&row["パス"]))
                && !text(&row["内容"]).starts_with("破棄：")
            {
                let ext = Path::new(text(&row["パス"]))
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy();
                *counts.entry(format!(".{ext}")).or_default() += 1;
            }
        }
        let mut out = format!(
            "# {} {} の結果\n\n状態：結果待ち\n\n## 依頼との対応\n\n{}\n\n制作：{} 件\n\n{}",
            self.persona,
            self.batch,
            read(&self.request)?,
            counts.values().sum::<usize>(),
            counts
                .iter()
                .map(|(e, n)| format!("- {e}: {n} 件"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let section = |title: &str, items: Vec<String>| {
            format!(
                "\n\n## {title}\n\n{}",
                if items.is_empty() {
                    "なし".into()
                } else {
                    items.join("\n")
                }
            )
        };
        out += &section(
            "成形待ち",
            waiting.iter().map(|(p, r)| format!("- {p}: {r}")).collect(),
        );
        out += &section(
            "続きを使った行",
            map(&s["build_continuations"])
                .iter()
                .map(|(p, r)| {
                    format!(
                        "- {p}: {}",
                        array(r).iter().map(text).collect::<Vec<_>>().join(", ")
                    )
                })
                .collect(),
        );
        out += &section(
            "repo の直下に作られたファイル",
            array(&s["strays"])
                .iter()
                .map(|v| format!("- {}", stray_message(v)))
                .collect(),
        );
        out += &section(
            "破棄した行と理由",
            discarded
                .iter()
                .map(|(p, r)| format!("- {p}: {r}"))
                .collect(),
        );
        out += &section(
            "先に要る原稿が欠けたまま書いた行",
            map(&s["draft_missing_prerequisites"])
                .iter()
                .filter(|(_, v)| !array(v).is_empty())
                .map(|(p, v)| {
                    format!(
                        "- {p}: {}",
                        array(v).iter().map(text).collect::<Vec<_>>().join(", ")
                    )
                })
                .collect(),
        );
        for (stage, label) in [("draft", "原稿"), ("build", "成形")] {
            out += &format!(
                "\n\n## {label}の検査とレビュー\n\n合格：{} 件、不合格：{} 件、作り直し：{} 回\n\n{label}の所要時間：{:.2} 秒\n",
                s[format!("{stage}_pass")].as_u64().unwrap_or(0),
                s[format!("{stage}_reject")].as_u64().unwrap_or(0),
                s[format!("{stage}_retries")].as_u64().unwrap_or(0),
                s["timings"][format!("{stage}-complete")]
                    .as_f64()
                    .unwrap_or(0.)
            );
            for entry in array(&s[format!("{stage}_history")]) {
                out += &format!(
                    "\nround {}: 不合格 {} 件\n",
                    entry["round"],
                    map(&entry["failures"]).len()
                );
                out += &map(&entry["failures"])
                    .iter()
                    .map(|(p, r)| format!("- {p}: {}", text(r)))
                    .collect::<Vec<_>>()
                    .join("\n");
                out.push('\n');
            }
            let notes = array(&s[format!("{stage}-review_notes")]);
            out += &format!(
                "\nレビュー役の所見：\n\n{}",
                if notes.is_empty() {
                    "対象なし".into()
                } else {
                    notes.iter().map(text).collect::<Vec<_>>().join("\n\n")
                }
            );
        }
        out += &format!(
            "\n\nレンダリング：{}/work/*/render/\n\n## 各段階の所要時間\n\n",
            rel(&self.root, &self.scratch)
        );
        out += &map(&s["timings"])
            .iter()
            .map(|(k, v)| format!("- {k}: {:.2} 秒", v.as_f64().unwrap_or(0.)))
            .collect::<Vec<_>>()
            .join("\n");
        write(&self.result, out + "\n")
    }
    fn protect_derived_sources(&self, drops: &BTreeMap<String, String>) -> Result<()> {
        if drops.is_empty() || !self.sources.try_exists()? {
            return Ok(());
        }
        let sources_to_drop: BTreeMap<_, _> = drops
            .keys()
            .map(|p| Ok((self.source(p)?, p)))
            .collect::<Result<_>>()?;
        let mut references = Vec::new();
        for entry in WalkDir::new(&self.sources).sort_by_file_name() {
            let entry = entry?;
            let path = entry.path();
            if !entry.file_type().is_file() || sources_to_drop.contains_key(path) {
                continue;
            }
            let relative = rel(&self.sources, path);
            let Some(child) = relative.strip_suffix(".md") else {
                continue;
            };
            let cfg = config_text(&read(path)?);
            if let Some(parent) = cfg.get("derived_from") {
                let parent = parent.trim_matches(['"', '\'']);
                if let Some(dropped) =
                    sources_to_drop.get(&self.sources.join(format!("{parent}.md")))
                {
                    references.push(format!("{child} → {dropped}"));
                }
            }
        }
        if !references.is_empty() {
            bail!(
                "破棄対象を derived_from で参照する原稿が残る: {}",
                references.join(", ")
            );
        }
        Ok(())
    }
    fn finish(&self, discard: bool) -> Result<()> {
        let _batch = batch_lock(&self.root, &self.persona, &self.batch, false)?;
        self.reload_state()?;
        let removes_files = discard || !self.args.drop.is_empty();
        let _persona = persona_lock(&self.root, &self.persona, !removes_files, true)?;
        let _repository = repository_lock(&self.root, "records")?;
        let rows = plan_rows(&self.plan)?;
        let state = self.state();
        if !discard
            && (state["redraft_requested_from"].is_string() || state["redraft_active"] == true)
        {
            bail!("原稿のやり直しが未完了の batch は受入できない");
        }
        let mut rebuilding: HashSet<String> = array(&state["rebuild_paths"])
            .iter()
            .map(|v| text(v).into())
            .collect();
        if !rebuilding.is_empty() {
            let head: HashSet<_> = cmd(
                &self.root,
                &[
                    "git".into(),
                    "ls-tree".into(),
                    "-rz".into(),
                    "--name-only".into(),
                    "HEAD".into(),
                ],
                false,
            )?
            .1
            .split('\0')
            .map(str::to_string)
            .collect();
            let changed = git(
                &self.root,
                &[
                    "status",
                    "--porcelain",
                    "-z",
                    "--",
                    &format!("people/{}", self.persona),
                ],
            )?;
            let changed: HashSet<_> = changed.split('\0').filter_map(|p| p.get(3..)).collect();
            rebuilding.retain(|p| {
                let path = rel(&self.root, &self.folder.join(p));
                !head.contains(&path) || changed.contains(path.as_str())
            });
            // A batch never accepted is accepted whole: earlier rebuilds and the
            // first build are all still uncommitted and may be dropped.
            let accepted_before = rows
                .iter()
                .any(|r| head.contains(&rel(&self.root, &self.folder.join(text(&r["パス"])))));
            if !accepted_before {
                rebuilding.clear();
            } else if rebuilding.is_empty() {
                bail!("受入するものがない（やり直した行はすべて受入済み）");
            }
        }
        let scoped: Vec<_> = rows
            .iter()
            .filter(|r| rebuilding.is_empty() || rebuilding.contains(text(&r["パス"])))
            .cloned()
            .collect();
        self.protected(&scoped, !rebuilding.is_empty())?;
        if !rebuilding.is_empty() && discard {
            bail!("再成形済み batch の一括破棄は受入済みのファイルに影響する");
        }
        let mut paths: BTreeSet<String> = files(self.plan.parent().unwrap())
            .iter()
            .filter(|p| {
                p.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .starts_with(&format!("{}.", self.batch))
            })
            .map(|p| rel(&self.root, p))
            .collect();
        paths.extend([
            "progress/findings.tsv".into(),
            "progress/batches.tsv".into(),
        ]);
        let active: Vec<_> = rows
            .iter()
            .filter(|r| !text(&r["内容"]).starts_with("破棄："))
            .collect();
        if !discard {
            if state["status"] != "done" {
                bail!("制作とレビューが完了していない");
            }
            for r in &active {
                let p = text(&r["パス"]);
                if !text(&r["内容"]).starts_with("成形待ち：") {
                    paths.insert(rel(&self.root, &safe_file(&self.folder, p)?));
                }
                paths.insert(rel(&self.root, &self.source(p)?));
            }
            paths.extend([
                format!("manifests/{}.tsv", self.persona),
                format!("personas/{}.md", self.persona),
                format!("progress/{}/facts.md", self.persona),
            ]);
            paths.extend(
                files(self.templates.parent().unwrap())
                    .iter()
                    .map(|p| rel(&self.root, p)),
            );
            paths.extend(
                git(
                    &self.root,
                    &[
                        "ls-files",
                        "-z",
                        "--",
                        &format!("templates/{}", self.persona),
                    ],
                )?
                .split('\0')
                .filter(|s| !s.is_empty())
                .map(str::to_string),
            );
        }
        let roadmap = format!("progress/{}/roadmap.md", self.persona);
        if self.root.join(&roadmap).exists() {
            paths.insert(roadmap);
        }
        let staged: HashSet<_> = git(&self.root, &["diff", "--cached", "--name-only", "-z"])?
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        let outside: Vec<_> = staged
            .iter()
            .filter(|p| !paths.contains(*p))
            .cloned()
            .collect();
        if !outside.is_empty() {
            bail!(
                "関係ないファイルが stage されている: {}",
                outside.join(", ")
            );
        }
        let drops: BTreeMap<String, String> = if discard {
            rows.iter()
                .map(|r| (text(&r["パス"]).into(), self.args.reason.clone()))
                .collect()
        } else {
            self.args
                .drop
                .iter()
                .map(|p| (p.clone(), "統括の抜き取りで不合格".into()))
                .collect()
        };
        let allowed: HashSet<_> = if rebuilding.is_empty() {
            rows.iter().map(|r| text(&r["パス"]).to_string()).collect()
        } else {
            rebuilding
        };
        if drops.keys().any(|p| !allowed.contains(p)) {
            bail!("--drop はこの batch の相対パスを指定する");
        }
        self.protect_derived_sources(&drops)?;
        for p in drops.keys() {
            remove(&safe_file(&self.folder, p)?)?;
            remove(&self.source(p)?)?;
        }
        mark_plan(&self.plan, &drops, "破棄")?;
        for (p, r) in &drops {
            self.event("coordinator", 0, "other", p, r)?;
        }
        if !discard {
            bin(
                &self.root,
                "check",
                &[self.persona.clone(), "--batch".into(), self.batch.clone()],
                true,
            )?;
            let mut args = vec![self.persona.clone()];
            for r in plan_rows(&self.plan)?
                .iter()
                .filter(|r| !marked(text(&r["内容"])))
            {
                args.extend(["--include".into(), text(&r["パス"]).into()]);
            }
            bin(&self.root, "manifest", &args, true)?;
        }
        self.record_tables(
            Some(if discard { "discarded" } else { "accepted" }),
            Some(drops.len()),
            false,
        )?;
        let marker = if discard { "破棄" } else { "受入済み" };
        let mut report = if self.result.exists() {
            let mut out = re(r"(?m)^状態：.*$")
                .replace_all(&read(&self.result)?, format!("状態：{marker}"))
                .to_string();
            if !out.contains("状態：") {
                out += &format!("\n状態：{marker}\n");
            }
            if !drops.is_empty() {
                out += "\n統括の判断：\n";
                out += &drops
                    .iter()
                    .map(|(p, r)| format!("- {p}: {r}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                out.push('\n');
            }
            out
        } else {
            paths.insert(rel(&self.root, &self.result));
            format!(
                "# {} {}\n\n状態：{marker}\n\n理由：{}\n",
                self.persona,
                self.batch,
                if discard { &self.args.reason } else { "" }
            )
        };
        let judgment = self.judgment_path();
        if judgment.is_file() {
            let section = read(&judgment)?;
            if !discard && section.lines().any(|line| line.trim() == SOL_REBUILD) {
                let request = read(&self.request)?;
                if !request.lines().any(|line| line.trim() == SOL_REBUILD) {
                    write(
                        &self.request,
                        format!("{}\n\n{SOL_REBUILD}\n", request.trim_end()),
                    )?;
                    paths.insert(rel(&self.root, &self.request));
                }
            }
            if !report.contains(section.trim_end()) {
                report.push('\n');
                report.push_str(section.trim_end());
                report.push('\n');
            }
        }
        write(&self.result, report)?;
        remove(&judgment)?;
        let tracked: HashSet<_> = git(&self.root, &["ls-files", "-z"])?
            .split('\0')
            .map(str::to_string)
            .collect();
        paths.retain(|p| self.root.join(p).exists() || tracked.contains(p));
        let mut args = vec!["git".into(), "add".into(), "--".into()];
        args.extend(paths.iter().cloned());
        cmd(&self.root, &args, true)?;
        let staged = git(&self.root, &["diff", "--cached", "--name-only", "-z"])?;
        if staged
            .split('\0')
            .filter(|s| !s.is_empty())
            .any(|p| !paths.contains(p))
        {
            bail!("stage の範囲が変わった");
        }
        let count = active
            .iter()
            .filter(|r| self.folder.join(text(&r["パス"])).is_file())
            .count();
        let message = if discard {
            format!(
                "Discard {} {}: {}",
                self.persona, self.batch, self.args.reason
            )
        } else {
            format!(
                "Accept {} {}: {} ({count} files)",
                self.persona,
                self.batch,
                self.scope()?
            )
        };
        git(&self.root, &["commit", "-m", &message])?;
        self.update(|s| {
            s["status"] = json!(if discard { "discarded" } else { "accepted" });
            s["stage"] = json!("finished");
        })?;
        if discard
            || !plan_rows(&self.plan)?
                .iter()
                .any(|r| text(&r["内容"]).starts_with("成形待ち："))
        {
            let _ = fs::remove_dir_all(&self.scratch);
        }
        println!("{marker}: {} {} {count} 件", self.persona, self.batch);
        Ok(())
    }
    fn launch(&self) -> Result<()> {
        let _lock = repository_lock(&self.root, "admission")?;
        if !self.request.is_file() {
            bail!("依頼なし: {}", self.request.display());
        }
        if fs2::available_space(&self.root)? < 30_000_000_000 {
            bail!("空き容量が 30,000,000,000 バイト未満");
        }
        let tool = self.root.join("ops/bin/codex-job");
        let output = cmd(
            &self.root,
            &[
                tool.to_string_lossy().into(),
                "status".into(),
                "--repo".into(),
                "personaCorpus".into(),
            ],
            true,
        )?
        .1;
        admit(&self.root, &self.persona, &self.batch, &output, false)?;
        let home = std::env::var_os("AGENT_OPS_HOME")
            .map(PathBuf::from)
            .unwrap_or(self.root.join("ops"));
        let mut names: HashSet<String> = fs::read_dir(home.join("jobs/personaCorpus"))
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().into())
            .collect();
        for line in output.lines().skip(1) {
            let p: Vec<_> = line.split('\t').collect();
            if p.len() >= 3 {
                names.insert(p[1].into());
            }
        }
        let original = format!("{}-{}", self.persona.split('-').next().unwrap(), self.batch);
        let mut name = original.clone();
        let mut suffix = 1;
        while names.contains(&name) {
            suffix += 1;
            name = format!("{original}-r{suffix}");
        }
        let mut args = vec![
            "bin/batch".into(),
            "run".into(),
            self.persona.clone(),
            self.batch.clone(),
            "--wait-lock".into(),
        ];
        for (enabled, flag) in [
            (self.args.plan_only, "--plan-only"),
            (self.args.review_again, "--review-again"),
            (self.args.draft_review_again, "--draft-review-again"),
            (self.args.rebuild, "--rebuild"),
            (self.args.redraft, "--redraft"),
        ] {
            if enabled {
                args.push(flag.into());
            }
        }
        if let Some(p) = &self.args.producer {
            args.extend(["--producer".into(), p.clone()]);
        }
        let command = args
            .iter()
            .map(|s| shell_words::quote(s).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        if self.args.redraft {
            let _batch = batch_lock(&self.root, &self.persona, &self.batch, false)?;
            self.reload_state()?;
            self.request_redraft()?;
        }
        let out = cmd(
            &self.root,
            &[
                tool.to_string_lossy().into(),
                "start".into(),
                "--repo".into(),
                "personaCorpus".into(),
                "--name".into(),
                name,
                "--command".into(),
                command,
            ],
            true,
        )?
        .1;
        print!("{out}");
        Ok(())
    }
}
fn text_value(v: &Value) -> &str {
    text(v)
}
fn marked(s: &str) -> bool {
    s.starts_with("破棄：") || s.starts_with("成形待ち：")
}
fn tail_chars(s: &str, n: usize) -> String {
    s.chars()
        .rev()
        .take(n)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}
fn hash_paths(paths: Vec<PathBuf>, include_missing: bool) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for p in paths {
        if p.is_file() {
            out.insert(
                p.to_string_lossy().to_string(),
                format!("{:x}", Sha256::digest(fs::read(&p)?)),
            );
        } else if include_missing {
            out.insert(p.to_string_lossy().to_string(), String::new());
        }
    }
    Ok(out)
}
fn load_table(p: &Path, header: &[String]) -> Result<Vec<Vec<String>>> {
    if !p.exists() {
        return Ok(vec![]);
    }
    let mut rows = tsv(&read(p)?)?;
    if !rows.is_empty() {
        if rows[0] != header {
            bail!(
                "記録の列が不正: {}",
                p.file_name().unwrap_or_default().to_string_lossy()
            );
        }
        rows.remove(0);
    }
    Ok(rows)
}
fn atomic_tsv(p: &Path, rows: &[Vec<String>]) -> Result<()> {
    let temp = p.with_extension("tmp");
    write_tsv(&temp, rows)?;
    fs::rename(temp, p)?;
    Ok(())
}
fn has_formula(path: &Path) -> Result<bool> {
    let mut zip = zip::ZipArchive::new(File::open(path)?)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        if entry.name().starts_with("xl/worksheets/") && entry.name().ends_with(".xml") {
            let mut xml = String::new();
            entry.read_to_string(&mut xml)?;
            let doc = roxmltree::Document::parse(&xml)?;
            if doc
                .descendants()
                .any(|n| n.is_element() && n.tag_name().name() == "f")
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn pending_count(root: &Path, p: &str, b: &str) -> Result<usize> {
    let plan = root.join(format!("progress/{p}/{b}.md"));
    if !plan.exists() {
        return Ok(0);
    }
    Ok(plan_rows(&plan)?
        .iter()
        .filter(|r| text(&r["内容"]).starts_with("成形待ち："))
        .count())
}
fn state_label(root: &Path, p: &str, b: &str, s: &Value) -> String {
    let status = text(&s["status"]);
    if status == "done" {
        return "結果待ち".into();
    }
    if status == "running" && !runtime_batches(root, p).contains(b) {
        return format!("中断 ({}, 状態記録のみ)", text(&s["stage"]));
    }
    let label = match status {
        "accepted" => "受入済み",
        "discarded" => "破棄",
        "failed" => "失敗",
        "planned" => "計画済み",
        "new" => "未開始",
        "running" => "進行中",
        _ => status,
    };
    format!("{label} ({}, {status})", text(&s["stage"]))
}
fn show_status(root: &Path, persona: Option<&str>, batch: Option<&str>) -> Result<()> {
    println!(
        "Codex 使用中 {} / 上限 {}",
        slot_usage(root)?,
        limit(root, "max_codex", "CORPUS_MAX_CODEX", 32)?
    );
    if let Some(b) = batch {
        if !re(r"^b\d{3,}$").is_match(b) {
            bail!("batch 名は b001 からの連番");
        }
        let p = persona.ok_or_else(|| anyhow!("persona なし"))?;
        let path = root.join(format!("scratch/{p}/{b}/state.json"));
        if path.exists() {
            let s: Value = serde_json::from_str(&read(&path)?)?;
            println!("{}", state_label(root, p, b, &s));
            println!(
                "成形の組 {} / 続きを使った行 {} 件",
                map(&s["build_groups"]).len(),
                map(&s["build_continuations"]).len()
            );
            for (g, v) in map(&s["active"]) {
                if re(r"-g\d+$").is_match(&g) {
                    println!("  組 {g}: {v}");
                }
            }
            println!(
                "{p} {b}: {} / {} / {} 件 / 原稿 合格{} 不合格{} / 成形 合格{} 不合格{} / 成形待ち{} / {}",
                text(&s["stage"]),
                text(&s["status"]),
                s["count"].as_u64().unwrap_or(0),
                s["draft_pass"].as_u64().unwrap_or(0),
                s["draft_reject"].as_u64().unwrap_or(0),
                s["build_pass"].as_u64().unwrap_or(0),
                s["build_reject"].as_u64().unwrap_or(0),
                pending_count(root, p, b)?,
                text(&s["reason"])
            );
        } else {
            let result = root.join(format!("progress/{p}/{b}.result.md"));
            println!(
                "{}",
                if result.exists() {
                    read(&result)?
                } else {
                    "依頼あり（状態なし）".into()
                }
            );
            println!("成形待ち {} 件", pending_count(root, p, b)?);
        }
        return Ok(());
    }
    let names = if let Some(p) = persona {
        vec![p.to_string()]
    } else {
        let mut names = vec![];
        for e in fs::read_dir(root.join("personas"))?.flatten() {
            let name = e
                .path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if re(r"^p\d\d-.*$").is_match(&name) {
                names.push(name);
            }
        }
        names.sort();
        names
    };
    println!("persona\t受入済み件数");
    for p in names {
        let manifest = root.join(format!("manifests/{p}.tsv"));
        let entries = if manifest.exists() {
            tsv(&read(&manifest)?)?
        } else {
            vec![]
        };
        println!("{p}\t{}", entries.len());
        if persona.is_none() {
            continue;
        }
        let mut counts = BTreeMap::<String, usize>::new();
        for row in &entries {
            if let Some(name) = row.first() {
                let ext = Path::new(name)
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                *counts.entry(ext).or_default() += 1;
            }
        }
        let definition = read(&root.join(format!("personas/{p}.md")))?;
        let mut in_section = false;
        let mut targets = BTreeMap::<Vec<String>, Option<f64>>::new();
        for line in definition.lines() {
            if line.starts_with("##") {
                in_section = line.contains("形式の目安");
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some(c) = re(r"(.+?)(?:[：:]|\|)\s*(\d+(?:\.\d+)?)\s*%").captures(line) {
                let mut ext: Vec<String> = EXT
                    .split_whitespace()
                    .filter(|e| {
                        matches(&format!(r"(?<![a-z]){}(?![a-z])", regex::escape(e)), &c[1])
                    })
                    .map(str::to_string)
                    .collect();
                ext.sort();
                ext.dedup();
                if !ext.is_empty() {
                    let value = c[2].parse::<f64>()?;
                    let old = targets.get(&ext).copied().flatten().unwrap_or(0.);
                    targets.insert(ext, Some(old + value));
                }
            }
        }
        let covered: HashSet<_> = targets.keys().flatten().cloned().collect();
        for e in counts.keys() {
            if !covered.contains(e) {
                targets.insert(vec![e.clone()], None);
            }
        }
        for (ext, target) in targets {
            let count: usize = ext
                .iter()
                .map(|e| counts.get(e).copied().unwrap_or(0))
                .sum();
            let ratio = if entries.is_empty() {
                0.
            } else {
                100. * count as f64 / entries.len() as f64
            };
            let difference = target
                .map(|t| format!("{:+.1} pt", ratio - t))
                .unwrap_or("目安なし".into());
            println!(
                "  {}: {count} 件 / {ratio:.1}% / 差 {difference}",
                ext.join("・")
            );
        }
        for req in files(&root.join(format!("progress/{p}")))
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .ends_with(".request.md")
            })
        {
            let file = req.file_name().unwrap().to_string_lossy();
            let b = file.trim_end_matches(".request.md");
            let state = root.join(format!("scratch/{p}/{b}/state.json"));
            let result = req.with_file_name(format!("{b}.result.md"));
            let mut label = "依頼あり".to_string();
            if result.exists() {
                let out = read(&result)?;
                label = re(r"(?m)^状態：(.*)")
                    .captures(&out)
                    .map(|c| c[1].to_string())
                    .unwrap_or("結果待ち".into());
            }
            let s = if state.exists() {
                serde_json::from_str::<Value>(&read(&state)?)?
            } else {
                json!({})
            };
            if state.exists() {
                label = state_label(root, &p, b, &s);
            }
            let groups = if state.exists() && runtime_batches(root, &p).contains(b) {
                map(&s["active"])
                    .keys()
                    .filter(|k| re(r"-g\d+$").is_match(k))
                    .count()
            } else {
                0
            };
            println!(
                "  {b}: {label} / 同時実行 {groups} 組 / 成形待ち {} 件",
                pending_count(root, &p, b)?
            );
            for (g, v) in map(&s["active"]) {
                if re(r"-g\d+$").is_match(&g) {
                    println!("    組 {g}: {v}");
                }
            }
        }
    }
    Ok(())
}
fn show_findings(root: &Path, since: Option<&str>, persona: Option<&str>) -> Result<()> {
    if let Some(s) = since {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
    }
    let load = |name: &str| -> Result<Vec<HashMap<String, String>>> {
        let p = root.join(format!("progress/{name}"));
        if p.exists() {
            tsv_dict(&read(&p)?)
        } else {
            Ok(vec![])
        }
    };
    let events: Vec<_> = load("findings.tsv")?
        .into_iter()
        .filter(|r| {
            since
                .map(|d| r.get("date").map(String::as_str).unwrap_or("") >= d)
                .unwrap_or(true)
                && persona
                    .map(|p| r.get("persona").map(String::as_str) == Some(p))
                    .unwrap_or(true)
        })
        .collect();
    let get = |r: &HashMap<String, String>, k: &str| r.get(k).cloned().unwrap_or_default();
    let mut groups = BTreeMap::<(String, String), Vec<_>>::new();
    for event in &events {
        groups
            .entry((get(event, "stage"), get(event, "criterion")))
            .or_default()
            .push(event);
    }
    let mut ordered: Vec<_> = groups.iter().collect();
    ordered.sort_by_key(|(key, v)| (std::cmp::Reverse(v.len()), *key));
    println!("段階 × 観点（多い順）\t件数\t代表の理由（3件まで）");
    for ((stage, criterion), items) in ordered {
        let mut reasons = vec![];
        for r in items {
            let reason = get(r, "reason");
            if !reasons.contains(&reason) && reasons.len() < 3 {
                reasons.push(reason);
            }
        }
        println!(
            "{stage}\t{criterion}\t{}\t{}",
            items.len(),
            reasons.join(" / ")
        );
    }
    println!("\nbatch ごとの初回の不合格率\npersona\tbatch\t日付\t原稿\t成形\t状態");
    let mut batches = load("batches.tsv")?;
    batches.sort_by_key(|r| (get(r, "date"), get(r, "persona"), get(r, "batch")));
    for r in batches {
        if since.map(|d| get(&r, "date").as_str() < d).unwrap_or(false)
            || persona.map(|p| get(&r, "persona") != p).unwrap_or(false)
        {
            continue;
        }
        let n = get(&r, "planned").parse::<f64>().unwrap_or(0.);
        let rate = |k: &str| {
            if n == 0. {
                0.
            } else {
                100. * get(&r, k).parse::<f64>().unwrap_or(0.) / n
            }
        };
        println!(
            "{}\t{}\t{}\t{:.1}%\t{:.1}%\t{}",
            get(&r, "persona"),
            get(&r, "batch"),
            get(&r, "date"),
            rate("draft_initial_rejects"),
            rate("build_initial_rejects"),
            get(&r, "status")
        );
    }
    let improvements = root.join("progress/improvements.md");
    if improvements.exists() {
        println!("\n改善の前後（記録日より前 / 記録日以降）");
        let mut date = String::new();
        for line in read(&improvements)?.lines() {
            if let Some(c) = re(r"\b(\d{4}-\d\d-\d\d)\b").captures(line) {
                date = c[1].into();
            }
            if date.is_empty() {
                continue;
            }
            let mut criteria: BTreeSet<_> = re(r"\b(?:D(?:10|[1-9])|V[1-6]|other)\b")
                .find_iter(line)
                .map(|m| m.as_str().to_string())
                .collect();
            for (_, c) in groups.keys() {
                if line.split_whitespace().any(|w| w == c) || line.contains(&format!("`{c}`")) {
                    criteria.insert(c.clone());
                }
            }
            for criterion in criteria {
                let before = events
                    .iter()
                    .filter(|r| get(r, "criterion") == criterion && get(r, "date") < date)
                    .count();
                let after = events
                    .iter()
                    .filter(|r| get(r, "criterion") == criterion && get(r, "date") >= date)
                    .count();
                println!("{date}\t{criterion}\t{before}\t{after}");
            }
        }
    }
    Ok(())
}
pub fn run(args: &[String]) -> i32 {
    if args.iter().any(|s| s == "--help" || s == "-h") {
        println!(
            "usage: corpus batch <start|resume|run|accept|discard|status|templates|findings> [arguments]"
        );
        return 0;
    }
    if let Err(e) = validate_args(args) {
        eprintln!("batch: {e}");
        return 2;
    }
    match main_inner(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("batch: {}", e.to_string().lines().next().unwrap_or("error"));
            1
        }
    }
}
fn main_inner(args: &[String]) -> Result<i32> {
    let root = crate::base();
    let action = args.first().map(String::as_str).unwrap_or("");
    if ["", "-h", "--help"].contains(&action) {
        println!(
            "usage: corpus batch <start|resume|run|accept|discard|status|templates|findings> [arguments]"
        );
        return Ok(if action.is_empty() { 2 } else { 0 });
    }
    if action == "codex-once" {
        let option = |flag: &str| {
            args.iter()
                .position(|s| s == flag)
                .and_then(|i| args.get(i + 1))
                .map(PathBuf::from)
                .ok_or_else(|| anyhow!("{flag} required"))
        };
        let brief = option("--brief")?;
        let output = option("--output")?;
        let brief = if brief.is_absolute() {
            brief
        } else {
            std::env::current_dir()?.join(brief)
        };
        let output = if output.is_absolute() {
            output
        } else {
            std::env::current_dir()?.join(output)
        };
        let brief = resolve_path(&brief)?;
        let output = resolve_path(&output)?;
        if !brief.starts_with(root.join("scratch")) || !output.starts_with(root.join("scratch")) {
            bail!("補助ジョブの入出力は scratch 内に限る");
        }
        fs::create_dir_all(output.parent().unwrap())?;
        let _slot = slot(&root)?;
        let watcher = Watch::begin(&root, output.parent().unwrap(), "auxiliary", 0)?;
        let mut command = Command::new(std::env::var("CODEX_BIN").unwrap_or("codex".into()));
        command
            .args([
                "exec",
                "-m",
                "gpt-6.2-astra",
                "-c",
                "model_reasoning_effort=\"medium\"",
                "-s",
                "workspace-write",
                "-c",
                "sandbox_workspace_write.network_access=true",
                "-C",
            ])
            .arg(&root)
            .arg("-o")
            .arg(&output)
            .arg("-")
            .current_dir(&root)
            .stdin(Stdio::piped());
        inherit_slot(&mut command, _slot.as_raw_fd());
        let mut child = command.spawn()?;
        child.stdin.take().unwrap().write_all(&fs::read(brief)?)?;
        let code = child.wait()?.code().unwrap_or(1);
        watcher.finish()?;
        return Ok(code);
    }
    let mut a = Args::default();
    let mut positional = vec![];
    let mut since = None;
    let mut selected_persona = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--plan-only" => a.plan_only = true,
            "--review-again" => a.review_again = true,
            "--draft-review-again" => a.draft_review_again = true,
            "--rebuild" => a.rebuild = true,
            "--redraft" => a.redraft = true,
            "--wait-lock" => a.wait_lock = true,
            "--producer" | "--reason" | "--fix" | "--since" | "--persona" => {
                let key = args[i].clone();
                i += 1;
                let v = args
                    .get(i)
                    .ok_or_else(|| anyhow!("{key} required"))?
                    .clone();
                match key.as_str() {
                    "--producer" => a.producer = Some(v),
                    "--reason" => a.reason = v,
                    "--fix" => a.fix = v,
                    "--persona" => selected_persona = Some(v),
                    _ => since = Some(v),
                }
            }
            "--drop" => {
                i += 1;
                while i < args.len() && !args[i].starts_with("--") {
                    a.drop.push(args[i].clone());
                    i += 1;
                }
                continue;
            }
            s if s.starts_with('-') => bail!("不明なオプション: {s}"),
            s => positional.push(s.to_string()),
        }
        i += 1;
    }
    a.persona = selected_persona.unwrap_or_else(|| positional.first().cloned().unwrap_or_default());
    a.batch = positional.get(1).cloned().unwrap_or_default();
    if action == "status" {
        let persona = if a.persona.is_empty() {
            None
        } else {
            Some(persona_name(&root, &a.persona)?)
        };
        show_status(
            &root,
            persona.as_deref(),
            (!a.batch.is_empty()).then_some(a.batch.as_str()),
        )?;
        return Ok(0);
    }
    if action == "findings" {
        let persona = if a.persona.is_empty() {
            None
        } else {
            Some(persona_name(&root, &a.persona)?)
        };
        show_findings(&root, since.as_deref(), persona.as_deref())?;
        return Ok(0);
    }
    if action == "templates" {
        if a.fix.is_empty() {
            bail!("--fix required");
        }
        a.batch = "b000".into();
        let mut b = Batch::new(&root, a)?;
        let _lock = persona_lock(&root, &b.persona, false, false)?;
        b.batch = format!(
            "templates-{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        b.scratch = root.join(format!("scratch/{}/{}", b.persona, b.batch));
        b.state_path = b.scratch.join("state.json");
        b.state = Arc::new(Mutex::new(
            json!({"completed":{},"timings":{},"status":"new","stage":"templates-review"}),
        ));
        fs::create_dir_all(&b.scratch)?;
        b.request = b.scratch.join("templates.request.md");
        write(&b.request, format!("# 型の修正\n{}\n", b.args.fix))?;
        b.planning = true;
        let result = b.review_templates(Some(&b.args.fix));
        b.record_tables(None, None, true)?;
        result?;
        return Ok(0);
    }
    if !["start", "resume", "run", "accept", "discard"].contains(&action) {
        bail!("不明な操作: {action}");
    }
    if action == "discard" && a.reason.is_empty() {
        bail!("--reason required");
    }
    let mut b = Batch::new(&root, a)?;
    match action {
        "start" | "resume" => b.launch()?,
        "run" => {
            let lock = {
                let _admission = repository_lock(&root, "admission")?;
                let tool = root.join("ops/bin/codex-job");
                let jobs = if tool.exists() {
                    cmd(
                        &root,
                        &[
                            tool.to_string_lossy().into(),
                            "status".into(),
                            "--repo".into(),
                            "personaCorpus".into(),
                        ],
                        true,
                    )?
                    .1
                } else {
                    String::new()
                };
                admit(&root, &b.persona, &b.batch, &jobs, b.args.wait_lock)?;
                batch_lock(&root, &b.persona, &b.batch, b.args.wait_lock)?
            };
            let _lock = lock;
            b.run()?;
        }
        _ => b.finish(action == "discard")?,
    }
    Ok(0)
}

fn validate_args(args: &[String]) -> Result<()> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or_else(|| anyhow!("操作が必要です"))?;
    let (min, max) = match action {
        "start" | "resume" | "run" | "accept" | "discard" => (2, 2),
        "status" => (0, 2),
        "templates" => (1, 1),
        "findings" | "codex-once" => (0, 0),
        _ => bail!("不明な操作: {action}"),
    };
    let mut n = 0;
    let mut seen = HashSet::new();
    let mut i = 1;
    while i < args.len() {
        let flag = args[i].as_str();
        if !flag.starts_with('-') {
            n += 1;
            i += 1;
            continue;
        }
        let allowed = match action {
            "start" | "resume" | "run" => {
                [
                    "--plan-only",
                    "--producer",
                    "--review-again",
                    "--draft-review-again",
                    "--rebuild",
                    "--redraft",
                ]
                .contains(&flag)
                    || (action == "run" && flag == "--wait-lock")
            }
            "accept" => flag == "--drop",
            "discard" => flag == "--reason",
            "templates" => flag == "--fix",
            "findings" => ["--since", "--persona"].contains(&flag),
            "codex-once" => ["--brief", "--output"].contains(&flag),
            _ => false,
        };
        if !allowed {
            bail!("不明なオプション: {flag}");
        }
        seen.insert(flag.to_string());
        if flag == "--drop" {
            i += 1;
            let start = i;
            while i < args.len() && !args[i].starts_with('-') {
                i += 1;
            }
            if i == start {
                bail!("--drop は相対パスが必要です");
            }
            continue;
        }
        if [
            "--producer",
            "--reason",
            "--fix",
            "--since",
            "--persona",
            "--brief",
            "--output",
        ]
        .contains(&flag)
        {
            i += 1;
            if i >= args.len() || args[i].starts_with('-') {
                bail!("{flag} に値が必要です");
            }
        }
        i += 1;
    }
    if n < min || n > max {
        bail!("{action} の引数の数が不正です");
    }
    for flag in match action {
        "discard" => vec!["--reason"],
        "templates" => vec!["--fix"],
        "codex-once" => vec!["--brief", "--output"],
        _ => vec![],
    } {
        if !seen.contains(flag) {
            bail!("{flag} が必要です");
        }
    }
    Ok(())
}

/// Clear close-on-exec in the child only; parallel children must not retain each other's slots.
fn inherit_slot(command: &mut Command, fd: i32) {
    unsafe {
        command.pre_exec(move || {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}
fn resolve_path(p: &Path) -> Result<PathBuf> {
    if p.exists() || p.is_symlink() {
        return Ok(p.canonicalize()?);
    }
    let parent = p.parent().ok_or_else(|| anyhow!("path has no parent"))?;
    Ok(resolve_path(parent)?.join(p.file_name().ok_or_else(|| anyhow!("path has no name"))?))
}
