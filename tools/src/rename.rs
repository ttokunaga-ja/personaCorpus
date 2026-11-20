//! File-name rules and the one-off renaming of accepted deliverables.
//!
//! `apply` moves files and rewrites plain-text references from
//! `progress/<persona>/rename.tsv`; files whose content still names an old
//! file are listed in `rename-refs.tsv` for a Codex job to rebuild. `verify`
//! restores the recorded modification times, rewrites the manifest and runs
//! the corpus checks. `commit` stages exactly the touched paths. `run` drives
//! the Codex jobs (names, content fixes, audit, revision) and the steps above.
use crate::manifest::{Locations, Row, absolute, accepted, digest, entries, read_rows, resolve};
use anyhow::{Context, Result, bail};
use clap::Parser;
use fs2::FileExt;
use regex::{Regex, RegexBuilder};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, SystemTime},
};
use unicode_casefold::UnicodeCaseFold;
use unicode_normalization::UnicodeNormalization;

/// Longest stem a person plausibly types; saved mail keeps its subject.
pub const STEM_MAX: usize = 40;
pub const EML_STEM_MAX: usize = 60;
const TEXT: [&str; 8] = ["txt", "md", "csv", "tsv", "html", "json", "xml", "log"];
type Pairs = Vec<(String, String)>;
type Inventory = BTreeMap<String, Row>;
const DATE_FORMS: &[&str] = &[
    "YYYY-MM-DD",
    "YYYY_MM_DD",
    "YYYY.MM.DD",
    "YYYY年M月D日",
    "YYYY年MM月DD日",
    "MM-DD",
    "MM.DD",
    "MMDD",
    "M月D日",
    "MM月DD日",
    "D日",
    "DD日",
    "D Mon",
    "DD Mon",
    "D Month",
    "DD Month",
    "Mon D",
    "Mon DD",
    "Month D",
    "Month DD",
    "D-Mon",
    "DD-Mon",
    "D_Mon",
    "DD_Mon",
];
const TOKENS: &str = "YYYY|Month|Mon|MM|DD|M|D";
const MON: &str = "Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec";
const MONTH: &str =
    "January|February|March|April|May|June|July|August|September|October|November|December";

fn re(s: &str) -> Regex {
    Regex::new(s).unwrap()
}

fn expand_date(format: &str, mut expand: impl FnMut(&str) -> String) -> String {
    use std::sync::LazyLock;
    static TOKEN: LazyLock<Regex> = LazyLock::new(|| re(TOKENS));
    let mut out = String::new();
    let mut last = 0;
    for m in TOKEN.find_iter(format) {
        out.push_str(&regex::escape(&format[last..m.start()]));
        out.push_str(&expand(m.as_str()));
        last = m.end();
    }
    out.push_str(&regex::escape(&format[last..]));
    out
}
fn date_pattern(format: &str) -> String {
    expand_date(format, |token| match token {
        "YYYY" => r"[0-9]{4}".into(),
        "MM" => r"(?:0[1-9]|1[0-2])".into(),
        "DD" => r"(?:0[1-9]|[12][0-9]|3[01])".into(),
        "M" => r"(?:1[0-2]|[1-9])".into(),
        "D" => r"(?:3[01]|[12][0-9]|[1-9])".into(),
        "Mon" => format!("(?:{MON})"),
        _ => format!("(?:{MONTH})"),
    })
}

pub(crate) struct NameRules<'a> {
    spec: Option<&'a str>,
    prefix: Option<Regex>,
    formats: Vec<&'a str>,
}
impl<'a> NameRules<'a> {
    pub(crate) fn new(spec: Option<&'a str>) -> Result<Self> {
        let formats: Vec<_> = spec
            .into_iter()
            .flat_map(|s| s.split('|'))
            .map(str::trim)
            .collect();
        let prefix = if spec.is_some() {
            for alt in &formats {
                let pattern = date_pattern(alt);
                if alt.is_empty() || pattern == regex::escape(alt) {
                    bail!("date_prefix の書式が不正: {alt}");
                }
            }
            let patterns = formats
                .iter()
                .map(|alt| {
                    let pattern = date_pattern(alt);
                    // A final space in the front matter is trimmed. Still require a
                    // separator when the format itself ends in a date token.
                    if alt.ends_with(" -") {
                        format!("{pattern} ")
                    } else if alt.ends_with(|c: char| c.is_ascii_alphabetic()) {
                        format!("{pattern}(?:[^[:alnum:]]|$)")
                    } else {
                        pattern
                    }
                })
                .collect::<Vec<_>>();
            Some(
                RegexBuilder::new(&format!(r"\A(?:{})", patterns.join("|")))
                    .case_insensitive(true)
                    .build()
                    .context("date_prefix")?,
            )
        } else {
            None
        };
        Ok(Self {
            spec,
            prefix,
            formats,
        })
    }

    pub(crate) fn name_problems(&self, rel: &str) -> Vec<String> {
        use std::sync::LazyLock;
        static DATE: LazyLock<Regex> = LazyLock::new(|| {
            re(&format!(
                r"(?i)\A(?:(?:{})(?:[^[:alnum:]]|$)|[0-9]{{4}}[-_.][0-9]{{1,2}}(?:[^0-9]|$)|[0-9]{{1,2}}月|[0-9]{{1,2}}日)",
                DATE_FORMS
                    .iter()
                    .filter(|f| **f != "MMDD")
                    .map(|f| date_pattern(f))
                    .collect::<Vec<_>>()
                    .join("|")
            ))
        });
        static COMPACT: LazyLock<Regex> =
            LazyLock::new(|| re(&format!(r"\A{}[-_ .]", date_pattern("MMDD"))));
        // Only flag unmistakable sentence patterns here. The naming review
        // checks summaries and conversational titles that need context.
        static SENTENCE: LazyLock<Regex> = LazyLock::new(|| {
            re(
                r"(?i)\A(?:do not|don't|(?:i|we) (?:looked|checked|wrote|thought|asked|noticed|didn't))\s|(?:に見えた|と思っていた|していない|ではない|できない|しない|入れない|使わない|ません|ました|[。！？])\z",
            )
        });
        let mut out = Vec::new();
        let path = Path::new(rel);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let mail = ext_of(rel) == "eml";
        let max = if mail { EML_STEM_MAX } else { STEM_MAX };
        if stem.chars().count() > max {
            out.push(format!("名前が長すぎる（{max} 文字まで）: {rel}"));
        }
        let date = DATE.find(&stem).or_else(|| COMPACT.find(&stem));
        let title = date
            .map_or(stem.as_ref(), |m| &stem[m.end()..])
            .trim_start_matches([' ', '-', '_']);
        if !mail && SENTENCE.is_match(title) {
            out.push(format!("名前が要約・結論・会話の文になっている: {rel}"));
        }
        if let Some(prefix) = &self.prefix
            && date.is_some()
            && !prefix.is_match(&stem)
        {
            out.push(format!(
                "日付の形が定義（{}）と違う: {rel}",
                self.spec.unwrap()
            ));
        }
        for dir in path.parent().into_iter().flat_map(Path::components) {
            let dir = dir.as_os_str().to_string_lossy();
            let Ok(day) = chrono::NaiveDate::parse_from_str(&dir, "%Y-%m-%d") else {
                continue;
            };
            if stem.contains(dir.as_ref())
                || self.formats.iter().chain(DATE_FORMS).any(|format| {
                    let date = expand_date(format, |token| {
                        let part = match token {
                            "YYYY" => "%Y",
                            "MM" => "%m",
                            "DD" => "%d",
                            "M" => "%-m",
                            "D" => "%-d",
                            "Mon" => "%b",
                            _ => "%B",
                        };
                        day.format(part).to_string()
                    });
                    RegexBuilder::new(&format!(r"\A{date}(?:[^[:alnum:]]|$)"))
                        .case_insensitive(true)
                        .build()
                        .is_ok_and(|r| r.is_match(&stem))
                })
            {
                out.push(format!(
                    "日付のフォルダと同じ日付を名前にも付けている: {rel}"
                ));
                break;
            }
        }
        out
    }
}

