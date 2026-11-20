//! File-name rules and the one-off renaming of accepted deliverables.
//!
//! `apply` moves files and rewrites plain-text references from
//! `progress/<persona>/rename.tsv`; files whose content still names an old
//! file are listed in `rename-refs.tsv` for a Codex job to rebuild. `verify`
//! restores the recorded modification times, rewrites the manifest and runs
//! the corpus checks. `commit` stages exactly the touched paths. `run` drives
//! the Codex jobs (names, content fixes, audit) and the steps above.
use crate::manifest::{Locations, accepted, digest, entries, read_rows, resolve, safe_relative};
use anyhow::{Context, Result, bail};
use clap::Parser;
use fs2::FileExt;
use regex::Regex;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, SystemTime},
};

/// Longest stem a person plausibly types; saved mail keeps its subject.
pub const STEM_MAX: usize = 40;
pub const EML_STEM_MAX: usize = 60;
const TEXT: [&str; 8] = ["txt", "md", "csv", "tsv", "html", "json", "xml", "log"];
const MONTHS: &str = "Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec";
const FULL_MONTHS: &str =
    "January|February|March|April|May|June|July|August|September|October|November|December";

fn re(s: &str) -> Regex {
    Regex::new(s).unwrap()
}
/// Regex for the persona's `date_prefix` (alternatives separated by `|`).
pub fn date_prefix_regex(spec: &str) -> Result<Regex> {
    let mut alts = Vec::new();
    for alt in spec.split('|').map(str::trim).filter(|s| !s.is_empty()) {
        let token = re(r"YYYY|Month|Mon|MM|DD|M|D");
        let mut out = String::from("^");
        let mut last = 0;
        for m in token.find_iter(alt) {
            out.push_str(&regex::escape(&alt[last..m.start()]));
            out.push_str(match m.as_str() {
                "YYYY" => r"\d{4}",
                "MM" => r"(0[1-9]|1[0-2])",
                "DD" => r"(0[1-9]|[12]\d|3[01])",
                "M" => r"([1-9]|1[0-2])",
                "D" => r"([1-9]|[12]\d|3[01])",
                "Mon" => "(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)",
                _ => "(January|February|March|April|May|June|July|August|September|October|November|December)",
            });
            last = m.end();
        }
        out.push_str(&regex::escape(&alt[last..]));
        alts.push(format!("(?:{out})"));
    }
    if alts.is_empty() {
        bail!("date_prefix が空")
    }
    Ok(Regex::new(&alts.join("|"))?)
}
/// Problems with a file name that a person would not have given it.
pub fn name_problems(rel: &str, date_prefix: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let path = Path::new(rel);
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let ext = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let max = if ext == "eml" { EML_STEM_MAX } else { STEM_MAX };
    if stem.chars().count() > max {
        out.push(format!("名前が長すぎる（{max} 文字まで）: {rel}"));
    }
    let dated = re(&format!(
        r"^(\d{{4}}[-_.]?\d{{2}}|\d{{1,2}}月|\d{{1,2}}日|\d{{4}}_|\d{{1,2}}[ ._-]?({MONTHS})[a-z]*\b|({MONTHS})[a-z]*\.? \d{{1,2}}\b|({FULL_MONTHS})\b)"
    ));
    if let Some(spec) = date_prefix
        && dated.is_match(&name)
    {
        match date_prefix_regex(spec) {
            Ok(r) if r.is_match(&name) => {}
            Ok(_) => out.push(format!("日付の形が定義（{spec}）と違う: {rel}")),
            Err(e) => out.push(format!("{e}")),
        }
    }
    let day = re(r"^\d{4}-\d{2}(-\d{2})?$");
    for dir in path.parent().into_iter().flat_map(|p| p.components()) {
        let dir = dir.as_os_str().to_string_lossy();
        if day.is_match(&dir) && dir.len() == 10 && name.contains(dir.as_ref()) {
            out.push(format!("日付のフォルダと同じ日付を名前にも付けている: {rel}"));
        }
    }
    out
}