struct Ctx {
    loc: Locations,
    base: PathBuf,
    persona: String,
}
impl Ctx {
    fn new(loc: Locations, persona: &str) -> Result<Self> {
        let loc = Locations {
            root: absolute(&loc.root)?,
            personas: absolute(&loc.personas)?,
            progress: absolute(&loc.progress)?,
            manifests: absolute(&loc.manifests)?,
            sources: absolute(&loc.sources)?,
        };
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
    fn definition(&self) -> PathBuf {
        self.loc.personas.join(format!("{}.md", self.persona))
    }
    fn audit(&self) -> PathBuf {
        self.progress().join("rename.audit.md")
    }
    fn date_prefix(&self) -> Result<Option<String>> {
        Ok(
            crate::batch::config_text(&fs::read_to_string(self.definition())?)
                .remove("date_prefix"),
        )
    }
    fn relative(&self, path: &Path) -> Result<String> {
        Ok(path
            .strip_prefix(&self.base)
            .with_context(|| format!("リポジトリの外のパス: {}", path.display()))?
            .to_str()
            .context("パスが UTF-8 ではない")?
            .to_string())
    }
    fn check_locations(&self) -> Result<()> {
        for path in [
            &self.loc.root,
            &self.loc.sources,
            &self.loc.progress,
            &self.loc.manifests,
            &self.loc.personas,
            &self.progress(),
            &self.sources(),
            &self.state_dir(),
        ] {
            self.relative(path)?;
            if crate::manifest::resolve_path(path)? != *path {
                bail!("リンクを含むフォルダ: {}", path.display());
            }
        }
        for path in [self.definition(), self.manifest(), self.map_path()] {
            crate::batch::safe_file(&self.base, &self.relative(&path)?)?;
        }
        Ok(())
    }
    fn lock(&self) -> Result<File> {
        self.check_locations()?;
        lock_file(
            &self.base,
            &format!("scratch/rename/{}/.lock", self.persona),
            true,
        )
    }

    /// (old, new) pairs from rename.tsv.
    fn pairs(&self) -> Result<Pairs> {
        let text = fs::read_to_string(self.map_path())
            .with_context(|| format!("{} がない", self.map_path().display()))?;
        let mut lines = text.lines().filter(|l| !l.trim().is_empty());
        if lines.next() != Some("旧\t新\t理由") {
            bail!("rename.tsv の見出しは「旧<TAB>新<TAB>理由」")
        }
        lines
            .map(|l| {
                let c = l.split('\t').collect::<Vec<_>>();
                if c.len() != 3 || c[2].trim().is_empty() {
                    bail!("rename.tsv の行は 3 列（旧、新、理由）: {l}")
                }
                Ok((c[0].to_string(), c[1].to_string()))
            })
            .collect()
    }
    fn recorded(&self) -> Result<BTreeMap<String, Row>> {
        let mut rows = BTreeMap::new();
        for row in read_rows(&self.manifest())? {
            let path = row[0].clone();
            crate::batch::safe_file(&self.home(), &path)?;
            chrono::DateTime::parse_from_rfc3339(&row[3])
                .with_context(|| format!("manifest の日時が不正: {path}"))?;
            if rows.insert(path.clone(), row).is_some() {
                bail!("manifest のパスが重複: {path}");
            }
        }
        Ok(rows)
    }
    fn inventory(&self) -> Result<BTreeMap<String, Row>> {
        let rows = self.recorded()?;
        let head = accepted(&self.home())?;
        if rows.keys().ne(head.iter()) {
            bail!(
                "manifest のパスが受入済みの HEAD と一致しない: {}",
                self.persona
            );
        }
        Ok(rows)
    }
    fn validation(&self) -> Result<Vec<String>> {
        if self.state_dir().join("applied.json").try_exists()? {
            let (current, delta) = revision_plan(&load_record(self)?, &self.pairs()?)?;
            return self.validate(&delta, &current);
        }
        let rows = self.inventory()?;
        match self.pairs() {
            Ok(pairs) => self.validate(&pairs, &rows),
            Err(e) => {
                let mut errors = self.validate(&[], &rows)?;
                errors.insert(0, format!("{e:#}"));
                Ok(errors)
            }
        }
    }
    fn validate(
        &self,
        pairs: &[(String, String)],
        rows: &BTreeMap<String, Row>,
    ) -> Result<Vec<String>> {
        let prefix = self.date_prefix()?;
        let mut errors = Vec::new();
        if prefix.is_none() {
            errors.push("persona の定義の先頭設定に date_prefix がない".into());
        }
        let rules = match NameRules::new(prefix.as_deref()) {
            Ok(rules) => rules,
            Err(e) => {
                errors.push(e.to_string());
                NameRules::new(None)?
            }
        };
        let folded = |p: &str| p.nfkc().case_fold().collect::<String>();
        let mut news = BTreeSet::new();
        let mut olds = BTreeSet::new();
        let renamed: BTreeSet<_> = pairs.iter().map(|(old, _)| old).collect();
        let retained: BTreeSet<_> = rows
            .keys()
            .filter(|p| !renamed.contains(p))
            .map(|p| folded(p))
            .collect();
        let files: BTreeSet<_> = rows
            .keys()
            .chain(pairs.iter().map(|(_, new)| new))
            .map(|p| folded(p))
            .collect();
        for (old, new) in pairs {
            if !rows.contains_key(old) {
                errors.push(format!("受入済みにない: {old}"));
            }
            if !olds.insert(old) {
                errors.push(format!("旧が重複: {old}"));
            }
            let target = crate::batch::safe_file(&self.home(), new);
            let source = crate::batch::safe_file(&self.sources(), &format!("{new}.md"));
            if target.is_err()
                || source.is_err()
                || new.contains('|')
                || new.chars().any(char::is_control)
            {
                errors.push(format!("新のパスが不正: {new}"));
                continue;
            }
            let key = folded(new);
            if !news.insert(key.clone())
                || retained.contains(&key)
                || rows.contains_key(new)
                || target?.try_exists()?
            {
                errors.push(format!("新が重複または既存: {new}"));
            }
            if source?.try_exists()? {
                errors.push(format!("新の原稿が既存: {new}.md"));
            }
            if Path::new(new)
                .ancestors()
                .skip(1)
                .any(|p| files.contains(&folded(&p.to_string_lossy())))
            {
                errors.push(format!("新の親がファイルと衝突する: {new}"));
            }
            if ext_of(old) != ext_of(new) {
                errors.push(format!("拡張子が変わる: {old} → {new}"));
            }
            errors.extend(rules.name_problems(new));
            if crate::batch::bad_path(new) {
                errors.push(format!("制作区分または ID: {new}"));
            }
        }
        for rel in rows.keys().filter(|p| !olds.contains(p)) {
            let mut problems = rules.name_problems(rel);
            if crate::batch::bad_path(rel) {
                problems.push(format!("制作区分または ID: {rel}"));
            }
            errors.extend(problems.into_iter().map(|e| format!("付け直しが要る: {e}")));
        }
        Ok(errors)
    }
}

fn basename(rel: &str) -> &str {
    rel.rsplit('/').next().unwrap()
}

struct References {
    pattern: Option<Regex>,
    replacements: BTreeMap<String, String>,
}
impl References {
    fn new(
        pairs: &[(String, String)],
        original: &BTreeMap<String, Row>,
        finals: &BTreeMap<String, String>,
        persona: &str,
    ) -> Result<Self> {
        let mut counts = BTreeMap::<&str, usize>::new();
        for path in original.keys() {
            *counts.entry(basename(path)).or_default() += 1;
        }
        let final_names: BTreeSet<_> = finals.keys().map(|p| basename(p)).collect();
        let mut replacements = BTreeMap::new();
        for (old, new) in pairs {
            if old.contains('/') {
                replacements.insert(old.clone(), new.clone());
            } else {
                // A root-level path and a bare basename have the same spelling.
                // Explicit paths remain usable when the basename is ambiguous.
                for (prefix, suffix) in [
                    ("./".to_string(), ""),
                    (format!("people/{persona}/"), ""),
                    (format!("sources/{persona}/"), ".md"),
                    ("path: ".to_string(), ""),
                    ("derived_from: ".to_string(), ""),
                    ("scan:".to_string(), ""),
                    ("| ".to_string(), " |"),
                ] {
                    replacements.insert(
                        format!("{prefix}{old}{suffix}"),
                        format!("{prefix}{new}{suffix}"),
                    );
                }
            }
            if counts[basename(old)] == 1 && !final_names.contains(basename(old)) {
                replacements.insert(basename(old).into(), basename(new).into());
            }
        }
        // A new name can itself contain an old name (for example "Revised notes.txt").
        for new in finals.keys().filter(|_| !pairs.is_empty()) {
            for new in [new.as_str(), basename(new)] {
                replacements.entry(new.into()).or_insert_with(|| new.into());
            }
        }
        let mut keys: Vec<_> = replacements.keys().collect();
        keys.sort_by_key(|name| std::cmp::Reverse(name.len()));
        let pattern = if keys.is_empty() {
            None
        } else {
            Some(
                RegexBuilder::new(
                    &keys
                        .into_iter()
                        .map(|k| regex::escape(k))
                        .collect::<Vec<_>>()
                        .join("|"),
                )
                .size_limit(32 * 1024 * 1024)
                .build()
                .context("旧名の検索パターン")?,
            )
        };
        Ok(Self {
            pattern,
            replacements,
        })
    }

    fn matches<'a>(&'a self, text: &'a str) -> impl Iterator<Item = regex::Match<'a>> {
        self.pattern
            .iter()
            .flat_map(move |pattern| pattern.find_iter(text))
            .filter(move |m| {
                let tail = &text[m.end()..];
                let embedded_before = text[..m.start()]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || "_-.".contains(c));
                let embedded_after = tail
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || "_-".contains(c))
                    || (tail.starts_with('.')
                        && tail[1..].starts_with(|c: char| c.is_ascii_alphanumeric())
                        && !tail.strip_prefix(".md").is_some_and(|after| {
                            !after.starts_with(|c: char| c.is_ascii_alphanumeric())
                        }));
                !embedded_before && !embedded_after && self.replacements[m.as_str()] != m.as_str()
            })
    }

    fn rewrite(&self, text: &str) -> Option<String> {
        let mut out = String::new();
        let mut end = 0;
        for m in self.matches(text) {
            out.push_str(&text[end..m.start()]);
            out.push_str(&self.replacements[m.as_str()]);
            end = m.end();
        }
        (end > 0).then(|| {
            out.push_str(&text[end..]);
            out
        })
    }

    fn stale(&self, text: &str) -> BTreeSet<String> {
        self.matches(text).map(|m| m.as_str().to_string()).collect()
    }
}