struct Ctx {
    loc: Locations,
    base: PathBuf,
    persona: String,
}
impl Ctx {
    fn new(loc: Locations, persona: &str) -> Result<Self> {
        let persona = resolve(&loc.root, persona)?;
        let base = loc
            .root
            .parent()
            .context("people folder has no parent")?
            .to_path_buf();
        Ok(Self { loc, base, persona })
    }
    fn home(&self) -> PathBuf {
        self.loc.root.join(&self.persona)
    }
    fn progress(&self) -> PathBuf {
        self.loc.progress.join(&self.persona)
    }
    fn sources(&self) -> PathBuf {
        self.loc.sources.join(&self.persona)
    }
    fn map_path(&self) -> PathBuf {
        self.progress().join("rename.tsv")
    }
    fn refs_path(&self) -> PathBuf {
        self.progress().join("rename-refs.tsv")
    }
    fn manifest(&self) -> PathBuf {
        self.loc.manifests.join(format!("{}.tsv", self.persona))
    }
    fn state_dir(&self) -> PathBuf {
        self.base.join("scratch/rename").join(&self.persona)
    }
    fn date_prefix(&self) -> Result<Option<String>> {
        let text = fs::read_to_string(self.loc.personas.join(format!("{}.md", self.persona)))?;
        let head = text
            .strip_prefix("---\n")
            .and_then(|t| t.split_once("\n---"))
            .map(|x| x.0)
            .unwrap_or("");
        Ok(head
            .lines()
            .find_map(|l| l.strip_prefix("date_prefix:"))
            .map(|v| v.trim().to_string()))
    }
    /// (old, new) pairs from rename.tsv.
    fn pairs(&self) -> Result<Vec<(String, String)>> {
        let text = fs::read_to_string(self.map_path())
            .with_context(|| format!("{} がない", self.map_path().display()))?;
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        if lines.next().map(|h| h.split('\t').take(2).collect::<Vec<_>>())
            != Some(vec!["旧", "新"])
        {
            bail!("rename.tsv の見出しは「旧<TAB>新<TAB>理由」")
        }
        lines
            .map(|l| {
                let c = l.split('\t').collect::<Vec<_>>();
                if c.len() < 3 || c[2].trim().is_empty() {
                    bail!("rename.tsv の行は 3 列（旧、新、理由）: {l}")
                }
                Ok((c[0].to_string(), c[1].to_string()))
            })
            .collect()
    }
    /// Accepted paths as recorded in the manifest (old names until committed).
    fn recorded(&self) -> Result<BTreeMap<String, [String; 4]>> {
        Ok(read_rows(&self.manifest())?
            .into_iter()
            .map(|r| (r[0].clone(), r))
            .collect())
    }
    fn validate(&self, pairs: &[(String, String)]) -> Result<Vec<String>> {
        let accepted = accepted(&self.home())?;
        let prefix = self.date_prefix()?;
        let mut errors = Vec::new();
        if prefix.is_none() {
            errors.push("persona の定義の先頭設定に date_prefix がない".into());
        }
        let (mut olds, mut news) = (BTreeSet::new(), BTreeSet::new());
        for (old, new) in pairs {
            if !accepted.contains(old) {
                errors.push(format!("受入済みにない: {old}"));
            }
            if !olds.insert(old) {
                errors.push(format!("旧が重複: {old}"));
            }
            if !safe_relative(new) || new.contains('\\') {
                errors.push(format!("新のパスが不正: {new}"));
                continue;
            }
            if !news.insert(new) || accepted.contains(new) || self.home().join(new).exists() {
                errors.push(format!("新が重複または既存: {new}"));
            }
            let ext = |s: &str| {
                Path::new(s)
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
            };
            if ext(old) != ext(new) {
                errors.push(format!("拡張子が変わる: {old} → {new}"));
            }
            if old == new {
                errors.push(format!("名前が変わらない: {old}"));
            }
            errors.extend(name_problems(new, prefix.as_deref()));
            if crate::batch::bad_path(new) {
                errors.push(format!("制作区分または ID: {new}"));
            }
        }
        // Every accepted name that breaks the rules must be renamed.
        for rel in accepted.difference(&olds.into_iter().cloned().collect()) {
            for e in name_problems(rel, prefix.as_deref()) {
                errors.push(format!("付け直しが要る: {e}"));
            }
        }
        Ok(errors)
    }
}
/// Replacement keys: old relative paths, then basenames that are unique.
fn replacements(pairs: &[(String, String)], accepted: &BTreeSet<String>) -> Vec<(String, String)> {
    let base = |s: &str| {
        Path::new(s)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    };
    let mut count = BTreeMap::<String, usize>::new();
    for a in accepted {
        *count.entry(base(a)).or_default() += 1;
    }
    let mut out = pairs.to_vec();
    for (old, new) in pairs {
        if count.get(&base(old)) == Some(&1) {
            out.push((base(old), base(new)));
        }
    }
    out.sort_by_key(|(k, _)| std::cmp::Reverse(k.len()));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}