fn text_digest(text: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
fn ext_of(p: &str) -> String {
    Path::new(p)
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_ascii_lowercase()
}
fn text_files(dir: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>> {
    if !dir.try_exists()? {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    for entry in walkdir::WalkDir::new(dir).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_symlink() {
            bail!("リンクを含むパス: {}", entry.path().display());
        }
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| extensions.contains(&e))
        {
            paths.push(entry.into_path());
        }
    }
    Ok(paths)
}
fn project_text(c: &Ctx) -> Result<impl Iterator<Item = PathBuf>> {
    let rename_records = [c.audit(), c.map_path(), c.refs_path()];
    Ok(text_files(&c.sources(), &["md"])?.into_iter().chain(
        text_files(&c.progress(), &TEXT)?
            .into_iter()
            .filter(move |p| !rename_records.contains(p)),
    ))
}
fn read_text(p: &Path) -> Result<String> {
    fs::read_to_string(p).with_context(|| p.display().to_string())
}
fn atomic_write(p: &Path, data: &[u8], keep_time: bool) -> Result<()> {
    let parent = p.parent().context("出力先に親フォルダがない")?;
    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;
    tmp.write_all(data)?;
    if let Some(meta) = metadata_if_exists(p)? {
        tmp.as_file().set_permissions(meta.permissions())?;
        if keep_time {
            filetime::set_file_mtime(
                tmp.path(),
                filetime::FileTime::from_last_modification_time(&meta),
            )?;
        }
    }
    tmp.persist(p)
        .with_context(|| format!("書き込めない: {}", p.display()))?;
    Ok(())
}
fn metadata_if_exists(p: &Path) -> Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(p) {
        Ok(meta) if meta.is_file() => Ok(Some(meta)),
        Ok(_) => bail!("通常のファイルではない: {}", p.display()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e).with_context(|| p.display().to_string()),
    }
}
fn save_json(path: &Path, value: &Value) -> Result<()> {
    fs::create_dir_all(path.parent().context("記録のフォルダがない")?)?;
    atomic_write(path, &serde_json::to_vec_pretty(value)?, false)
}
fn load_record(c: &Ctx) -> Result<Value> {
    Ok(serde_json::from_str(
        &read_text(&c.state_dir().join("applied.json")).context("apply の記録がない")?,
    )?)
}
fn final_paths(
    rows: &BTreeMap<String, Row>,
    pairs: &[(String, String)],
) -> BTreeMap<String, String> {
    let renamed: BTreeMap<_, _> = pairs.iter().map(|(old, new)| (old, new)).collect();
    rows.keys()
        .map(|old| (renamed.get(old).map_or(old, |&p| p).clone(), old.clone()))
        .collect()
}
fn inventory_at(rows: &Inventory, pairs: &Pairs) -> Inventory {
    final_paths(rows, pairs)
        .into_iter()
        .map(|(path, old)| (path, rows[&old].clone()))
        .collect()
}
fn revision_plan(record: &Value, pairs: &Pairs) -> Result<(Inventory, Pairs)> {
    let previous: Pairs = serde_json::from_value(record["pairs"].clone())?;
    let rows: Inventory = serde_json::from_value(record["rows"].clone())?;
    let applied: BTreeMap<_, _> = previous.iter().map(|(old, new)| (old, new)).collect();
    let requested: BTreeSet<_> = pairs.iter().map(|(old, _)| old).collect();
    if requested.len() != pairs.len() || requested.into_iter().ne(applied.keys().copied()) {
        bail!("見直しで rename.tsv の旧パスが変更された");
    }
    let delta = pairs
        .iter()
        .filter_map(|(old, new)| {
            let current = applied[old];
            (current != new).then(|| (current.clone(), new.clone()))
        })
        .collect();
    Ok((inventory_at(&rows, &previous), delta))
}
fn previous_pairs(record: &Value) -> Result<Vec<Pairs>> {
    match record.get("previous_pairs") {
        Some(value) => Ok(serde_json::from_value(value.clone())?),
        None => Ok(Vec::new()),
    }
}

type Fingerprints = BTreeMap<String, Option<String>>;

fn check_revision_inputs(c: &Ctx, record: &Value) -> Result<()> {
    if record["report_hash"] != digest(&c.refs_path())? {
        bail!("適用済みの rename-refs.tsv が変更された");
    }
    let verified = c.state_dir().join("verified.json");
    if metadata_if_exists(&verified)?.is_some() {
        let mut expected: Fingerprints = serde_json::from_str(&read_text(&verified)?)?;
        let previous_hash = expected.remove(&c.relative(&c.map_path())?).flatten();
        if previous_hash.as_deref() == record["map_hash"].as_str()
            && expected != fingerprints(c, &expected.keys().cloned().collect())?
        {
            bail!("verify の後に対応表以外の付け直しに関係するファイルが変わった");
        }
    }
    Ok(())
}
fn extract_reference_text(p: &Path, rel: &str, ocr: Option<&str>) -> Result<String> {
    let ext = ext_of(rel);
    if ["wav", "mp3", "m4a"].contains(&ext.as_str()) {
        return Ok(String::new());
    }
    if ext == "zip" {
        let zip = zip::ZipArchive::new(File::open(p)?)?;
        return Ok(zip.file_names().collect::<Vec<_>>().join("\n"));
    }
    let (text, _, _) = crate::check::extract(p, &ext, ocr)?;
    Ok(text)
}

struct Edit {
    path: PathBuf,
    old: String,
    new: String,
}

fn apply(c: &Ctx) -> Result<Vec<String>> {
    let _lock = c.lock()?;
    let record_path = c.state_dir().join("applied.json");
    let pending = c.state_dir().join("applying.json");
    let map_hash = digest(&c.map_path())
        .with_context(|| format!("対応表が読めない: {}", c.map_path().display()))?;
    let applied = if metadata_if_exists(&record_path)?.is_some() {
        Some(load_record(c)?)
    } else {
        None
    };
    if pending.try_exists()? {
        let unfinished: Value = serde_json::from_str(&read_text(&pending)?)?;
        if applied.as_ref() != Some(&unfinished) {
            bail!("前回の apply が中断された: {}", pending.display());
        }
        fs::remove_file(&pending)?;
    }
    if let Some(record) = &applied
        && record["map_hash"] == map_hash
    {
        return Ok(serde_json::from_value(record["ref_rows"].clone())?);
    }
    let pairs = c.pairs()?;
    let (current, delta) = match &applied {
        Some(record) => revision_plan(record, &pairs)?,
        None => (c.inventory()?, pairs.clone()),
    };
    let errors = c.validate(&delta, &current)?;
    if !errors.is_empty() {
        bail!("rename.tsv が不正:\n{}", errors.join("\n"))
    }
    let mut history = Vec::new();
    let mut previous_changes: BTreeSet<String> = BTreeSet::new();
    let mut previous_edits: BTreeMap<String, String> = BTreeMap::new();
    let mut previous_refs: BTreeMap<String, String> = BTreeMap::new();
    let rows = if let Some(record) = &applied {
        check_revision_inputs(c, record)?;
        history = previous_pairs(record)?;
        if !delta.is_empty() {
            history.push(serde_json::from_value(record["pairs"].clone())?);
        }
        previous_changes = serde_json::from_value(record["changed"].clone())?;
        previous_edits = serde_json::from_value(record["edited_text"].clone())?;
        for row in serde_json::from_value::<Vec<String>>(record["ref_rows"].clone())? {
            let (p, names) = row.split_once('\t').context("参照の記録が不正")?;
            previous_refs.insert(p.into(), names.into());
        }
        serde_json::from_value(record["rows"].clone())?
    } else {
        current.clone()
    };
    let finals = final_paths(&current, &delta);
    let keys = References::new(&delta, &current, &finals, &c.persona)?;
    let mut changed = previous_changes.clone();
    changed.extend([c.relative(&c.refs_path())?, c.relative(&c.map_path())?]);
    let mut moves = BTreeMap::new();
    for (old, new) in &delta {
        for (from, to) in [
            (c.home().join(old), c.home().join(new)),
            (
                c.sources().join(format!("{old}.md")),
                c.sources().join(format!("{new}.md")),
            ),
        ] {
            if metadata_if_exists(&from)?.is_none() {
                bail!("移動元がない: {}", from.display());
            }
            crate::batch::safe_file(&c.base, &c.relative(&from)?)?;
            changed.insert(c.relative(&from)?);
            changed.insert(c.relative(&to)?);
            moves.insert(from, to);
        }
    }

    // Read and check all inputs before moving or rewriting any of them.
    let mut edits = Vec::new();
    let mut edited_text = BTreeMap::new();
    let mut refs = BTreeMap::<String, BTreeSet<String>>::new();
    let ocr = if delta.is_empty() {
        None
    } else {
        crate::check::ocr_language()?
    };
    for (rel, old) in &finals {
        let path = c.home().join(old);
        crate::batch::safe_file(&c.home(), old)?;
        let expected = previous_edits.get(old).unwrap_or(&current[old][1]);
        if metadata_if_exists(&path)?.is_none()
            || (!previous_refs.contains_key(old) && digest(&path)? != *expected)
        {
            bail!("配布物の中身が適用済みの記録または元の manifest と一致しない: {old}");
        }
        if let Some(hash) = previous_edits.get(old) {
            edited_text.insert(rel.clone(), hash.clone());
        }
        if let Some(names) = previous_refs.get(old) {
            refs.entry(rel.clone()).or_default().insert(names.clone());
        }
        if delta.is_empty() {
            continue;
        }
        if TEXT.contains(&ext_of(rel).as_str()) {
            let text = read_text(&path)?;
            if let Some(new) = keys.rewrite(&text) {
                changed.insert(c.relative(&c.home().join(rel))?);
                edited_text.insert(rel.clone(), text_digest(&new));
                edits.push(Edit {
                    path,
                    old: text,
                    new,
                });
            }
        } else {
            let text = extract_reference_text(&path, rel, ocr)
                .with_context(|| format!("参照の検査で中身を読めない: {old}"))?;
            let stale = keys.stale(&text);
            if !stale.is_empty() {
                refs.entry(rel.clone()).or_default().extend(stale);
            }
        }
    }
    // Scan the original text so a replacement is never applied twice to a moved source.
    let mut originals = BTreeSet::new();
    if !delta.is_empty() {
        for path in project_text(c)? {
            let text = read_text(&path)?;
            let destination = moves.get(&path).unwrap_or(&path);
            if let Some(new) = keys.rewrite(&text) {
                originals.insert(c.relative(&path)?);
                changed.insert(c.relative(destination)?);
                edits.push(Edit {
                    path,
                    old: text,
                    new,
                });
            } else if destination != &path {
                originals.insert(c.relative(&path)?);
            }
        }
    }
    originals.retain(|p| !previous_changes.contains(p));
    ensure_unmodified(c, &originals)?;
    if applied.is_some() && !moves.is_empty() {
        // Intermediate names never recorded in HEAD cannot be staged as deletions.
        let from = moves
            .keys()
            .map(|p| c.relative(p))
            .collect::<Result<Vec<_>>>()?;
        let mut args = vec!["ls-tree", "-rz", "--name-only", "HEAD", "--"];
        args.extend(from.iter().map(String::as_str));
        let head = git(&c.base, &args)?;
        let tracked: BTreeSet<_> = head.split_terminator('\0').collect();
        for path in from {
            if !tracked.contains(path.as_str()) {
                changed.remove(&path);
            }
        }
    }
    let ref_rows: Vec<_> = refs
        .iter()
        .map(|(p, names)| {
            format!(
                "{p}\t{}",
                names.iter().cloned().collect::<Vec<_>>().join(" / ")
            )
        })
        .collect();
    let report = std::iter::once("パス\t中身に残る古い名前")
        .chain(ref_rows.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let refs_path = c.refs_path();
    let previous_report = metadata_if_exists(&refs_path)?
        .map(|_| fs::read(&refs_path))
        .transpose()?;
    for to in moves.values() {
        fs::create_dir_all(to.parent().context("新しいパスの親がない")?)?;
    }
    let record = json!({
        "pairs": pairs, "previous_pairs": history, "map_hash": map_hash, "report_hash": text_digest(&report),
        "rows": rows, "changed": changed,
        "edited_text": edited_text, "refs": refs.keys().collect::<Vec<_>>(), "ref_rows": ref_rows,
    });
    save_json(&pending, &record)?;
    let mut moved = Vec::new();
    let mut edited = Vec::new();
    let mut report_written = false;
    let result = (|| -> Result<()> {
        for (from, to) in &moves {
            if metadata_if_exists(to)?.is_some() {
                bail!("新しいパスが既存: {}", to.display());
            }
            fs::rename(from, to)
                .with_context(|| format!("{} → {}", from.display(), to.display()))?;
            moved.push((from, to));
        }
        for edit in &edits {
            let path = moves.get(&edit.path).unwrap_or(&edit.path);
            atomic_write(path, edit.new.as_bytes(), true)?;
            edited.push((path, &edit.old));
        }
        atomic_write(&refs_path, report.as_bytes(), false)?;
        report_written = true;
        save_json(&record_path, &record)
    })();
    if let Err(error) = result {
        let rollback = (|| -> Result<()> {
            for (path, old) in edited.into_iter().rev() {
                atomic_write(path, old.as_bytes(), true)?;
            }
            for (from, to) in moved.into_iter().rev() {
                fs::rename(to, from)?;
            }
            if report_written {
                match previous_report {
                    Some(bytes) => atomic_write(&refs_path, &bytes, false)?,
                    None => fs::remove_file(&refs_path)?,
                }
            }
            fs::remove_file(&pending)?;
            Ok(())
        })();
        if let Err(rollback) = rollback {
            bail!(
                "{error:#}\napply の復元に失敗した: {rollback:#}\n記録: {}",
                pending.display()
            );
        }
        return Err(error);
    }
    fs::remove_file(pending)?;
    for from in moves.keys() {
        // Git does not record empty directories. Remove only empty source ancestors.
        for dir in from
            .ancestors()
            .skip(1)
            .take_while(|p| *p != c.home() && *p != c.sources())
        {
            if fs::remove_dir(dir).is_err() {
                break;
            }
        }
    }
    Ok(ref_rows)
}

fn ensure_unmodified(c: &Ctx, paths: &BTreeSet<String>) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }
    let mut args = vec!["ls-files", "-z", "--"];
    args.extend(paths.iter().map(String::as_str));
    let tracked = git(&c.base, &args)?;
    let tracked: BTreeSet<_> = tracked.split_terminator('\0').collect();
    if let Some(p) = paths.iter().find(|p| !tracked.contains(p.as_str())) {
        bail!("Git 管理外の原稿または進捗を変更する: {p}");
    }
    args.splice(..2, ["diff", "HEAD", "--no-renames", "--name-only", "-z"]);
    if let Some(path) = git(&c.base, &args)?.split_terminator('\0').next() {
        bail!("原稿または進捗に既存の変更がある: {path}");
    }
    Ok(())
}