fn replace_all(text: &str, keys: &[(String, String)], pattern: &Regex) -> String {
    let map = keys.iter().cloned().collect::<BTreeMap<_, _>>();
    pattern
        .replace_all(text, |c: &regex::Captures| map[&c[0]].clone())
        .into_owned()
}
fn pattern(keys: &[(String, String)]) -> Regex {
    Regex::new(
        &keys
            .iter()
            .map(|(k, _)| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|"),
    )
    .unwrap()
}
fn md_files(dir: &Path) -> Vec<PathBuf> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .collect()
}
fn rewrite_keep_time(p: &Path, text: &str) -> Result<()> {
    let time = fs::metadata(p)?.modified()?;
    fs::write(p, text)?;
    filetime::set_file_mtime(p, filetime::FileTime::from_system_time(time))?;
    Ok(())
}
fn ext_of(p: &str) -> String {
    Path::new(p)
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
}
/// Record of what apply touched, for verify and commit.
fn save_record(c: &Ctx, v: &Value) -> Result<()> {
    fs::create_dir_all(c.state_dir())?;
    fs::write(c.state_dir().join("applied.json"), serde_json::to_string_pretty(v)?)?;
    Ok(())
}
fn load_record(c: &Ctx) -> Result<Value> {
    Ok(serde_json::from_str(
        &fs::read_to_string(c.state_dir().join("applied.json")).context("apply の記録がない")?,
    )?)
}
fn apply(c: &Ctx) -> Result<Vec<String>> {
    let pairs = c.pairs()?;
    let errors = c.validate(&pairs)?;
    if !errors.is_empty() {
        bail!("rename.tsv が不正:\n{}", errors.join("\n"))
    }
    let accepted = accepted(&c.home())?;
    let mut changed = BTreeSet::<String>::new();
    let repo = |p: &Path| -> String {
        p.strip_prefix(&c.base)
            .unwrap_or(p)
            .to_string_lossy()
            .into_owned()
    };
    for (old, new) in &pairs {
        for (from, to) in [
            (c.home().join(old), c.home().join(new)),
            (
                c.sources().join(format!("{old}.md")),
                c.sources().join(format!("{new}.md")),
            ),
        ] {
            if !from.exists() {
                continue;
            }
            fs::create_dir_all(to.parent().unwrap())?;
            fs::rename(&from, &to)?;
            changed.insert(repo(&from));
            changed.insert(repo(&to));
            let mut dir = from.parent();
            while let Some(d) = dir {
                if d == c.home() || d == c.sources() || fs::remove_dir(d).is_err() {
                    break;
                }
                dir = d.parent();
            }
        }
    }
    let keys = replacements(&pairs, &accepted);
    let pat = pattern(&keys);
    let skip = ["rename.tsv", "rename-refs.tsv", "rename.audit.md"];
    let mut texts = md_files(&c.sources());
    texts.extend(
        md_files(&c.progress())
            .into_iter()
            .filter(|p| !skip.iter().any(|s| p.ends_with(s))),
    );
    let finals = accepted
        .iter()
        .map(|a| {
            pairs
                .iter()
                .find(|(o, _)| o == a)
                .map(|(_, n)| n.clone())
                .unwrap_or(a.clone())
        })
        .collect::<BTreeSet<_>>();
    for rel in &finals {
        if TEXT.contains(&ext_of(rel).as_str()) {
            texts.push(c.home().join(rel));
        }
    }
    for p in texts {
        let Ok(text) = fs::read_to_string(&p) else {
            continue;
        };
        let out = replace_all(&text, &keys, &pat);
        if out != text {
            rewrite_keep_time(&p, &out)?;
            changed.insert(repo(&p));
        }
    }
    // Deliverables whose source names a renamed file must say the new name.
    let new_names = keys
        .iter()
        .map(|(_, n)| n.clone())
        .collect::<BTreeSet<_>>();
    let mut refs = Vec::new();
    for rel in &finals {
        if TEXT.contains(&ext_of(rel).as_str()) {
            continue;
        }
        let source = c.sources().join(format!("{rel}.md"));
        let Ok(body) = fs::read_to_string(&source) else {
            continue;
        };
        let body = body
            .split_once("\n---")
            .filter(|_| body.starts_with("---\n"))
            .map(|x| x.1)
            .unwrap_or(&body);
        let named = new_names
            .iter()
            .filter(|n| body.contains(n.as_str()))
            .collect::<Vec<_>>();
        if named.is_empty() {
            continue;
        }
        let stale = stale_names(c, rel, &keys)?;
        if !stale.is_empty() {
            refs.push(format!("{rel}\t{}", stale.join(" / ")));
        }
    }
    fs::write(
        c.refs_path(),
        format!("パス\t中身に残る古い名前\n{}", refs.iter().map(|r| r.clone() + "\n").collect::<String>()),
    )?;
    changed.insert(repo(&c.refs_path()));
    changed.insert(repo(&c.map_path()));
    save_record(
        c,
        &json!({"pairs": pairs, "changed": changed, "refs": refs.iter().map(|r| r.split('\t').next().unwrap()).collect::<Vec<_>>()}),
    )?;
    Ok(refs)
}
/// Old names still present in a deliverable's extracted text.
fn stale_names(c: &Ctx, rel: &str, keys: &[(String, String)]) -> Result<Vec<String>> {
    let p = c.home().join(rel);
    let (text, _, _) = crate::check::extract(&p, &ext_of(rel), None)?;
    Ok(keys
        .iter()
        .filter(|(k, _)| text.contains(k.as_str()))
        .map(|(k, _)| k.clone())
        .collect())
}
fn verify(c: &Ctx) -> Result<Vec<String>> {
    let record = load_record(c)?;
    let pairs: Vec<(String, String)> = serde_json::from_value(record["pairs"].clone())?;
    let refs: Vec<String> = serde_json::from_value(record["refs"].clone())?;
    let recorded = c.recorded()?;
    let mut errors = Vec::new();
    let mut finals = recorded.keys().cloned().collect::<BTreeSet<_>>();
    for (old, new) in &pairs {
        finals.remove(old);
        finals.insert(new.clone());
        if c.home().join(old).exists() || !c.home().join(new).is_file() {
            errors.push(format!("付け直しが反映されていない: {old} → {new}"));
        }
    }
    let accepted = recorded.keys().cloned().collect::<BTreeSet<_>>();
    let keys = replacements(&pairs, &accepted);
    for rel in &refs {
        if !c.home().join(rel).is_file() {
            errors.push(format!("参照を直すファイルがない: {rel}"));
            continue;
        }
        match stale_names(c, rel, &keys) {
            Ok(s) if s.is_empty() => {}
            Ok(s) => errors.push(format!("中身に古い名前が残る: {rel}: {}", s.join(" / "))),
            Err(e) => errors.push(format!("中身を読めない: {rel}: {e}")),
        }
    }
    let rel_keys = pairs.iter().map(|(o, _)| o.clone()).collect::<Vec<_>>();
    for p in md_files(&c.sources()) {
        let text = fs::read_to_string(&p).unwrap_or_default();
        if let Some(k) = rel_keys.iter().find(|k| text.contains(k.as_str())) {
            errors.push(format!("原稿に古いパスが残る: {}: {k}", p.display()));
        }
    }
    if !errors.is_empty() {
        return Ok(errors);
    }
    // Keep each file's recorded time (the document's date), then record it.
    let old_of = pairs
        .iter()
        .map(|(o, n)| (n.clone(), o.clone()))
        .collect::<BTreeMap<_, _>>();
    for rel in &finals {
        let row = &recorded[old_of.get(rel).unwrap_or(rel)];
        let stamp = chrono::DateTime::parse_from_rfc3339(&row[3])?;
        let t = filetime::FileTime::from_unix_time(stamp.timestamp(), 0);
        filetime::set_file_mtime(c.home().join(rel), t)?;
    }
    let rows = entries(&c.home(), &finals)?;
    let mut out = String::new();
    for r in &rows {
        out.push_str(&r.join("\t"));
        out.push('\n');
    }
    fs::write(c.manifest(), out)?;
    for (old, new) in &pairs {
        if recorded[old][1] != digest(&c.home().join(new))?
            && !refs.contains(new)
            && !TEXT.contains(&ext_of(new).as_str())
        {
            errors.push(format!("付け直しだけのはずの中身が変わった: {new}"));
        }
    }
    let selected = BTreeMap::from([(c.persona.clone(), finals)]);
    let mut args = vec![c.persona.clone()];
    args.extend(c.loc.check_args());
    let report = c.state_dir().join("check.tsv");
    args.extend(["--report".into(), report.to_string_lossy().into_owned()]);
    if crate::check::run_selected(&args, Some(&selected)) != 0 {
        errors.push(format!(
            "検査に通らない: {}",
            fs::read_to_string(&report).unwrap_or_default().trim()
        ));
    }
    Ok(errors)
}
fn git(base: &Path, args: &[&str]) -> Result<String> {
    let o = Command::new("git").arg("-C").arg(base).args(args).output()?;
    if !o.status.success() {
        bail!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&o.stderr)
        )
    }
    Ok(String::from_utf8(o.stdout)?)
}
fn lock_file(base: &Path, rel: &str, wait: bool) -> Result<File> {
    let p = base.join(rel);
    fs::create_dir_all(p.parent().unwrap())?;
    let f = OpenOptions::new().create(true).append(true).open(p)?;
    if wait {
        f.lock_exclusive()?
    } else {
        f.try_lock_exclusive()?
    }
    Ok(f)
}
fn commit(c: &Ctx, push: bool) -> Result<String> {
    let record = load_record(c)?;
    let pairs: Vec<(String, String)> = serde_json::from_value(record["pairs"].clone())?;
    let refs: Vec<String> = serde_json::from_value(record["refs"].clone())?;
    let mut paths: BTreeSet<String> = serde_json::from_value(record["changed"].clone())?;
    let repo = |p: &Path| {
        p.strip_prefix(&c.base)
            .unwrap_or(p)
            .to_string_lossy()
            .into_owned()
    };
    for rel in &refs {
        paths.insert(repo(&c.home().join(rel)));
        let s = c.sources().join(format!("{rel}.md"));
        if s.exists() {
            paths.insert(repo(&s));
        }
    }
    paths.insert(repo(&c.manifest()));
    paths.insert(repo(&c.loc.personas.join(format!("{}.md", c.persona))));
    let audit = c.progress().join("rename.audit.md");
    if audit.exists() {
        paths.insert(repo(&audit));
    }
    let _records = lock_file(&c.base, "scratch/.locks/records.lock", true)?;
    let tracked = git(&c.base, &["ls-files", "-z"])?
        .split('\0')
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    paths.retain(|p| c.base.join(p).exists() || tracked.contains(p));
    let mut args = vec!["add", "--"];
    args.extend(paths.iter().map(String::as_str));
    git(&c.base, &args)?;
    let staged = git(&c.base, &["diff", "--cached", "--name-only", "-z"])?;
    if let Some(p) = staged
        .split('\0')
        .filter(|s| !s.is_empty())
        .find(|p| !paths.contains(*p))
    {
        bail!("stage の範囲外の変更がある: {p}")
    }
    let message = format!(
        "Rename {} files: 中身を説明する名前・日付の不統一を付け直し ({} files, 中身の参照の修正 {})",
        c.persona,
        pairs.len(),
        refs.len()
    );
    git(&c.base, &["commit", "-qm", &message])?;
    if push {
        git(&c.base, &["push", "-q", "origin", "main"])?;
    }
    let _ = fs::remove_file(c.state_dir().join("applied.json"));
    Ok(message)
}