fn verify(c: &Ctx) -> Result<Vec<String>> {
    let _lock = c.lock()?;
    let verified = c.state_dir().join("verified.json");
    match fs::remove_file(&verified) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }
    let record = load_record(c)?;
    if record["map_hash"] != digest(&c.map_path())?
        || record["report_hash"] != digest(&c.refs_path())?
    {
        bail!("適用済みの rename.tsv または rename-refs.tsv が変更された");
    }
    let pairs: Pairs = serde_json::from_value(record["pairs"].clone())?;
    let refs: BTreeSet<String> = serde_json::from_value(record["refs"].clone())?;
    let edited_text: BTreeMap<String, String> =
        serde_json::from_value(record["edited_text"].clone())?;
    let rows: Inventory = serde_json::from_value(record["rows"].clone())?;
    let finals = final_paths(&rows, &pairs);
    let targets: BTreeMap<_, _> = pairs.iter().map(|(old, new)| (old, new)).collect();
    let mut keys = Vec::new();
    let mut past_paths = BTreeSet::new();
    // Verify against the original names and every applied intermediate name.
    for previous in std::iter::once(Vec::new()).chain(previous_pairs(&record)?) {
        let previous_paths = final_paths(&rows, &previous);
        let superseded: Pairs = previous_paths
            .into_iter()
            .filter(|(path, _)| !finals.contains_key(path))
            .map(|(path, old)| (path, targets[&old].clone()))
            .collect();
        past_paths.extend(superseded.iter().cloned());
        keys.push(References::new(
            &superseded,
            &inventory_at(&rows, &previous),
            &finals,
            &c.persona,
        )?);
    }
    let stale_names = |text: &str| -> BTreeSet<String> {
        keys.iter().flat_map(|keys| keys.stale(text)).collect()
    };
    let mut errors = Vec::new();
    let prefix = c
        .date_prefix()?
        .context("persona の定義の先頭設定に date_prefix がない")?;
    let names = NameRules::new(Some(&prefix))?;
    for (old, new) in &past_paths {
        if c.home().join(old).try_exists()? || c.sources().join(format!("{old}.md")).try_exists()? {
            errors.push(format!("旧パスが残る: {old} → {new}"));
        }
    }
    for path in project_text(c)? {
        let stale = stale_names(&read_text(&path)?);
        if !stale.is_empty() {
            errors.push(format!("古い名前が残る: {}: {stale:?}", path.display()));
        }
    }
    let ocr = if refs.is_empty() {
        None
    } else {
        crate::check::ocr_language()?
    };
    for (rel, old) in &finals {
        let path = crate::batch::safe_file(&c.home(), rel)?;
        errors.extend(names.name_problems(rel));
        if metadata_if_exists(&path)?.is_none() {
            errors.push(format!("付け直しが反映されていない: {old} → {rel}"));
            continue;
        }
        if TEXT.contains(&ext_of(rel).as_str()) || refs.contains(rel) {
            let text = if TEXT.contains(&ext_of(rel).as_str()) {
                read_text(&path)?
            } else {
                match extract_reference_text(&path, rel, ocr) {
                    Ok(text) => text,
                    Err(e) => {
                        errors.push(format!("中身を読めない: {rel}: {e:#}"));
                        continue;
                    }
                }
            };
            let stale = stale_names(&text);
            if !stale.is_empty() {
                errors.push(format!("中身に古い名前が残る: {rel}: {stale:?}"));
            }
        }
        let expected = edited_text.get(rel).unwrap_or(&rows[old][1]);
        if !refs.contains(rel) && digest(&path)? != *expected {
            errors.push(format!("参照の置換以外で中身が変わった: {rel}"));
        }
    }
    if !errors.is_empty() {
        return Ok(errors);
    }
    for (rel, old) in &finals {
        let stamp = chrono::DateTime::parse_from_rfc3339(&rows[old][3])?;
        let time =
            filetime::FileTime::from_unix_time(stamp.timestamp(), stamp.timestamp_subsec_nanos());
        filetime::set_file_mtime(c.home().join(rel), time)?;
    }
    let selected = BTreeMap::from([(c.persona.clone(), finals.keys().cloned().collect())]);
    let mut args = vec![c.persona.clone()];
    args.extend(c.loc.check_args());
    let report = c.state_dir().join("check.tsv");
    args.extend(["--report".into(), report.to_string_lossy().into_owned()]);
    if crate::check::run_selected(&args, Some(&selected)) != 0 {
        let detail = fs::read_to_string(&report).unwrap_or_default();
        return Ok(vec![format!(
            "検査に通らない: {}\n{}",
            report.display(),
            detail.trim()
        )]);
    }

    let rows = entries(&c.home(), &selected[&c.persona])?;
    let mut manifest = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(Vec::new());
    for row in rows {
        manifest.write_record(row)?;
    }
    atomic_write(&c.manifest(), &manifest.into_inner()?, false)?;
    let paths = commit_paths(c, &record)?;
    save_json(&verified, &json!(fingerprints(c, &paths)?))?;
    Ok(errors)
}

fn git(base: &Path, args: &[&str]) -> Result<String> {
    let o = Command::new("git")
        .arg("-C")
        .arg(base)
        .arg("--literal-pathspecs")
        .args(args)
        .output()?;
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
    if crate::manifest::resolve_path(&p)? != p {
        bail!("リンクを含むロックのパス: {}", p.display());
    }
    fs::create_dir_all(p.parent().unwrap())?;
    let f = OpenOptions::new().create(true).append(true).open(p)?;
    if wait {
        f.lock_exclusive()?
    } else {
        f.try_lock_exclusive()?
    }
    Ok(f)
}
fn commit_paths(c: &Ctx, record: &Value) -> Result<BTreeSet<String>> {
    let refs: Vec<String> = serde_json::from_value(record["refs"].clone())?;
    let mut paths: BTreeSet<String> = serde_json::from_value(record["changed"].clone())?;
    for rel in &refs {
        paths.insert(c.relative(&c.home().join(rel))?);
    }
    paths.insert(c.relative(&c.manifest())?);
    paths.insert(c.relative(&c.definition())?);
    Ok(paths)
}
fn fingerprints(c: &Ctx, paths: &BTreeSet<String>) -> Result<Fingerprints> {
    paths
        .iter()
        .map(|path| {
            let p = crate::batch::safe_file(&c.base, path)?;
            let hash = metadata_if_exists(&p)?.map(|_| digest(&p)).transpose()?;
            Ok((path.clone(), hash))
        })
        .collect()
}
fn check_index(c: &Ctx, paths: &BTreeSet<String>) -> Result<()> {
    let staged = git(
        &c.base,
        &["diff", "--cached", "--no-renames", "--name-only", "-z"],
    )?;
    if let Some(p) = staged.split_terminator('\0').find(|p| !paths.contains(*p)) {
        bail!("stage の範囲外の変更がある: {p}")
    }
    Ok(())
}
fn commit(c: &Ctx) -> Result<String> {
    let _lock = c.lock()?;
    let _records = lock_file(&c.base, "scratch/.locks/records.lock", true)?;
    let record = load_record(c)?;
    let mut paths = commit_paths(c, &record)?;
    let audit = c.audit();
    if metadata_if_exists(&audit)?.is_some() {
        if read_text(&audit)?.lines().next() != Some("verdict: accept") {
            bail!("監査が合格していない: {}", audit.display());
        }
        paths.insert(c.relative(&audit)?);
    }
    check_index(c, &paths)?;
    let verified: Fingerprints = serde_json::from_str(
        &read_text(&c.state_dir().join("verified.json")).context("verify が合格していない")?,
    )?;
    let original_paths = commit_paths(c, &record)?;
    if verified != fingerprints(c, &original_paths)? {
        bail!("verify の後に名前の付け直しに関係するファイルが変わった");
    }
    let rows: BTreeMap<String, Row> = serde_json::from_value(record["rows"].clone())?;
    let pairs: Vec<(String, String)> = serde_json::from_value(record["pairs"].clone())?;
    let finals = final_paths(&rows, &pairs);
    crate::manifest::verify(&c.home(), &c.manifest(), &finals.into_keys().collect())?;

    let mut args = vec!["add", "-A", "--"];
    args.extend(paths.iter().map(String::as_str));
    git(&c.base, &args)?;
    check_index(c, &paths)?;
    let message = format!(
        "Rename {} files: 中身を説明する名前・日付の不統一を付け直し ({} files, 中身の参照の修正 {})",
        c.persona,
        pairs.len(),
        record["refs"].as_array().context("参照の記録が不正")?.len()
    );
    // A path-limited commit cannot pick up an unrelated add racing the index check.
    let mut args = vec!["commit", "--only", "-qm", &message, "--"];
    args.extend(paths.iter().map(String::as_str));
    git(&c.base, &args)?;
    save_json(&c.state_dir().join("state.json"), &json!({"stage": "done"}))?;
    fs::remove_file(c.state_dir().join("applied.json"))?;
    Ok(message)
}

// ---- driver -------------------------------------------------------------

struct Driver {
    loc: Locations,
    base: PathBuf,
}
const LIMIT: &str = r"(?i)利用上限|usage limit|rate limit|\b429\b|\bquota\b";
const MAX_REVISIONS: u64 = 2;

fn revisions(state: &Value) -> u64 {
    state["revisions"].as_u64().unwrap_or(0)
}