// ---- driver -------------------------------------------------------------

struct Driver {
    loc: Locations,
    base: PathBuf,
}
const LIMIT: &str = r"(?i)利用上限|usage limit|rate limit|\b429\b|\bquota\b";
impl Driver {
    fn ops(&self) -> PathBuf {
        std::env::var_os("AGENT_OPS_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.base.join("ops"))
    }
    fn log(&self, p: &str, what: &str, detail: &str) -> Result<()> {
        let path = self.base.join("scratch/rename/log.tsv");
        fs::create_dir_all(path.parent().unwrap())?;
        let mut f = OpenOptions::new().create(true).append(true).open(path)?;
        use std::io::Write;
        writeln!(
            f,
            "{}\t{p}\t{what}\t{}",
            chrono::Utc::now().to_rfc3339(),
            detail.replace(['\n', '\t'], " ")
        )?;
        Ok(())
    }
    fn job_status(&self, name: &str) -> Result<String> {
        let o = Command::new(self.base.join("ops/bin/codex-job"))
            .args(["status", "--repo", "personaCorpus"])
            .output()?;
        Ok(String::from_utf8_lossy(&o.stdout)
            .lines()
            .find_map(|l| {
                let c = l.split('\t').collect::<Vec<_>>();
                (c.len() >= 3 && c[0] == "personaCorpus" && c[1] == name).then(|| c[2].to_string())
            })
            .unwrap_or_else(|| "missing".into()))
    }
    fn limited(&self, name: &str) -> bool {
        let raw = fs::read(self.ops().join("jobs/personaCorpus").join(name).join("log"))
            .unwrap_or_default();
        let tail = String::from_utf8_lossy(&raw[raw.len().saturating_sub(8192)..]).into_owned();
        re(LIMIT).is_match(&tail)
    }
    fn launch(&self, p: &str, kind: &str, model: &str, extra: &str) -> Result<String> {
        let short = p.split('-').next().unwrap();
        let mut name = format!("{short}-{kind}");
        let mut i = 1;
        while self.ops().join("jobs/personaCorpus").join(&name).exists() {
            i += 1;
            name = format!("{short}-{kind}-r{i}");
        }
        let work = self.base.join("scratch/rename").join(p).join(&name);
        fs::create_dir_all(&work)?;
        let context = json!({"persona": p, "scratch": work.strip_prefix(&self.base)?.to_string_lossy()});
        let brief = work.join("brief.md");
        fs::write(
            &brief,
            fs::read_to_string(self.base.join("prompts").join(format!("{kind}.md")))?
                + "\n\nRENAME_CONTEXT\n"
                + &serde_json::to_string(&context)?
                + "\n"
                + extra,
        )?;
        let cmd = shell_words::join([
            self.base.join("bin/batch").to_string_lossy().into_owned(),
            "codex-once".into(),
            "--brief".into(),
            brief.to_string_lossy().into_owned(),
            "--output".into(),
            work.join("out.md").to_string_lossy().into_owned(),
        ]);
        let command = format!("cd {} && {cmd}", shell_words::quote(&self.base.to_string_lossy()));
        let o = Command::new(self.base.join("ops/bin/codex-job"))
            .args([
                "start", "--repo", "personaCorpus", "--name", &name, "--model", model, "--effort",
                "medium", "--command", &command,
            ])
            .output()?;
        if !o.status.success() {
            bail!("codex-job start: {}", String::from_utf8_lossy(&o.stderr))
        }
        self.log(p, kind, &name)?;
        Ok(name)
    }
    fn state_path(&self, p: &str) -> PathBuf {
        self.base.join("scratch/rename").join(p).join("state.json")
    }
    fn state(&self, p: &str) -> Value {
        fs::read_to_string(self.state_path(p))
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(json!({"stage": "names", "attempts": 0}))
    }
    fn store(&self, p: &str, v: &Value) -> Result<()> {
        fs::create_dir_all(self.state_path(p).parent().unwrap())?;
        fs::write(self.state_path(p), serde_json::to_string_pretty(v)?)?;
        Ok(())
    }
    /// One step for one persona. Returns true when the persona is finished.
    fn step(&self, p: &str, push: bool, paused: &mut Option<SystemTime>) -> Result<bool> {
        let mut s = self.state(p);
        let stage = s["stage"].as_str().unwrap_or("names").to_string();
        if ["done", "hold"].contains(&stage.as_str()) {
            return Ok(true);
        }
        let ctx = Ctx::new(self.loc.clone(), p)?;
        let job = s["job"].as_str().unwrap_or("").to_string();
        if !job.is_empty() {
            let status = self.job_status(&job)?;
            if ["running", "stalled", "stdin-wait", "limit"].contains(&status.as_str()) {
                return Ok(false);
            }
            s["job"] = json!(null);
            if status != "done" {
                if self.limited(&job) {
                    self.log(p, "limit", &job)?;
                    *paused = Some(SystemTime::now() + Duration::from_secs(1800));
                } else {
                    s["attempts"] = json!(s["attempts"].as_i64().unwrap_or(0) + 1);
                    s["errors"] = json!(format!("Codex ジョブが正常終了しなかった: {job}"));
                }
                self.store(p, &s)?;
                return Ok(false);
            }
            s["ran"] = json!(job);
            self.store(p, &s)?;
        }
        let attempts = s["attempts"].as_i64().unwrap_or(0);
        let hold = |s: &mut Value, reason: String| -> Result<bool> {
            s["stage"] = json!("hold");
            s["reason"] = json!(reason);
            self.store(p, s)?;
            self.log(p, "hold", s["reason"].as_str().unwrap())?;
            Ok(true)
        };
        let errors_note = |s: &Value| {
            s["errors"]
                .as_str()
                .map(|e| format!("\n前回の不合格の理由（直すこと）:\n{e}\n"))
                .unwrap_or_default()
        };
        let can_launch = paused.is_none_or(|t| SystemTime::now() >= t);
        match stage.as_str() {
            "names" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    s["job"] = json!(self.launch(p, "rename", "gpt-6.2-sol", &errors_note(&s))?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                let errors = ctx.pairs().and_then(|pairs| ctx.validate(&pairs));
                match errors {
                    Ok(e) if e.is_empty() => {
                        let refs = apply(&ctx)?;
                        self.log(p, "applied", &format!("refs {}", refs.len()))?;
                        s = json!({"stage": if refs.is_empty() {"verify"} else {"fix"}, "attempts": 0});
                    }
                    other => {
                        let e = match other {
                            Ok(e) => e.join("\n"),
                            Err(e) => e.to_string(),
                        };
                        if attempts >= 2 {
                            return hold(&mut s, e);
                        }
                        s = json!({"stage": "names", "attempts": attempts + 1, "errors": e});
                    }
                }
                self.store(p, &s)?;
            }
            "fix" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    s["job"] = json!(self.launch(p, "rename-fix", "gpt-6.2-sol", &errors_note(&s))?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                s = json!({"stage": "verify", "attempts": attempts});
                self.store(p, &s)?;
            }
            "verify" => {
                let errors = verify(&ctx)?;
                if errors.is_empty() {
                    s = json!({"stage": "audit", "attempts": 0});
                } else if attempts >= 2 {
                    return hold(&mut s, errors.join("\n"));
                } else {
                    s = json!({"stage": "fix", "attempts": attempts + 1, "errors": errors.join("\n")});
                }
                self.store(p, &s)?;
            }
            "audit" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    let _ = fs::remove_file(ctx.progress().join("rename.audit.md"));
                    s["job"] = json!(self.launch(p, "rename-audit", "gpt-6.2-astra", "")?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                let audit = fs::read_to_string(ctx.progress().join("rename.audit.md")).unwrap_or_default();
                match audit.lines().next() {
                    Some("verdict: accept") => {
                        let message = commit(&ctx, push)?;
                        self.log(p, "commit", &message)?;
                        s = json!({"stage": "done"});
                        self.store(p, &s)?;
                        return Ok(true);
                    }
                    Some("verdict: hold") => {
                        return hold(&mut s, "監査 hold: progress/".to_string() + p + "/rename.audit.md");
                    }
                    _ if attempts >= 2 => return hold(&mut s, "監査の書式が不正".into()),
                    _ => {
                        s = json!({"stage": "audit", "attempts": attempts + 1});
                        self.store(p, &s)?;
                    }
                }
            }
            other => bail!("unknown stage {other}"),
        }
        Ok(false)
    }
}

#[derive(Parser)]
#[command(
    name = "rename",
    about = "Rename accepted deliverables to names a person would give: validate|apply|verify|commit|run <persona>..."
)]
struct Options {
    action: String,
    personas: Vec<String>,
    #[arg(long)]
    push: bool,
    #[arg(long, default_value_t = 60)]
    interval: u64,
    #[command(flatten)]
    locations: Locations,
}
fn execute(o: Options) -> Result<i32> {
    let one = || -> Result<Ctx> {
        match o.personas.as_slice() {
            [p] => Ctx::new(o.locations.clone(), p),
            _ => bail!("persona を 1 つ指定する"),
        }
    };
    match o.action.as_str() {
        "validate" => {
            let c = one()?;
            let e = c.validate(&c.pairs()?)?;
            for l in &e {
                println!("{l}");
            }
            Ok(if e.is_empty() { 0 } else { 1 })
        }
        "apply" => {
            let c = one()?;
            let _l = lock_file(&c.base, &format!("scratch/{}/.lock", c.persona), false)
                .context("同じ persona の batch が進行中")?;
            for r in apply(&c)? {
                println!("{r}");
            }
            Ok(0)
        }
        "verify" => {
            let c = one()?;
            let e = verify(&c)?;
            for l in &e {
                println!("{l}");
            }
            Ok(if e.is_empty() { 0 } else { 1 })
        }
        "commit" => {
            let c = one()?;
            println!("{}", commit(&c, o.push)?);
            Ok(0)
        }
        "run" => {
            let (personas, push, interval) = (o.personas.clone(), o.push, o.interval);
            let base = o.locations.root.parent().unwrap().to_path_buf();
            let d = Driver {
                loc: o.locations.clone(),
                base: base.clone(),
            };
            let names = personas
                .iter()
                .map(|p| resolve(&o.locations.root, p))
                .collect::<Result<Vec<_>>>()?;
            let mut locks = BTreeMap::<String, File>::new();
            let mut paused = None;
            loop {
                let mut open = 0;
                for p in &names {
                    if !locks.contains_key(p) {
                        if d.state(p)["stage"] == "done" {
                            continue;
                        }
                        match lock_file(&base, &format!("scratch/{p}/.lock"), false) {
                            Ok(f) => {
                                locks.insert(p.clone(), f);
                            }
                            Err(_) => {
                                open += 1;
                                continue;
                            }
                        }
                    }
                    match d.step(p, push, &mut paused) {
                        Ok(true) => {
                            locks.remove(p);
                        }
                        Ok(false) => open += 1,
                        Err(e) => {
                            d.log(p, "error", &format!("{e:#}"))?;
                            let mut s = d.state(p);
                            s["stage"] = json!("hold");
                            s["reason"] = json!(format!("{e:#}"));
                            d.store(p, &s)?;
                            locks.remove(p);
                        }
                    }
                }
                if open == 0 {
                    return Ok(0);
                }
                std::thread::sleep(Duration::from_secs(interval));
            }
        }
        other => bail!("unknown action: {other}"),
    }
}
pub fn run(args: &[String]) -> i32 {
    let o = match Options::try_parse_from(std::iter::once("rename".to_string()).chain(args.iter().cloned())) {
        Ok(o) => o,
        Err(e) => {
            let code = e.exit_code();
            let _ = e.print();
            return code;
        }
    };
    match execute(o) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e:#}");
            1
        }
    }
}