fn resumable_audit_hold(state: &Value) -> bool {
    state["stage"] == "hold"
        && revisions(state) < MAX_REVISIONS
        && state["reason"]
            .as_str()
            .is_some_and(|reason| reason.starts_with("監査 hold"))
}

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
        if !o.status.success() {
            bail!("codex-job status: {}", String::from_utf8_lossy(&o.stderr));
        }
        Ok(String::from_utf8(o.stdout)?
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
        while self
            .ops()
            .join("jobs/personaCorpus")
            .join(&name)
            .try_exists()?
            || self
                .base
                .join("scratch/rename")
                .join(p)
                .join(&name)
                .try_exists()?
        {
            i += 1;
            name = format!("{short}-{kind}-r{i}");
        }
        let work = self.base.join("scratch/rename").join(p).join(&name);
        fs::create_dir_all(&work)?;
        let context =
            json!({"persona": p, "scratch": work.strip_prefix(&self.base)?.to_string_lossy()});
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
            "--model".into(),
            model.into(),
            "--brief".into(),
            brief.to_string_lossy().into_owned(),
            "--output".into(),
            work.join("out.md").to_string_lossy().into_owned(),
        ]);
        let command = format!(
            "cd {} && {cmd}",
            shell_words::quote(&self.base.to_string_lossy())
        );
        let o = Command::new(self.base.join("ops/bin/codex-job"))
            .args([
                "start",
                "--repo",
                "personaCorpus",
                "--name",
                &name,
                "--model",
                model,
                "--effort",
                "medium",
                "--command",
                &command,
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
    fn state(&self, p: &str) -> Result<Value> {
        match metadata_if_exists(&self.state_path(p))? {
            Some(_) => Ok(serde_json::from_str(&read_text(&self.state_path(p))?)?),
            None => Ok(json!({"stage": "names", "attempts": 0})),
        }
    }
    fn store(&self, p: &str, v: &Value) -> Result<()> {
        save_json(&self.state_path(p), v)
    }
    fn start_revision(&self, p: &str, state: &Value, reason: &str) -> Result<()> {
        self.store(
            p,
            &json!({"stage": "revise", "attempts": 0, "revisions": revisions(state) + 1}),
        )?;
        self.log(p, "revise", reason)
    }
    /// One step for one persona. Returns true when the persona is finished.
    fn step(&self, p: &str, paused: &mut Option<SystemTime>) -> Result<bool> {
        let mut s = self.state(p)?;
        let stage = s["stage"].as_str().context("stage が不正")?.to_string();
        if resumable_audit_hold(&s) {
            self.start_revision(p, &s, s["reason"].as_str().unwrap())?;
            return Ok(false);
        }
        if ["done", "hold"].contains(&stage.as_str()) {
            return Ok(true);
        }
        let ctx = Ctx::new(self.loc.clone(), p)?;
        // A completed apply can be found after an interruption before the driver
        // saved its next stage.
        if stage == "names" && ctx.state_dir().join("applied.json").try_exists()? {
            let refs = apply(&ctx)?;
            self.store(
                p,
                &json!({"stage": if refs.is_empty() {"verify"} else {"fix"}, "attempts": 0}),
            )?;
            return Ok(false);
        }
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
                    let reason = format!("Codex ジョブが正常終了しなかった: {job} ({status})");
                    let attempts = s["attempts"].as_i64().unwrap_or(0);
                    if attempts >= 2 {
                        s["stage"] = json!("hold");
                        s["reason"] = json!(reason);
                        self.store(p, &s)?;
                        self.log(p, "hold", &reason)?;
                        return Ok(true);
                    }
                    s["attempts"] = json!(attempts + 1);
                    s["errors"] = json!(reason);
                }
                self.store(p, &s)?;
                return Ok(false);
            }
            s["ran"] = json!(job);
            self.store(p, &s)?;
        }
        let attempts = s["attempts"].as_i64().unwrap_or(0);
        let revision_count = revisions(&s);
        let next = |stage: &str, attempts: i64| json!({"stage": stage, "attempts": attempts, "revisions": revision_count});
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
        let stopped = crate::autopilot::usage_stopped(&self.base);
        if let Some(reason) = &stopped
            && s["usage_stop"].as_str() != Some(reason)
        {
            s["usage_stop"] = json!(reason);
            self.store(p, &s)?;
            self.log(p, "usage-stop", reason)?;
        }
        let can_launch = stopped.is_none() && paused.is_none_or(|t| SystemTime::now() >= t);
        match stage.as_str() {
            "names" | "revise" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    let kind = if stage == "revise" {
                        "rename-revise"
                    } else {
                        "rename"
                    };
                    s["job"] = json!(self.launch(p, kind, "gpt-6.2-sol", &errors_note(&s))?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                let errors = ctx.validation();
                match errors {
                    Ok(e) if e.is_empty() => {
                        let refs = apply(&ctx)?;
                        self.log(p, "applied", &format!("refs {}", refs.len()))?;
                        s = next(if refs.is_empty() { "verify" } else { "fix" }, 0);
                    }
                    other => {
                        let e = match other {
                            Ok(e) => e.join("\n"),
                            Err(e) => e.to_string(),
                        };
                        if attempts >= 2 {
                            return hold(&mut s, e);
                        }
                        s = next(&stage, attempts + 1);
                        s["errors"] = json!(e);
                    }
                }
                self.store(p, &s)?;
            }
            "fix" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    s["job"] =
                        json!(self.launch(p, "rename-fix", "gpt-6.2-sol", &errors_note(&s))?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                s = next("verify", attempts);
                self.store(p, &s)?;
            }
            "verify" => {
                let errors = verify(&ctx)?;
                if errors.is_empty() {
                    s = next("audit", 0);
                } else if attempts >= 2 {
                    return hold(&mut s, errors.join("\n"));
                } else {
                    s = next("fix", attempts + 1);
                    s["errors"] = json!(errors.join("\n"));
                }
                self.store(p, &s)?;
            }
            "audit" => {
                if s["ran"].is_null() {
                    if !can_launch {
                        return Ok(false);
                    }
                    if metadata_if_exists(&ctx.audit())?.is_some() {
                        fs::remove_file(ctx.audit())?;
                    }
                    s["job"] = json!(self.launch(p, "rename-audit", "gpt-6.2-astra", "")?);
                    self.store(p, &s)?;
                    return Ok(false);
                }
                let audit = match metadata_if_exists(&ctx.audit())? {
                    Some(_) => read_text(&ctx.audit())?,
                    None => String::new(),
                };
                match audit.lines().next() {
                    Some("verdict: accept") => {
                        let message = commit(&ctx)?;
                        self.log(p, "commit", &message)?;
                        return Ok(true);
                    }
                    Some("verdict: hold") => {
                        let reason = format!("監査 hold: progress/{p}/rename.audit.md");
                        if revision_count >= MAX_REVISIONS {
                            return hold(&mut s, reason);
                        }
                        self.start_revision(p, &s, &reason)?;
                    }
                    _ if attempts >= 2 => return hold(&mut s, "監査の書式が不正".into()),
                    _ => {
                        s = next("audit", attempts + 1);
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
    #[arg(id = "persona_names", required = true)]
    personas: Vec<String>,
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
            let e = c.validation()?;
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
            let _l = lock_file(&c.base, &format!("scratch/{}/.lock", c.persona), false)
                .context("同じ persona の batch が進行中")?;
            println!("{}", commit(&c)?);
            Ok(0)
        }
        "run" => {
            let ctx = Ctx::new(o.locations.clone(), &o.personas[0])?;
            ctx.check_locations()?;
            let base = ctx.base;
            // The jobs use bin/rename and repo-relative paths from their briefs.
            for (path, name) in [
                (&ctx.loc.root, "people"),
                (&ctx.loc.personas, "personas"),
                (&ctx.loc.progress, "progress"),
                (&ctx.loc.manifests, "manifests"),
                (&ctx.loc.sources, "sources"),
            ] {
                if *path != base.join(name) {
                    bail!("run はリポジトリの既定の配置が必要: {}", path.display());
                }
            }
            let d = Driver {
                loc: ctx.loc,
                base: base.clone(),
            };
            let names = o
                .personas
                .iter()
                .map(|p| resolve(&d.loc.root, p))
                .collect::<Result<BTreeSet<_>>>()?;
            let mut locks = BTreeMap::<String, File>::new();
            let mut paused = None;
            loop {
                let mut open = 0;
                for p in &names {
                    if !locks.contains_key(p) {
                        let state = d.state(p)?;
                        if state["stage"] == "done"
                            || (state["stage"] == "hold" && !resumable_audit_hold(&state))
                        {
                            continue;
                        }
                        match lock_file(&base, &format!("scratch/{p}/.lock"), false) {
                            Ok(f) => {
                                locks.insert(p.clone(), f);
                            }
                            Err(e)
                                if e.downcast_ref::<std::io::Error>()
                                    .is_some_and(|e| e.kind() == ErrorKind::WouldBlock) =>
                            {
                                open += 1;
                                continue;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    match d.step(p, &mut paused) {
                        Ok(true) => {
                            locks.remove(p);
                        }
                        Ok(false) => open += 1,
                        Err(e) => {
                            d.log(p, "error", &format!("{e:#}"))?;
                            let mut s = d.state(p)?;
                            s["stage"] = json!("hold");
                            s["reason"] = json!(format!("{e:#}"));
                            d.store(p, &s)?;
                            locks.remove(p);
                        }
                    }
                }
                if open == 0 {
                    let mut result = 0;
                    for p in &names {
                        let state = d.state(p)?;
                        if state["stage"] == "hold" {
                            eprintln!(
                                "{p}: 保留: {}",
                                state["reason"].as_str().unwrap_or_default()
                            );
                            result = 1;
                        }
                    }
                    return Ok(result);
                }
                std::thread::sleep(Duration::from_secs(o.interval));
            }
        }
        other => bail!("unknown action: {other}"),
    }
}
pub fn run(args: &[String]) -> i32 {
    let o = match Options::try_parse_from(
        std::iter::once("rename".to_string()).chain(args.iter().cloned()),
    ) {
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
