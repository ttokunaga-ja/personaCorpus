use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
const ROAD: [&str; 7] = [
    "batch",
    "範囲",
    "件数の目安",
    "重点",
    "含めたいもの",
    "避けるもの",
    "状態",
];
const ACTIVE: [&str; 4] = ["running", "stalled", "stdin-wait", "limit"];
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}
#[derive(Subcommand)]
enum Action {
    Start {
        #[arg(long)]
        personas: Option<String>,
        #[arg(long)]
        target: Option<i64>,
    },
    Run {
        #[arg(long)]
        personas: Option<String>,
        #[arg(long)]
        target: Option<i64>,
    },
    Once {
        #[arg(long)]
        personas: Option<String>,
        #[arg(long)]
        target: Option<i64>,
    },
    Status {
        #[arg(long)]
        personas: Option<String>,
        #[arg(long)]
        target: Option<i64>,
    },
    Stop {
        #[arg(long)]
        now: bool,
    },
    /// Answer a question the coordinator left for the user (item as shown in status).
    Answer {
        item: String,
        text: String,
        /// The user approved deleting this many accepted files in the coordinator's next merge.
        #[arg(long)]
        allow_delete: Option<usize>,
    },
}
fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
}
fn epoch() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
fn read(p: &Path) -> Result<String> {
    Ok(fs::read_to_string(p)?)
}
fn save(p: &Path, v: &Value) -> Result<()> {
    fs::create_dir_all(p.parent().unwrap())?;
    let tmp = p.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(v)? + "\n")?;
    fs::rename(tmp, p)?;
    Ok(())
}
fn list(p: &Path) -> Vec<PathBuf> {
    let mut v = fs::read_dir(p)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect::<Vec<_>>();
    v.sort();
    v
}
fn regex(s: &str) -> regex::Regex {
    regex::Regex::new(s).unwrap()
}
fn cells(s: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut last = '\0';
    for c in s.chars() {
        if c == '|' && last != '\\' {
            parts.push(String::new())
        } else {
            parts.last_mut().unwrap().push(c)
        }
        last = c;
    }
    parts
}
#[derive(Clone)]
struct Row {
    v: BTreeMap<String, String>,
    line: usize,
}
impl Row {
    fn get(&self, k: &str) -> &str {
        self.v.get(k).map(String::as_str).unwrap_or("")
    }
}
fn table(p: &Path, columns: &[&str], root: &Path) -> Result<Vec<Row>> {
    if !p.exists() {
        bail!(
            "ファイルなし: {}",
            p.strip_prefix(root).unwrap_or(p).display()
        )
    }
    let mut header: Option<Vec<String>> = None;
    let mut out = Vec::new();
    for (i, l) in read(p)?.lines().enumerate() {
        if !l.trim().starts_with('|') {
            continue;
        }
        let c = cells(l.trim().trim_matches('|'))
            .into_iter()
            .map(|s| s.trim().trim_matches('`').to_string())
            .collect::<Vec<_>>();
        if columns.iter().all(|x| c.iter().any(|v| v == x)) {
            header = Some(c);
            continue;
        }
        let Some(h) = &header else { continue };
        if c.iter().all(|x| regex(r"^:?-+:?$").is_match(x)) {
            continue;
        }
        if c.len() != h.len() {
            bail!("表の列数が不正: {}", p.display())
        }
        out.push(Row {
            v: h.iter().cloned().zip(c).collect(),
            line: i,
        });
    }
    if header.is_none() {
        bail!("表の列が不足: {}", columns.join("、"))
    }
    Ok(out)
}
fn replace_cell(p: &Path, r: &Row, column: &str, value: &str) -> Result<()> {
    let text = read(p)?;
    let mut lines = text.lines().map(str::to_string).collect::<Vec<_>>();
    let header = lines
        .iter()
        .find(|l| l.trim().starts_with('|') && l.contains(column) && l.contains("batch"))
        .context("table header missing")?;
    let h = cells(header.trim().trim_matches('|'))
        .into_iter()
        .map(|s| s.trim().trim_matches('`').to_string())
        .collect::<Vec<_>>();
    let index = h
        .iter()
        .position(|s| s == column)
        .context("column missing")?;
    let mut row = cells(&lines[r.line]);
    row[index + 1] = format!(" {value} ");
    lines[r.line] = row.join("|");
    fs::write(p, lines.join("\n") + "\n")?;
    Ok(())
}
fn lock(root: &Path, name: &str, blocking: bool) -> Result<File> {
    let p = root.join("scratch/.locks").join(format!("{name}.lock"));
    fs::create_dir_all(p.parent().unwrap())?;
    let f = OpenOptions::new().create(true).append(true).open(p)?;
    if blocking {
        FileExt::lock_exclusive(&f)?
    } else {
        FileExt::try_lock_exclusive(&f)?
    }
    Ok(f)
}
fn locked(p: &Path) -> Result<bool> {
    let f = OpenOptions::new().append(true).open(p)?;
    match FileExt::try_lock_exclusive(&f) {
        Ok(()) => Ok(false),
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(true),
        Err(e) => Err(e.into()),
    }
}
fn loop_running(root: &Path) -> Result<bool> {
    match lock(root, "autopilot-loop", false) {
        Ok(_) => Ok(false),
        Err(e)
            if e.downcast_ref::<std::io::Error>()
                .is_some_and(|e| e.kind() == std::io::ErrorKind::WouldBlock) =>
        {
            Ok(true)
        }
        Err(e) => Err(e),
    }
}
fn ledger(root: &Path) -> Result<Vec<BTreeMap<String, String>>> {
    let p = root.join("progress/batches.tsv");
    let mut rows = if p.exists() {
        let mut r = csv::ReaderBuilder::new().delimiter(b'\t').from_path(p)?;
        let h = r.headers()?.clone();
        r.records()
            .map(|r| {
                r.map(|r| {
                    h.iter()
                        .map(str::to_string)
                        .zip(r.iter().map(str::to_string))
                        .collect()
                })
            })
            .collect::<std::result::Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    // Batches finished before the ledger existed are recorded only in their result report.
    rows.extend(reported(root)?);
    Ok(rows)
}
fn write_holds(root: &Path, rows: &[Row]) -> Result<()> {
    let mut text = "# 保留\n\n| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n".to_string();
    for r in rows {
        text.push_str(&format!(
            "| {} |\n",
            ["persona", "batch", "理由", "日時"]
                .iter()
                .map(|k| regex(r"[|\r\n\t]").replace_all(r.get(k), " ").into_owned())
                .collect::<Vec<_>>()
                .join(" | ")
        ))
    }
    fs::create_dir_all(root.join("progress"))?;
    fs::write(root.join("progress/holds.md"), text)?;
    Ok(())
}
/// Coordinator checks (progress/checks.tsv): (persona/batch, 判定) in order.
fn check_rows(root: &Path) -> Result<Vec<(String, String)>> {
    let p = root.join("progress/checks.tsv");
    if !p.exists() {
        return Ok(Vec::new());
    }
    let text = read(&p)?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
    let col = |name: &str| header.iter().position(|h| *h == name);
    let (Some(pi), Some(bi), Some(vi)) = (col("persona"), col("batch"), col("判定")) else {
        bail!("progress/checks.tsv の見出しが不正");
    };
    Ok(lines
        .map(|l| l.split('\t').collect::<Vec<_>>())
        .filter(|c| c.len() > pi.max(bi).max(vi))
        .map(|c| (format!("{}/{}", c[pi], c[bi]), c[vi].to_string()))
        .collect())
}
fn reported(root: &Path) -> Result<Vec<BTreeMap<String, String>>> {
    let mut rows = Vec::new();
    let progress = root.join("progress");
    if !progress.is_dir() {
        return Ok(rows);
    }
    let name = regex(r"^(b\d+)\.result\.md$");
    let state = regex(r"(?m)^状態：(破棄|受入済み)\s*$");
    for persona in fs::read_dir(&progress)? {
        let persona = persona?.path();
        if !persona.is_dir() {
            continue;
        }
        for file in fs::read_dir(&persona)? {
            let file = file?.path();
            let Some(fname) = file.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Some(batch) = name.captures(fname) else {
                continue;
            };
            let text = read(&file)?;
            let Some(found) = state.captures(&text) else {
                continue;
            };
            let status = if &found[1] == "破棄" {
                "discarded"
            } else {
                "accepted"
            };
            rows.push(BTreeMap::from([
                (
                    "persona".to_string(),
                    persona.file_name().unwrap().to_string_lossy().into_owned(),
                ),
                ("batch".to_string(), batch[1].to_string()),
                ("status".to_string(), status.to_string()),
            ]));
        }
    }
    Ok(rows)
}
struct Output {
    code: i32,
    text: String,
}
fn command(root: &Path, args: &[String], check: bool) -> Result<Output> {
    use std::io::{Read, Seek};
    let mut stream = tempfile::tempfile()?;
    let status = Command::new(&args[0])
        .args(&args[1..])
        .current_dir(root)
        .stdout(Stdio::from(stream.try_clone()?))
        .stderr(Stdio::from(stream.try_clone()?))
        .status()?;
    stream.rewind()?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let code = status.code().unwrap_or(-1);
    if check && code != 0 {
        bail!(
            "{}: {}",
            shell_words::join(args),
            text.chars()
                .rev()
                .take(4000)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
        )
    }
    Ok(Output { code, text })
}
fn argv(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}
struct Pilot {
    root: PathBuf,
    home: PathBuf,
    state: Value,
    jobs: BTreeMap<String, String>,
    personas: Option<String>,
    target: i64,
    push_attempted: bool,
    /// Personas that may start new work in this pass (None: all).
    allowed: Option<BTreeSet<String>>,
    /// Set after a coordinator fix changed the tools; the loop re-executes itself.
    restart: bool,
}
impl Pilot {
    fn new(personas: Option<String>, target: i64) -> Result<Self> {
        let root = crate::base();
        let home = root.join("scratch/autopilot");
        let p = home.join("state.json");
        let mut state = if p.exists() {
            serde_json::from_str(&read(&p)?)?
        } else {
            json!({})
        };
        for (k, v) in [
            ("batches", json!({})),
            ("roadmaps", json!({})),
            ("push_failures", json!(0)),
        ] {
            if state.get(k).is_none() {
                state[k] = v
            }
        }
        Ok(Self {
            root,
            home,
            state,
            jobs: BTreeMap::new(),
            personas,
            target,
            push_attempted: false,
            allowed: None,
            restart: false,
        })
    }
    fn store(&self) -> Result<()> {
        save(&self.home.join("state.json"), &self.state)
    }
    fn log(&self, action: &str, p: &str, b: &str, detail: &str) -> Result<()> {
        fs::create_dir_all(&self.home)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.home.join("log.tsv"))?;
        let mut w = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .terminator(csv::Terminator::Any(b'\n'))
            .from_writer(file);
        w.write_record([&now(), action, p, b, detail])?;
        w.flush()?;
        Ok(())
    }
    fn cmd(&self, args: &[&str], check: bool) -> Result<Output> {
        command(&self.root, &argv(args), check)
    }
    fn job(&self, args: &[String]) -> Result<Output> {
        let mut a = vec![
            self.root
                .join("ops/bin/codex-job")
                .to_string_lossy()
                .into_owned(),
        ];
        a.extend_from_slice(args);
        command(&self.root, &a, true)
    }
    fn refresh(&mut self) -> Result<()> {
        let o = self.job(&argv(&["status", "--repo", "personaCorpus"]))?;
        self.jobs = o
            .text
            .lines()
            .filter_map(|l| {
                let c = l.split('\t').collect::<Vec<_>>();
                if c.len() >= 3 && c[0] == "personaCorpus" {
                    Some((c[1].into(), c[2].into()))
                } else {
                    None
                }
            })
            .collect();
        Ok(())
    }
    fn unique(&self, base: &str) -> String {
        let home = std::env::var_os("AGENT_OPS_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.root.join("ops"));
        let mut names = self.jobs.keys().cloned().collect::<BTreeSet<_>>();
        for p in list(&home.join("jobs/personaCorpus")) {
            names.insert(p.file_name().unwrap().to_string_lossy().into_owned());
        }
        let mut n = base.to_string();
        let mut i = 1;
        while names.contains(&n) {
            i += 1;
            n = format!("{base}-r{i}")
        }
        n
    }
    /// A Codex job that ended on the usage limit (its log says so).
    fn limited(&self, job: &str) -> bool {
        let home = std::env::var_os("AGENT_OPS_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.root.join("ops"));
        let log = home.join("jobs/personaCorpus").join(job).join("log");
        let text = fs::read(&log).unwrap_or_default();
        let tail = String::from_utf8_lossy(&text[text.len().saturating_sub(8192)..]).into_owned();
        regex(r"(?i)利用上限|usage limit|rate limit|\b429\b|\bquota\b").is_match(&tail)
    }
    /// Stop launching Codex for 30 minutes after a job hit the usage limit.
    fn note_limit(&mut self, job: &str) -> Result<()> {
        if !self.paused() {
            self.log(
                "limit",
                "",
                "",
                &format!("{job}: Codex の利用上限。30 分待つ"),
            )?;
        }
        self.state["limit_until"] = json!(epoch() + 1800.);
        self.store()
    }
    fn paused(&self) -> bool {
        epoch() < self.state["limit_until"].as_f64().unwrap_or(0.)
    }
    fn names(&self) -> Result<Vec<String>> {
        let mut names = list(&self.root.join("personas"))
            .into_iter()
            .filter(|p| p.extension().is_some_and(|e| e == "md"))
            .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
            .filter(|s| regex(r"^p[0-9][0-9]-.+$").is_match(s))
            .collect::<Vec<_>>();
        if let Some(p) = &self.personas {
            let req = p.split(',').collect::<Vec<_>>();
            names.retain(|n| {
                req.contains(&n.as_str()) || req.contains(&n.split('-').next().unwrap())
            });
            if req.iter().any(|r| {
                !names
                    .iter()
                    .any(|n| n == r || n.split('-').next() == Some(r))
            }) {
                bail!("未知の persona: {p}")
            }
        }
        Ok(names)
    }
    fn holds(&self) -> Result<Vec<Row>> {
        let p = self.root.join("progress/holds.md");
        if p.exists() {
            table(&p, &["persona", "batch", "理由", "日時"], &self.root)
        } else {
            Ok(Vec::new())
        }
    }
    fn held(&self, p: &str, b: &str) -> Result<bool> {
        Ok(self
            .holds()?
            .iter()
            .any(|r| r.get("persona") == p && r.get("batch") == b))
    }
    fn hold(&self, p: &str, b: &str, reason: &str) -> Result<()> {
        let _lock = lock(&self.root, "records", true)?;
        let mut rows = self.holds()?;
        if let Some(old) = rows
            .iter_mut()
            .find(|r| r.get("persona") == p && r.get("batch") == b)
        {
            if old.get("理由") == reason {
                return Ok(());
            }
            old.v.insert("理由".into(), reason.into());
            old.v.insert("日時".into(), now());
        } else {
            rows.push(Row {
                v: [
                    ("persona", p.to_string()),
                    ("batch", b.to_string()),
                    ("理由", reason.to_string()),
                    ("日時", now()),
                ]
                .into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect(),
                line: 0,
            })
        }
        write_holds(&self.root, &rows)?;
        drop(_lock);
        self.log("hold", p, b, reason)
    }
    fn stops(&self) -> Result<Vec<String>> {
        let mut reasons = Vec::new();
        if self.home.join("stop").exists() {
            reasons.push("stop".into())
        }
        let threshold = std::env::var("AUTOPILOT_MIN_FREE_BYTES")
            .unwrap_or("30000000000".into())
            .parse::<f64>()?;
        if (fs2::available_space(&self.root)? as f64) < threshold {
            reasons.push("空き容量30GB未満".into())
        }
        if self
            .holds()?
            .iter()
            .filter(|r| r.get("batch").starts_with('b'))
            .count()
            >= 5
        {
            reasons.push("保留5件以上".into())
        }
        if self.state["push_failures"].as_i64().unwrap_or(0) >= 3 {
            reasons.push("push連続失敗".into())
        }
        if self.paused() {
            reasons.push("Codex の利用上限（30 分ごとに試す）".into())
        }
        Ok(reasons)
    }
    fn count(&self, p: &str) -> Result<usize> {
        let path = self.root.join("manifests").join(format!("{p}.tsv"));
        if !path.exists() {
            return Ok(0);
        }
        Ok(read(&path)?
            .lines()
            .filter(|l| !l.trim().is_empty())
            .count())
    }
    fn roadmap(&self, p: &str) -> Result<Vec<Row>> {
        let rows = table(
            &self.root.join("progress").join(p).join("roadmap.md"),
            &ROAD,
            &self.root,
        )?;
        if rows.is_empty()
            || rows.iter().any(|r| {
                let minimum = if r.get("範囲").starts_with("再制作：") {
                    1
                } else {
                    30
                };
                r.get("範囲").is_empty()
                    || !r.get("件数の目安").chars().all(|c| c.is_ascii_digit())
                    || !r
                        .get("件数の目安")
                        .parse::<i64>()
                        .is_ok_and(|n| (minimum..=40).contains(&n))
            })
        {
            bail!("roadmap: 範囲が空または件数が範囲外（通常30〜40、再制作1〜40）")
        }
        let open: i64 = rows
            .iter()
            .filter(|r| r.get("状態").is_empty())
            .map(|r| r.get("件数の目安").parse::<i64>().unwrap())
            .sum();
        let need = self.goal(p)? - self.count(p)? as i64;
        if need > 0 && open < need && self.active(p)?.is_empty() {
            bail!("roadmap: 未着手の行の件数が次の区切りまでに足りない")
        }
        Ok(rows)
    }
    fn limits(&self) -> Result<Value> {
        let cfg = self.root.join("config/limits.json");
        Ok(if cfg.exists() {
            serde_json::from_str(&read(&cfg)?)?
        } else {
            json!({})
        })
    }
    /// The persona's whole scale: "全体で N 件" in its definition's 規模の目安,
    /// capped by --target.
    fn final_target(&self, p: &str) -> Result<i64> {
        let text = read(&self.root.join("personas").join(format!("{p}.md")))?;
        let section = text
            .split("\n## ")
            .find(|s| s.starts_with("規模の目安"))
            .unwrap_or("");
        let scale = regex(r"全体で\s*([0-9][0-9,]*)\s*件")
            .captures(section)
            .and_then(|c| c[1].replace(',', "").parse::<i64>().ok())
            .unwrap_or(self.target);
        Ok(self.target.min(scale))
    }
    /// The persona's current milestone: the next multiple of `milestone` above the
    /// unfinished persona with the fewest accepted files, capped by the persona's
    /// whole scale. All personas reach one milestone before any goes further.
    fn goal(&self, p: &str) -> Result<i64> {
        let step = self.limits()?["milestone"].as_i64().unwrap_or(300).max(1);
        let mut least = None::<i64>;
        for q in self.names()? {
            let n = self.count(&q)? as i64;
            if n < self.final_target(&q)? {
                least = Some(least.map_or(n, |l| l.min(n)));
            }
        }
        let whole = self.final_target(p)?;
        Ok(least.map_or(whole, |l| whole.min(step * (l / step + 1))))
    }
    /// Batches of the persona that are neither accepted nor discarded.
    fn active(&self, p: &str) -> Result<BTreeSet<String>> {
        let mut active = self.state["batches"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, v)| {
                k.starts_with(&format!("{p}/")) && !v["accepted"].as_bool().unwrap_or(false)
            })
            .map(|(k, _)| k.split('/').next_back().unwrap().to_string())
            .collect::<BTreeSet<_>>();
        for (b, path) in self.states(p) {
            let v: Value = serde_json::from_str(&read(&path)?)?;
            if !["accepted", "discarded"].contains(&v["status"].as_str().unwrap_or("")) {
                active.insert(b);
            }
        }
        let ledger = ledger(&self.root)?;
        active.retain(|b| {
            !ledger.iter().any(|r| {
                r.get("persona").map(String::as_str) == Some(p)
                    && r.get("batch") == Some(b)
                    && r.get("status")
                        .is_some_and(|s| ["accepted", "discarded"].contains(&s.as_str()))
            })
        });
        let pat = regex(&format!(
            r"^{}-(b\d+)(?:-r\d+)?$",
            regex::escape(p.split('-').next().unwrap())
        ));
        for (n, s) in &self.jobs {
            if ACTIVE.contains(&s.as_str()) {
                if let Some(c) = pat.captures(n) {
                    active.insert(c[1].into());
                }
            }
        }
        Ok(active)
    }
    /// A roadmap rewrite must keep every row that already has a state.
    fn kept(&self, name: &str, rows: &[Row]) -> Result<()> {
        let prev = self.home.join("jobs").join(name).join("previous.md");
        if name.is_empty() || !prev.exists() {
            return Ok(());
        }
        let lost: Vec<_> = table(&prev, &ROAD, &self.root)
            .unwrap_or_default()
            .iter()
            .filter(|o| !o.get("状態").is_empty())
            .filter(|o| {
                !rows
                    .iter()
                    .any(|r| r.get("範囲") == o.get("範囲") && r.get("状態") == o.get("状態"))
            })
            .map(|o| o.get("範囲").to_string())
            .collect();
        if !lost.is_empty() {
            bail!("roadmap: 着手済みの行が消えた: {}", lost.join(", "))
        }
        Ok(())
    }
    fn rows(&self, p: &str, b: &str) -> Result<Vec<Row>> {
        table(
            &self.root.join("progress").join(p).join(format!("{b}.md")),
            &["パス", "内容"],
            &self.root,
        )
    }
    fn hashes(&self, p: &str, b: &str) -> Result<Value> {
        let mut hashes = json!({});
        for row in self.rows(p, b)? {
            let rel = row.get("パス");
            if rel.is_empty()
                || Path::new(rel).is_absolute()
                || Path::new(rel)
                    .components()
                    .any(|c| c == std::path::Component::ParentDir)
                || rel.contains('\\')
            {
                bail!("計画のパスが不正")
            }
            for (folder, suffix) in [("people", ""), ("sources", ".md")] {
                let base = self.root.join(folder).join(p);
                let file = base.join(format!("{rel}{suffix}"));
                if file.is_symlink()
                    || !crate::manifest::resolve_path(&file)?
                        .starts_with(crate::manifest::resolve_path(&base)?)
                {
                    bail!("計画のパスがリンク")
                }
                let key = file
                    .strip_prefix(&self.root)?
                    .to_string_lossy()
                    .into_owned();
                hashes[&key] = if file.exists() {
                    json!(crate::manifest::digest(&file)?)
                } else {
                    Value::Null
                };
            }
        }
        Ok(hashes)
    }
    fn latest(&self, p: &str, b: &str) -> Option<String> {
        let base = format!("{}-{b}", p.split('-').next().unwrap());
        let pat = regex(&format!(r"^{}(?:-r(\d+))?$", regex::escape(&base)));
        self.jobs
            .keys()
            .filter_map(|n| {
                pat.captures(n).map(|c| {
                    (
                        c.get(1)
                            .map(|v| v.as_str().parse::<i64>().unwrap_or(1))
                            .unwrap_or(1),
                        n.clone(),
                    )
                })
            })
            .max()
            .map(|(_, n)| n)
    }
    fn batch_cmd(&mut self, action: &str, p: &str, b: &str, flags: &[String]) -> Result<()> {
        let mut a = vec![
            self.root.join("bin/batch").to_string_lossy().into_owned(),
            action.into(),
            p.into(),
            b.into(),
        ];
        a.extend_from_slice(flags);
        command(&self.root, &a, true)?;
        self.log(action, p, b, &flags.join(" "))?;
        if let Err(e) = self.refresh() {
            self.log("status-error", p, b, &e.to_string())?
        }
        Ok(())
    }
    // A coordinator may resume a held batch after its last audit. The journal
    // records successful launches even when the coordinator just merged a fix.
    fn resumed_after_audit(&self, p: &str, b: &str, job: &str) -> Result<bool> {
        let path = self.home.join("log.tsv");
        if !path.exists() {
            return Ok(false);
        }
        let mut seen_audit = false;
        let mut resumed = false;
        let mut log = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path(path)?;
        for row in log.records() {
            let row = row?;
            if row.get(2) != Some(p) || row.get(3) != Some(b) {
                continue;
            }
            match row.get(1) {
                Some("audit") => {
                    seen_audit = row.get(4) == Some(job);
                    resumed = false;
                }
                Some("resume") if seen_audit => resumed = true,
                _ => (),
            }
        }
        Ok(resumed)
    }
    fn set_record(&mut self, p: &str, b: &str, kind: &str, record: Value) {
        if kind == "roadmap" {
            self.state["roadmaps"][p] = record
        } else {
            self.state["batches"][format!("{p}/{b}")]["audit"] = record;
        }
    }
    fn launch(&mut self, p: &str, b: &str, kind: &str, mut rec: Value) -> Result<Value> {
        let name = self.unique(&format!(
            "{}{}{kind}",
            p.split('-').next().unwrap(),
            if b.is_empty() {
                "-".to_string()
            } else {
                format!("-{b}-")
            }
        ));
        let work = self.home.join("jobs").join(&name);
        fs::create_dir_all(&work)?;
        let output = format!(
            "progress/{p}/{}.md",
            if b.is_empty() {
                "roadmap".to_string()
            } else {
                format!("{b}.audit")
            }
        );
        let context = json!({"persona":p,"batch":b,"target":self.goal(p)?,"final_target":self.final_target(p)?,"accepted":self.count(p)?,"scratch":work.strip_prefix(&self.root)?.to_string_lossy(),"output":output});
        let brief = work.join("brief.md");
        fs::write(
            &brief,
            read(&self.root.join("prompts").join(format!("{kind}.md")))?
                + "\n\nAUTOPILOT_CONTEXT\n"
                + &serde_json::to_string(&context)?
                + "\n",
        )?;
        let output = self.root.join(&output);
        if output.exists() {
            fs::copy(&output, work.join("previous.md"))?;
            // An audit must be written afresh; a roadmap is extended in place.
            if kind == "audit" {
                fs::remove_file(&output)?;
            }
        }
        rec["job"] = json!(name);
        rec["attempts"] = json!(rec["attempts"].as_i64().unwrap_or(0) + 1);
        if kind == "audit" {
            rec["hashes"] = self.hashes(p, b)?
        }
        self.set_record(p, b, kind, rec.clone());
        self.store()?;
        let cmd = shell_words::join([
            self.root.join("bin/batch").to_string_lossy().into_owned(),
            "codex-once".into(),
            "--brief".into(),
            brief.to_string_lossy().into_owned(),
            "--output".into(),
            work.join("out.md").to_string_lossy().into_owned(),
        ]);
        let command = format!(
            "cd {} && {cmd}",
            shell_words::quote(&self.root.to_string_lossy())
        );
        self.job(&argv(&[
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            &name,
            "--model",
            "gpt-6.2-astra",
            "--effort",
            "medium",
            "--command",
            &command,
        ]))?;
        self.jobs.insert(name.clone(), "running".into());
        self.log(kind, p, b, &name)?;
        Ok(rec)
    }
    fn ensure_roadmap(&mut self, p: &str) -> Result<Option<Vec<Row>>> {
        let mut rec = self.state["roadmaps"].get(p).cloned().unwrap_or(json!({}));
        let name = rec["job"].as_str().unwrap_or("").to_string();
        let name = name.as_str();
        if self
            .jobs
            .get(name)
            .is_some_and(|s| ACTIVE.contains(&s.as_str()))
        {
            return Ok(None);
        }
        let path = self.root.join("progress").join(p).join("roadmap.md");
        if let Some(line) = read(&path)
            .unwrap_or_default()
            .lines()
            .find(|l| l.contains("規模の見直し："))
        {
            if !self.held(p, "roadmap")? {
                self.hold(p, "roadmap", line.trim())?;
            }
            return Ok(None);
        }
        let checked = if !name.is_empty() && self.jobs.get(name).map(String::as_str) != Some("done")
        {
            Err(anyhow::anyhow!("roadmap ジョブが正常終了していない"))
        } else if rec["kept"].as_str() == Some(name) {
            self.roadmap(p)
        } else {
            self.roadmap(p).and_then(|rows| {
                self.kept(name, &rows)?;
                Ok(rows)
            })
        };
        match checked {
            Ok(rows) => {
                rec["valid"] = json!(true);
                rec["kept"] = json!(name);
                rec["attempts"] = json!(0);
                self.state["roadmaps"][p] = rec;
                Ok(Some(rows))
            }
            Err(e) => {
                if !self.held(p, "roadmap")? {
                    if rec["attempts"].as_i64().unwrap_or(0) >= 2 {
                        self.hold(p, "roadmap", &e.to_string())?;
                    } else {
                        self.launch(p, "", "roadmap", rec)?;
                    }
                }
                Ok(None)
            }
        }
    }
    fn audit(&self, p: &str, b: &str) -> Result<(String, Vec<String>)> {
        let path = self
            .root
            .join("progress")
            .join(p)
            .join(format!("{b}.audit.md"));
        let text = read(&path)?;
        let lines = text.lines().collect::<Vec<_>>();
        if lines.len() < 2
            || !["verdict: accept", "verdict: hold"].contains(&lines[0])
            || !lines[1].starts_with("drop: ")
        {
            bail!("監査の先頭の書式が不正")
        }
        let drops: Vec<String> = serde_json::from_str(&lines[1][6..])
            .map_err(|_| anyhow::anyhow!("監査のdropが不正"))?;
        let drop_set = drops.iter().cloned().collect::<BTreeSet<_>>();
        if drop_set.len() != drops.len() {
            bail!("監査のdropが不正")
        }
        let rows = self.rows(p, b)?;
        let paths = rows
            .iter()
            .map(|r| r.get("パス").to_string())
            .collect::<BTreeSet<_>>();
        if !drop_set.is_subset(&paths) || lines[0] == "verdict: accept" && drops.len() > 1 {
            bail!("監査のdropが計画外またはacceptで2件以上")
        }
        let inspected = table(&path, &["パス", "判定", "理由"], &self.root)?;
        let seen = inspected
            .iter()
            .map(|r| r.get("パス").to_string())
            .collect::<BTreeSet<_>>();
        let available = rows
            .iter()
            .filter(|r| {
                !r.get("内容").starts_with("破棄：") && !r.get("内容").starts_with("成形待ち：")
            })
            .map(|r| r.get("パス").to_string())
            .collect::<BTreeSet<_>>();
        if inspected.is_empty() || seen.len() != inspected.len() || !seen.is_subset(&available) {
            bail!("監査の見たファイル一覧が不正")
        }
        if inspected
            .iter()
            .any(|r| !["合格", "破棄"].contains(&r.get("判定")) || r.get("理由").is_empty())
        {
            bail!("監査の判定・理由が不正")
        }
        let formats = |set: &BTreeSet<String>| {
            set.iter()
                .map(|s| {
                    Path::new(s)
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_lowercase()
                })
                .collect::<BTreeSet<_>>()
        };
        if seen.len() < available.len().min(8.max((rows.len() + 4) / 5))
            || !formats(&available).is_subset(&formats(&seen))
        {
            bail!("監査の件数・形式が不足")
        }
        let rejects = inspected
            .iter()
            .filter(|r| r.get("判定") == "破棄")
            .map(|r| r.get("パス").to_string())
            .collect::<BTreeSet<_>>();
        if lines[0] == "verdict: accept" && (rejects.len() > 1 || rejects != drop_set) {
            bail!("監査の判定とdropが不一致")
        }
        Ok((lines[0][9..].into(), drops))
    }
    fn update_roadmap(&self, p: &str, b: &str, status: &str) -> Result<()> {
        let path = self.root.join("progress").join(p).join("roadmap.md");
        if !path.exists() {
            return Ok(());
        }
        let pat = regex(&format!(r"(?:^|\W){}(?:$|\W)", regex::escape(b)));
        for row in table(&path, &ROAD, &self.root)? {
            if pat.is_match(row.get("状態")) {
                replace_cell(&path, &row, "状態", &format!("{status} {b}"))?;
                return Ok(());
            }
        }
        Ok(())
    }
    fn process_batch(&mut self, p: &str, b: &str, state: &Value) -> Result<()> {
        let key = format!("{p}/{b}");
        if self.state["batches"].get(&key).is_none() {
            self.state["batches"][&key] = json!({})
        }
        let mut rec = self.state["batches"][&key].clone();
        if self.held(p, b)?
            || ledger(&self.root)?.iter().any(|r| {
                r.get("persona").map(String::as_str) == Some(p)
                    && r.get("batch").map(String::as_str) == Some(b)
                    && r.get("status")
                        .is_some_and(|s| ["accepted", "discarded"].contains(&s.as_str()))
            })
        {
            return Ok(());
        }
        let latest = self.latest(p, b);
        let job_state = latest
            .as_ref()
            .and_then(|n| self.jobs.get(n))
            .map(String::as_str)
            .unwrap_or("");
        let runtime = self
            .root
            .join("scratch")
            .join(p)
            .join(".runtime-locks")
            .join(format!("{b}.lock"));
        if (runtime.exists() && locked(&runtime)?) || ACTIVE.contains(&job_state) {
            return Ok(());
        }
        let status = state["status"].as_str().unwrap_or("");
        if job_state == "failed" || ["failed", "running"].contains(&status) {
            let reason = format!(
                "{} {}",
                state["reason"].as_str().unwrap_or(""),
                state["failure_tail"].as_str().unwrap_or("")
            );
            // bin/batch records a usage-limit failure as the reason; the tail may be left
            // over from an earlier Codex run and must not hide a later plan or check failure.
            let limited = regex(r"(?i)利用上限|usage limit|rate limit|\b429\b|\bquota\b")
                .is_match(state["reason"].as_str().unwrap_or(""));
            if limited {
                if rec.get("limit_since").is_none() {
                    rec["limit_since"] = json!(epoch());
                    self.state["batches"][&key] = rec.clone()
                }
                if epoch() - rec["limit_since"].as_f64().unwrap_or(0.) < 1800. {
                    return Ok(());
                }
            } else if rec["resumes"].as_i64().unwrap_or(0) >= 1 {
                self.hold(p, b, &format!("resume 後に再失敗: {reason}"))?;
                return Ok(());
            }
            self.batch_cmd("resume", p, b, &[])?;
            if limited {
                rec["limit_since"] = json!(epoch())
            } else {
                rec["resumes"] = json!(rec["resumes"].as_i64().unwrap_or(0) + 1)
            }
            self.state["batches"][&key] = rec;
            return self.store();
        }
        if status != "done" {
            return Ok(());
        }
        let rows = self.rows(p, b)?;
        let mut rejected = state["draft_failures"]
            .as_object()
            .map(|m| m.keys().cloned().collect::<BTreeSet<_>>())
            .unwrap_or_default();
        let findings = self.root.join("progress/findings.tsv");
        if findings.exists() {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .from_path(findings)?;
            let h = reader.headers()?.clone();
            for row in reader.records() {
                let row = row?;
                let get = |k: &str| {
                    h.iter()
                        .position(|s| s == k)
                        .and_then(|i| row.get(i))
                        .unwrap_or("")
                };
                if get("persona") == p
                    && get("batch") == b
                    && ["draft-check", "draft-review"].contains(&get("stage"))
                {
                    rejected.insert(get("path").into());
                }
            }
        }
        let discarded = rows
            .iter()
            .filter(|r| {
                r.get("内容").starts_with("破棄：")
                    && (rejected.contains(r.get("パス"))
                        || r.get("内容").starts_with("破棄：先に要る原稿が不合格"))
            })
            .count();
        let redo = if discarded >= 3 && rec["redrafts"].as_i64().unwrap_or(0) < 1 {
            Some(("--redraft", "redrafts", 1))
        } else if rows.iter().any(|r| r.get("内容").starts_with("成形待ち："))
            && rec["rebuilds"].as_i64().unwrap_or(0) < 2
        {
            Some(("--rebuild", "rebuilds", 2))
        } else {
            None
        };
        if let Some((flag, k, max)) = redo {
            self.batch_cmd("resume", p, b, &[flag.into()])?;
            rec[k] = json!(max.min(rec[k].as_i64().unwrap_or(0) + 1));
            rec.as_object_mut().unwrap().remove("audit");
            self.state["batches"][&key] = rec;
            return self.store();
        }
        let audit = rec.get("audit").cloned().unwrap_or(json!({}));
        let name_owned = audit["job"].as_str().unwrap_or("").to_string();
        let name = name_owned.as_str();
        if self
            .jobs
            .get(name)
            .is_some_and(|s| ACTIVE.contains(&s.as_str()))
        {
            return Ok(());
        }
        if name.is_empty() || self.resumed_after_audit(p, b, name)? {
            if !self.paused() {
                self.launch(
                    p,
                    b,
                    "audit",
                    if name.is_empty() { audit } else { json!({}) },
                )?;
            }
            return Ok(());
        }
        if audit.get("hashes") != Some(&self.hashes(p, b)?) {
            return self.hold(p, b, "監査前後の sha256 が不一致");
        }
        let result = if self.jobs.get(name).map(String::as_str) != Some("done") {
            Err(anyhow::anyhow!("監査ジョブが正常終了していない"))
        } else {
            self.audit(p, b)
        };
        let (verdict, drops) = match result {
            Ok(v) => v,
            Err(e) => {
                let mut audit = audit;
                if self.jobs.get(name).map(String::as_str) == Some("failed") && self.limited(name) {
                    self.note_limit(name)?;
                    audit.as_object_mut().unwrap().remove("job");
                    audit["attempts"] = json!((audit["attempts"].as_i64().unwrap_or(1) - 1).max(0));
                    rec["audit"] = audit;
                    self.state["batches"][&key] = rec;
                    return self.store();
                }
                if self.paused() {
                    return Ok(());
                }
                if audit["attempts"].as_i64().unwrap_or(0) >= 2 {
                    self.hold(p, b, &format!("監査の書式・実行エラー: {e}"))?;
                } else {
                    self.launch(p, b, "audit", audit)?;
                }
                return Ok(());
            }
        };
        if verdict == "hold" {
            return self.hold(p, b, &format!("監査 hold: progress/{p}/{b}.audit.md"));
        }
        if self.state["push_failures"].as_i64().unwrap_or(0) >= 3 {
            return Ok(());
        }
        self.state["push_pending"] = json!(true);
        self.store()?;
        self.update_roadmap(p, b, "受入済み")?;
        let flags = if drops.is_empty() {
            Vec::new()
        } else {
            std::iter::once("--drop".into()).chain(drops).collect()
        };
        if let Err(e) = self.batch_cmd("accept", p, b, &flags) {
            // Acceptance is deterministic; retrying the same command cannot succeed.
            self.update_roadmap(p, b, "進行中")?;
            self.state["push_pending"] = json!(false);
            self.store()?;
            let reason: String = e
                .to_string()
                .replace('|', "／")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            return self.hold(
                p,
                b,
                &format!(
                    "受入に失敗: {}",
                    reason.chars().take(300).collect::<String>()
                ),
            );
        }
        self.state["batches"][&key]["accepted"] = json!(true);
        self.state["push_pending"] = json!(true);
        self.store()?;
        self.push()
    }
    fn push(&mut self) -> Result<()> {
        if !self.state["push_pending"].as_bool().unwrap_or(false) || self.push_attempted {
            return Ok(());
        }
        self.push_attempted = true;
        let result = {
            let _lock = lock(&self.root, "records", true)?;
            self.cmd(&["git", "push", "origin", "main"], false)?
        };
        if result.code != 0 {
            self.state["push_failures"] =
                json!(self.state["push_failures"].as_i64().unwrap_or(0) + 1);
            self.log(
                "push-failed",
                "",
                "",
                &result
                    .text
                    .chars()
                    .rev()
                    .take(2000)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>(),
            )?;
            if self.state["push_failures"].as_i64().unwrap_or(0) >= 3 {
                self.hold("全体","push","push が3回連続で失敗。新しい受入を停止。統括がリモートと未送信commitを確認する")?
            }
        } else {
            self.state["push_failures"] = json!(0);
            self.state["push_pending"] = json!(false);
            if self.held("全体", "push")? {
                let _lock = lock(&self.root, "records", true)?;
                let p = self.root.join("progress/holds.md");
                fs::write(
                    &p,
                    read(&p)?
                        .lines()
                        .filter(|l| !l.starts_with("| 全体 | push |"))
                        .collect::<Vec<_>>()
                        .join("\n")
                        + "\n",
                )?;
            }
            self.log("push", "", "", "")?;
        }
        self.store()
    }
    fn commit_holds(&mut self) -> Result<()> {
        let path = "progress/holds.md";
        if !self.root.join(path).exists() {
            return Ok(());
        }
        {
            let _lock = lock(&self.root, "records", true)?;
            if self
                .cmd(&["git", "diff", "--cached", "--name-only"], true)?
                .text
                .lines()
                .any(|p| p != path)
            {
                self.log("holds-uncommitted", "", "", "無関係なstageがある")?;
                return Ok(());
            }
            if self
                .cmd(&["git", "status", "--porcelain", "--", path], true)?
                .text
                .is_empty()
            {
                return Ok(());
            }
            self.state["push_pending"] = json!(true);
            self.store()?;
            self.cmd(&["git", "add", "--", path], true)?;
            self.cmd(
                &["git", "commit", "-m", "Update autopilot holds", "--", path],
                true,
            )?;
        }
        self.push()
    }
    fn states(&self, p: &str) -> Vec<(String, PathBuf)> {
        list(&self.root.join("scratch").join(p))
            .into_iter()
            .filter(|x| {
                x.file_name().unwrap().to_string_lossy().starts_with('b')
                    && x.join("state.json").exists()
            })
            .map(|x| {
                (
                    x.file_name().unwrap().to_string_lossy().into_owned(),
                    x.join("state.json"),
                )
            })
            .collect()
    }
    fn new_batch(&mut self, p: &str, rows: &[Row]) -> Result<()> {
        if self.count(p)? >= self.goal(p)? as usize || self.held(p, "roadmap")? {
            return Ok(());
        }
        let active = self.active(p)?;
        let Some(row) = rows.iter().find(|r| r.get("状態").is_empty()) else {
            return Ok(());
        };
        if !active.is_empty() && !row.v.values().any(|v| v.contains("並行可")) {
            return Ok(());
        }
        if !active.is_empty() {
            let config = self.limits()?;
            let max = std::env::var("CORPUS_MAX_BATCHES_PER_PERSONA")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(config["max_batches_per_persona"].as_u64().unwrap_or(3) as usize);
            if active.len() >= max
                || !ledger(&self.root)?.iter().any(|r| {
                    r.get("persona").map(String::as_str) == Some(p)
                        && r.get("status").map(String::as_str) == Some("accepted")
                })
            {
                return Ok(());
            }
        }
        let folder = self.root.join("progress").join(p);
        let mut maximum = 0;
        let pat = regex(r"^b(\d+)(?:\.|$)");
        for x in list(&folder) {
            if let Some(c) = pat.captures(&x.file_name().unwrap().to_string_lossy()) {
                maximum = maximum.max(c[1].parse::<u64>()?)
            }
        }
        for b in &active {
            maximum = maximum.max(b[1..].parse::<u64>()?)
        }
        let b = format!("b{:03}", maximum + 1);
        let request = format!(
            "# {p} {b} の依頼\n\n{}",
            ROAD[1..6]
                .iter()
                .map(|k| format!("- {k}：{}\n", row.get(k)))
                .collect::<String>()
        );
        fs::write(folder.join(format!("{b}.request.md")), request)?;
        let key = format!("{p}/{b}");
        if self.state["batches"].get(&key).is_none() {
            self.state["batches"][&key] = json!({})
        }
        self.state["batches"][&key]["launch_pending"] = json!(true);
        replace_cell(
            &folder.join("roadmap.md"),
            row,
            "状態",
            &format!("進行中 {b}"),
        )?;
        self.store()?;
        self.batch_cmd("start", p, &b, &[])?;
        self.state["batches"][&key]
            .as_object_mut()
            .unwrap()
            .remove("launch_pending");
        self.store()
    }
    fn once(&mut self) -> Result<()> {
        if self
            .cmd(&["git", "branch", "--show-current"], true)?
            .text
            .trim()
            != "main"
        {
            bail!("自動運転は main で実行する")
        }
        self.state["configuration"] = json!({"target":self.target,"personas":self.personas});
        self.store()?;
        self.push_attempted = false;
        self.push()?;
        self.allowed = self.admitted()?;
        for p in self.names()? {
            if self.home.join("stop-now").exists() {
                return Ok(());
            }
            if let Err(e) = self.once_persona(&p) {
                self.log("error", &p, "", &e.to_string())?;
            }
            self.store()?;
        }
        self.commit_holds()?;
        if let Err(e) = self.coordinate() {
            self.log("error", "", "", &format!("統括の起動: {e}"))?;
        }
        Ok(())
    }
    /// Personas with work in progress, plus the waiting personas with the most
    /// accepted files, up to `max_active_personas` (absent: no cap).
    fn admitted(&self) -> Result<Option<BTreeSet<String>>> {
        let Some(max) = self.limits()?["max_active_personas"].as_u64() else {
            return Ok(None);
        };
        let mut working = BTreeSet::new();
        let mut waiting = Vec::new();
        for p in self.names()? {
            let road = self.state["roadmaps"][&p]["job"].as_str().unwrap_or("");
            let n = self.count(&p)?;
            if !self.active(&p)?.is_empty()
                || self
                    .jobs
                    .get(road)
                    .is_some_and(|s| ACTIVE.contains(&s.as_str()))
            {
                working.insert(p);
            } else if (n as i64) < self.goal(&p)? {
                waiting.push((std::cmp::Reverse(n), p));
            }
        }
        waiting.sort();
        let room = (max as usize).saturating_sub(working.len());
        working.extend(waiting.into_iter().take(room).map(|(_, p)| p));
        Ok(Some(working))
    }
    fn once_persona(&mut self, p: &str) -> Result<()> {
        let paths = self.states(p).into_iter().collect::<BTreeMap<_, _>>();
        let mut batches = paths.keys().cloned().collect::<BTreeSet<_>>();
        for key in self.state["batches"].as_object().unwrap().keys() {
            if let Some(b) = key.strip_prefix(&format!("{p}/")) {
                batches.insert(b.into());
            }
        }
        let pat = regex(&format!(
            r"^{}-(b\d+)(?:-r\d+)?$",
            regex::escape(p.split('-').next().unwrap())
        ));
        for name in self.jobs.keys() {
            if let Some(c) = pat.captures(name) {
                batches.insert(c[1].into());
            }
        }
        for b in batches {
            if self.home.join("stop-now").exists() {
                return Ok(());
            }
            let process = (|| -> Result<()> {
                let state = if let Some(path) = paths.get(&b) {
                    serde_json::from_str(&read(path)?)?
                } else {
                    json!({"status":"running"})
                };
                if self.state["batches"][format!("{p}/{b}")]["launch_pending"]
                    .as_bool()
                    .unwrap_or(false)
                    && self.latest(p, &b).is_none()
                {
                    return Ok(());
                }
                self.process_batch(p, &b, &state)
            })();
            if let Err(e) = process {
                self.log("error", p, &b, &e.to_string())?;
            }
            self.store()?;
        }
        let pending = self.state["batches"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(k, v)| {
                k.starts_with(&format!("{p}/")) && v["launch_pending"].as_bool().unwrap_or(false)
            })
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        for k in pending {
            if self.stops()?.is_empty() {
                let b = k.split('/').next_back().unwrap();
                if self.latest(p, b).is_none() {
                    self.batch_cmd("start", p, b, &[])?;
                }
                self.state["batches"][&k]
                    .as_object_mut()
                    .unwrap()
                    .remove("launch_pending");
                self.store()?;
            }
        }
        if self.home.join("stop-now").exists() {
            return Ok(());
        }
        if self.stops()?.is_empty()
            && self.count(p)? < self.goal(p)? as usize
            && self.allowed.as_ref().is_none_or(|a| a.contains(p))
        {
            if let Some(rows) = self.ensure_roadmap(p)? {
                self.new_batch(p, &rows)?;
            }
        }
        Ok(())
    }
    /// A fresh worktree on main for the coordinator to edit (Codex cannot write .git).
    fn prepare_worktree(&self) -> Result<()> {
        let wt = self.home.join("wt");
        let w = wt.to_string_lossy().into_owned();
        if wt.exists() {
            self.cmd(&["git", "worktree", "remove", "--force", &w], false)?;
            if wt.exists() {
                fs::remove_dir_all(&wt)?;
            }
        }
        self.cmd(&["git", "worktree", "prune"], true)?;
        self.cmd(
            &[
                "git",
                "worktree",
                "add",
                "-q",
                "-B",
                "autopilot-coordinator",
                &w,
                "main",
            ],
            true,
        )?;
        let venv = self.root.join(".venv");
        if venv.exists() {
            std::os::unix::fs::symlink(&venv, wt.join(".venv"))?;
        }
        Ok(())
    }
    /// Carry out the finished coordinator's actions.json. Codex decides; this executes
    /// with the same gates as the coordinator rules (tests, full check, protected paths).
    fn apply(&mut self) -> Result<()> {
        self.state["coordinator"]["applied"] = json!(true);
        self.store()?;
        let job = self.state["coordinator"]["job"]
            .as_str()
            .unwrap_or("")
            .to_string();
        let path = self.home.join("jobs").join(&job).join("actions.json");
        let mut results = Vec::new();
        match read(&path).and_then(|t| Ok(serde_json::from_str::<Value>(&t)?)) {
            Err(e) => results.push(format!("{job}: actions.json を読めない: {e}")),
            Ok(v) => {
                for a in v["actions"].as_array().cloned().unwrap_or_default() {
                    match self.action(&a) {
                        Ok(m) => results.push(format!("ok {a}: {m}")),
                        Err(e) => {
                            results.push(format!("失敗 {a}: {e}（後続の操作を中止）"));
                            break;
                        }
                    }
                }
            }
        }
        self.log("coordinator-applied", "", "", &results.join(" / "))?;
        self.state["coordinator"]["last_result"] = json!(results);
        self.store()
    }
    fn action(&mut self, a: &Value) -> Result<String> {
        let get = |k: &str| a[k].as_str().unwrap_or("").to_string();
        let kind = get("do");
        if kind == "merge" {
            return self.merge(&get("message"));
        }
        if kind == "ask" {
            let (item, question) = (get("item"), get("question"));
            if item.is_empty() || question.is_empty() {
                bail!("ask には item と question が要る")
            }
            self.state["coordinator"]["asked"][&item] = json!(question.clone());
            self.log("coordinator-ask", "", "", &format!("{item}: {question}"))?;
            return Ok("ユーザーの判断待ちにした".into());
        }
        if kind == "start" {
            let stop = self.home.join("stop");
            if stop.exists() {
                fs::remove_file(stop)?;
            }
            return Ok("停止の指示を解除した".into());
        }
        let (p, b) = (get("persona"), get("batch"));
        if !regex(r"^p\d\d-[a-z0-9-]+$").is_match(&p)
            || !self.root.join("personas").join(format!("{p}.md")).exists()
            || !regex(r"^b\d{3}$").is_match(&b)
        {
            bail!("persona・batch が不正")
        }
        match kind.as_str() {
            "accept" => {
                let drops: Vec<String> = a["drop"]
                    .as_array()
                    .map(|d| {
                        d.iter()
                            .filter_map(|v| v.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                if drops
                    .iter()
                    .any(|d| d.is_empty() || d.starts_with('/') || d.contains(".."))
                {
                    bail!("drop のパスが不正")
                }
                let mut flags = Vec::new();
                if !drops.is_empty() {
                    flags.push("--drop".to_string());
                    flags.extend(drops);
                }
                self.update_roadmap(&p, &b, "受入済み")?;
                self.state["push_pending"] = json!(true);
                self.batch_cmd("accept", &p, &b, &flags)?;
                self.state["batches"][format!("{p}/{b}")]["accepted"] = json!(true);
                self.store()?;
                self.push()?;
            }
            "discard" => {
                let reason = get("reason");
                if reason.is_empty() {
                    bail!("破棄の理由が無い")
                }
                self.state["push_pending"] = json!(true);
                self.batch_cmd("discard", &p, &b, &["--reason".into(), reason])?;
                self.push()?;
            }
            "resume" => {
                let flag = get("flag");
                if !["", "--rebuild", "--redraft", "--review-again"].contains(&flag.as_str()) {
                    bail!("resume の flag が不正")
                }
                let flags = if flag.is_empty() { vec![] } else { vec![flag] };
                self.batch_cmd("resume", &p, &b, &flags)?;
            }
            "unhold" => {
                let _lock = lock(&self.root, "records", true)?;
                let rows: Vec<Row> = self
                    .holds()?
                    .into_iter()
                    .filter(|r| !(r.get("persona") == p && r.get("batch") == b))
                    .collect();
                write_holds(&self.root, &rows)?;
                drop(_lock);
                self.log("unhold", &p, &b, "統括")?;
            }
            _ => bail!("不明な操作"),
        }
        Ok("done".into())
    }
    fn merge(&mut self, message: &str) -> Result<String> {
        if message.trim().is_empty() {
            bail!("merge の message が無い")
        }
        let wt = self.home.join("wt");
        let w = wt.to_string_lossy().into_owned();
        if !wt.exists() {
            bail!("worktree が無い")
        }
        self.cmd(&["git", "-C", &w, "add", "-A", "--", ".", ":!.venv"], true)?;
        let diff = self
            .cmd(
                &[
                    "git",
                    "-C",
                    &w,
                    "diff",
                    "--cached",
                    "--name-status",
                    "--no-renames",
                    "-z",
                ],
                true,
            )?
            .text;
        let parts: Vec<&str> = diff.split('\0').filter(|s| !s.is_empty()).collect();
        let changes: Vec<(String, String)> = parts
            .chunks(2)
            .filter(|c| c.len() == 2)
            .map(|c| (c[0].to_string(), c[1].to_string()))
            .collect();
        if changes.is_empty() {
            return Ok("変更なし".into());
        }
        let refused: Vec<String> = changes
            .iter()
            .filter(|(st, path)| {
                ["SPEC.md", "AGENTS.md"].contains(&path.as_str())
                    || path.starts_with(".git")
                    || ((path.starts_with("people/") || path.starts_with("sources/")) && st != "D")
            })
            .map(|(_, p)| p.clone())
            .collect();
        let removed = changes
            .iter()
            .filter(|(st, p)| st == "D" && p.starts_with("people/"))
            .count();
        let allowance = self.home.join("allow-delete");
        let limit = fs::read_to_string(&allowance)
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(0)
            .max(3);
        if !refused.is_empty() || removed > limit {
            self.cmd(&["git", "-C", &w, "reset", "-q"], true)?;
            bail!(
                "取り込めない変更（SPEC.md・AGENTS.md はユーザーの確認が要る。people/・sources/ は削除だけ、一度に {limit} 件まで。削除 {removed} 件）: {}",
                refused.join(", ")
            )
        }
        self.cmd(&["git", "-C", &w, "commit", "-qm", message], true)?;
        if self
            .cmd(&["git", "-C", &w, "rebase", "-q", "main"], false)?
            .code
            != 0
        {
            self.cmd(&["git", "-C", &w, "rebase", "--abort"], false)?;
            bail!("main への rebase が衝突した")
        }
        let code = changes.iter().any(|(_, p)| {
            ["tools/", "tests/", "ops/"]
                .iter()
                .any(|d| p.starts_with(d))
        });
        if code {
            for script in ["tests/run.sh", "ops/tests/run.sh"] {
                if wt.join(script).exists() {
                    let o = command(
                        &wt,
                        &[wt.join(script).to_string_lossy().into_owned()],
                        false,
                    )?;
                    if o.code != 0 {
                        let tail: String = o
                            .text
                            .lines()
                            .rev()
                            .take(15)
                            .collect::<Vec<_>>()
                            .into_iter()
                            .rev()
                            .collect::<Vec<_>>()
                            .join("\n");
                        bail!("{script} が失敗した:\n{tail}")
                    }
                }
            }
            if wt.join("tools/Cargo.toml").exists() {
                let manifest = wt.join("tools/Cargo.toml").to_string_lossy().into_owned();
                self.cmd(
                    &[
                        "cargo",
                        "build",
                        "--release",
                        "-q",
                        "--manifest-path",
                        &manifest,
                    ],
                    true,
                )?;
                let binary = wt
                    .join("tools/target/release/corpus")
                    .to_string_lossy()
                    .into_owned();
                let base = format!("CORPUS_BASE={}", self.root.to_string_lossy());
                let o = self.cmd(
                    &["env", &base, &binary, "check", "--all", "--accepted"],
                    false,
                )?;
                if o.code != 0 {
                    let tail: String = o
                        .text
                        .lines()
                        .rev()
                        .take(15)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join("\n");
                    bail!("受入済みの全ファイルの検査に通らない:\n{tail}")
                }
            }
        }
        let personas: BTreeSet<String> = changes
            .iter()
            .filter(|(_, p)| p.starts_with("people/") || p.starts_with("manifests/"))
            .filter_map(|(_, p)| {
                let rest = p.split_once('/')?.1;
                Some(rest.split(['/', '.']).next()?.to_string())
            })
            .collect();
        for persona in personas {
            let base = format!("CORPUS_BASE={w}");
            let manifest = self
                .root
                .join("bin/manifest")
                .to_string_lossy()
                .into_owned();
            let o = self.cmd(&["env", &base, &manifest, &persona, "--verify"], false)?;
            if o.code != 0 {
                bail!("manifest の照合に通らない（{persona}）: {}", o.text.trim())
            }
        }
        {
            let _lock = lock(&self.root, "records", true)?;
            self.cmd(
                &["git", "merge", "--ff-only", "-q", "autopilot-coordinator"],
                true,
            )?;
            self.state["push_pending"] = json!(true);
            self.store()?;
        }
        self.push_attempted = false;
        self.push()?;
        if code && self.root.join("tools/Cargo.toml").exists() {
            let manifest = self
                .root
                .join("tools/Cargo.toml")
                .to_string_lossy()
                .into_owned();
            self.cmd(
                &[
                    "cargo",
                    "build",
                    "--release",
                    "-q",
                    "--manifest-path",
                    &manifest,
                ],
                true,
            )?;
            self.restart = true;
        }
        let head = self
            .cmd(&["git", "rev-parse", "--short", "HEAD"], true)?
            .text;
        if removed > 3 && allowance.exists() {
            fs::remove_file(&allowance)?;
        }
        Ok(format!("取り込んだ {}", head.trim()))
    }
    /// What the coordinator must handle now: holds, accepted batches due for a check,
    /// and batches whose processing keeps failing.
    fn agenda(&self) -> Result<Vec<String>> {
        let mut items: Vec<String> = self
            .holds()?
            .iter()
            .map(|r| format!("hold:{}/{}", r.get("persona"), r.get("batch")))
            .collect();
        let log = self.home.join("log.tsv");
        let lines: Vec<Vec<String>> = if log.exists() {
            read(&log)?
                .lines()
                .map(|l| l.split('\t').map(str::to_string).collect())
                .filter(|c: &Vec<String>| c.len() >= 4)
                .collect()
        } else {
            Vec::new()
        };
        let checks = check_rows(&self.root)?;
        let checked: BTreeSet<String> = checks.iter().map(|(k, _)| k.clone()).collect();
        let streak = checks.iter().rev().take_while(|(_, v)| v == "一致").count();
        let mut accepts: Vec<String> = Vec::new();
        for c in lines.iter().filter(|c| c[1] == "accept") {
            let key = format!("{}/{}", c[2], c[3]);
            accepts.retain(|k| k != &key);
            accepts.push(key);
        }
        let due: Vec<String> = if streak >= 10 {
            let last = accepts.iter().rposition(|k| checked.contains(k));
            let since: Vec<String> = accepts[last.map_or(0, |i| i + 1)..]
                .iter()
                .filter(|k| !checked.contains(*k))
                .cloned()
                .collect();
            if since.len() >= 10 { since } else { Vec::new() }
        } else {
            accepts
                .into_iter()
                .filter(|k| !checked.contains(k))
                .collect()
        };
        items.extend(due.into_iter().map(|k| format!("check:{k}")));
        let since = self.state["coordinator"]["launched"].as_str().unwrap_or("");
        let mut errors: BTreeMap<String, usize> = BTreeMap::new();
        for c in lines
            .iter()
            .filter(|c| c[1] == "error" && c[0].as_str() > since)
        {
            *errors
                .entry(format!("errors:{}/{}", c[2], c[3]))
                .or_default() += 1;
        }
        items.extend(errors.into_iter().filter(|(_, n)| *n >= 3).map(|(k, _)| k));
        Ok(items)
    }
    /// Launch the Codex coordinator (prompts/coordinator.md) for the agenda, one job at a time.
    /// An item still open after three coordinator runs is left for the user.
    fn coordinate(&mut self) -> Result<()> {
        let active = self.jobs.iter().any(|(n, s)| {
            regex(r"^coordinator(?:-r\d+)?$").is_match(n) && ACTIVE.contains(&s.as_str())
        });
        if active {
            return Ok(());
        }
        let job = self.state["coordinator"]["job"]
            .as_str()
            .unwrap_or("")
            .to_string();
        if self.state["coordinator"]["applied"] == json!(false)
            && self.jobs.get(&job).map(String::as_str) == Some("failed")
            && self.limited(&job)
        {
            self.state["coordinator"]["applied"] = json!(true);
            for item in self.state["coordinator"]["agenda"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .filter_map(Value::as_str)
            {
                let n = self.state["coordinator"]["attempts"][item]
                    .as_i64()
                    .unwrap_or(1);
                self.state["coordinator"]["attempts"][item] = json!((n - 1).max(0));
            }
            self.note_limit(&job)?;
        }
        if self.paused() {
            return self.store();
        }
        if self.state["coordinator"]["applied"] == json!(false) {
            self.apply()?;
            self.push_attempted = false;
            self.commit_holds()?;
        }
        let agenda = self.agenda()?;
        if self.state["coordinator"].get("attempts").is_none() {
            self.state["coordinator"]["attempts"] = json!({});
        }
        if !self.state["coordinator"]["asked"].is_object() {
            self.state["coordinator"]["asked"] = json!({});
        }
        let answers_file = self.home.join("answers.tsv");
        let mut answers = serde_json::Map::new();
        if answers_file.exists() {
            for line in read(&answers_file)?.lines() {
                if let Some((item, text)) = line.split_once('\t') {
                    answers.insert(item.to_string(), json!(text));
                }
            }
        }
        for item in answers.keys() {
            self.state["coordinator"]["asked"]
                .as_object_mut()
                .unwrap()
                .remove(item);
            self.state["coordinator"]["attempts"]
                .as_object_mut()
                .unwrap()
                .remove(item);
        }
        let asked = self.state["coordinator"]["asked"].as_object_mut().unwrap();
        asked.retain(|k, _| agenda.contains(k));
        let asked: Vec<String> = asked.keys().cloned().collect();
        let agenda: Vec<String> = agenda.into_iter().filter(|i| !asked.contains(i)).collect();
        let attempts = self.state["coordinator"]["attempts"]
            .as_object_mut()
            .unwrap();
        attempts.retain(|k, _| agenda.contains(k) || k.starts_with("errors:"));
        let mut open = Vec::new();
        let mut stuck = Vec::new();
        for item in &agenda {
            if attempts.get(item).and_then(Value::as_i64).unwrap_or(0) >= 3 {
                stuck.push(item.clone())
            } else {
                open.push(item.clone())
            }
        }
        let known: Vec<String> = self.state["coordinator"]["stuck"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        for item in stuck.iter().filter(|i| !known.contains(i)) {
            self.log("coordinator-stuck", "", "", item)?;
        }
        self.state["coordinator"]["stuck"] = json!(stuck);
        if open.is_empty() {
            return self.store();
        }
        for item in &open {
            let n = self.state["coordinator"]["attempts"][item]
                .as_i64()
                .unwrap_or(0);
            self.state["coordinator"]["attempts"][item] = json!(n + 1);
        }
        let name = self.unique("coordinator");
        let work = self.home.join("jobs").join(&name);
        fs::create_dir_all(&work)?;
        self.prepare_worktree()?;
        let scratch = work
            .strip_prefix(&self.root)?
            .to_string_lossy()
            .into_owned();
        let context = json!({
            "agenda": open,
            "scratch": scratch,
            "worktree": "scratch/autopilot/wt",
            "actions": format!("{scratch}/actions.json"),
            "previous": self.state["coordinator"]["last_result"].clone(),
            "answers": answers,
            "launched_by": "autopilot",
        });
        if answers_file.exists() {
            fs::remove_file(&answers_file)?;
        }
        let brief = work.join("brief.md");
        fs::write(
            &brief,
            read(&self.root.join("prompts/coordinator.md"))?
                + "\n\nAUTOPILOT_CONTEXT\n"
                + &serde_json::to_string(&context)?
                + "\n",
        )?;
        self.state["coordinator"]["job"] = json!(name);
        self.state["coordinator"]["agenda"] = json!(open);
        self.state["coordinator"]["launched"] = json!(now());
        self.state["coordinator"]["applied"] = json!(false);
        self.store()?;
        let cmd = shell_words::join([
            self.root.join("bin/batch").to_string_lossy().into_owned(),
            "codex-once".into(),
            "--brief".into(),
            brief.to_string_lossy().into_owned(),
            "--output".into(),
            work.join("out.md").to_string_lossy().into_owned(),
        ]);
        let command = format!(
            "cd {} && {cmd}",
            shell_words::quote(&self.root.to_string_lossy())
        );
        self.job(&argv(&[
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            &name,
            "--model",
            "gpt-6.2-astra",
            "--effort",
            "medium",
            "--command",
            &command,
        ]))?;
        self.jobs.insert(name.clone(), "running".into());
        self.log("coordinator", "", "", &format!("{name} {}", open.join(" ")))
    }
    fn status(&self) -> Result<()> {
        println!("persona\t受入済み/目標\t進行中 batch:段階\t成形待ち\t破棄\t保留\troadmap残り");
        for p in self.names()? {
            let mut running = Vec::new();
            let mut pending = 0;
            let mut dropped = 0;
            for (b, path) in self.states(&p) {
                match read(&path).and_then(|s| Ok(serde_json::from_str::<Value>(&s)?)) {
                    Ok(s) => {
                        if !["accepted", "discarded"].contains(&s["status"].as_str().unwrap_or(""))
                        {
                            running.push(format!(
                                "{b}:{}({})",
                                s["stage"].as_str().unwrap_or("?"),
                                s["status"].as_str().unwrap_or("?")
                            ))
                        }
                    }
                    Err(e) => running.push(format!("{b}:error {e}")),
                }
            }
            let mut reported = running
                .iter()
                .map(|s| s.split(':').next().unwrap().to_string())
                .collect::<BTreeSet<_>>();
            let pat = regex(&format!(
                r"^{}-(b\d+)(?:-r\d+)?$",
                regex::escape(p.split('-').next().unwrap())
            ));
            for (n, s) in &self.jobs {
                if ACTIVE.contains(&s.as_str()) {
                    if let Some(c) = pat.captures(n) {
                        if reported.insert(c[1].into()) {
                            running.push(format!("{}:起動待ち({s})", &c[1]));
                        }
                    }
                }
            }
            for plan in list(&self.root.join("progress").join(&p)) {
                let name = plan.file_name().unwrap().to_string_lossy();
                if regex(r"^b\d+\.md$").is_match(&name) {
                    if let Ok(rows) = self.rows(&p, plan.file_stem().unwrap().to_str().unwrap()) {
                        pending += rows
                            .iter()
                            .filter(|r| r.get("内容").starts_with("成形待ち："))
                            .count();
                        dropped += rows
                            .iter()
                            .filter(|r| r.get("内容").starts_with("破棄："))
                            .count();
                    }
                }
            }
            let remaining = self
                .roadmap(&p)
                .map(|r| {
                    r.iter()
                        .filter(|r| r.get("状態").is_empty())
                        .count()
                        .to_string()
                })
                .unwrap_or("未作成/不正".into());
            let holds = self
                .holds()?
                .iter()
                .filter(|r| [p.as_str(), "全体"].contains(&r.get("persona")))
                .map(|r| format!("{}: {}", r.get("batch"), r.get("理由")))
                .collect::<Vec<_>>()
                .join("; ");
            println!(
                "{p}\t{}/{}\t{}\t{pending}\t{dropped}\t{}\t{remaining}",
                self.count(&p)?,
                self.goal(&p)?,
                if running.is_empty() {
                    "-".into()
                } else {
                    running.join(",")
                },
                if holds.is_empty() { "-" } else { &holds }
            );
        }
        let mut used = 0;
        for path in list(&self.root.join("scratch/.slots")) {
            if locked(&path)? {
                used += 1
            }
        }
        let live = loop_running(&self.root)?
            || self.jobs.iter().any(|(n, s)| {
                regex(r"^autopilot(?:-r\d+)?$").is_match(n) && ACTIVE.contains(&s.as_str())
            });
        println!(
            "Codex 使用中: {used} / 自動運転: {}",
            if live { "稼働中" } else { "停止" }
        );
        let job = self.state["coordinator"]["job"].as_str().unwrap_or("");
        let stuck: Vec<&str> = self.state["coordinator"]["stuck"]
            .as_array()
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        let streak = check_rows(&self.root)?
            .iter()
            .rev()
            .take_while(|(_, v)| v == "一致")
            .count();
        println!(
            "統括: {} / 確認の連続一致: {streak} / 3 回で片付かず手が要るもの: {}",
            if job.is_empty() {
                "未起動".to_string()
            } else {
                format!(
                    "{job}（{}）",
                    self.jobs.get(job).map_or("不明", String::as_str)
                )
            },
            if stuck.is_empty() {
                "なし".to_string()
            } else {
                stuck.join(" ")
            }
        );
        let asked: Vec<String> = self.state["coordinator"]["asked"]
            .as_object()
            .map(|a| {
                a.iter()
                    .map(|(k, v)| format!("{k}（{}）", v.as_str().unwrap_or("")))
                    .collect()
            })
            .unwrap_or_default();
        if !asked.is_empty() {
            println!(
                "ユーザーの判断待ち（`bin/autopilot answer <項目> <答え>` で答える）: {}",
                asked.join(" / ")
            );
        }
        let reasons = self.stops()?.join("、");
        println!(
            "止める条件: {}",
            if reasons.is_empty() {
                "なし"
            } else {
                &reasons
            }
        );
        Ok(())
    }
}
fn execute(a: Action) -> Result<i32> {
    let (personas, target, status) = match &a {
        Action::Start { personas, target }
        | Action::Run { personas, target }
        | Action::Once { personas, target } => (personas.clone(), *target, false),
        Action::Status { personas, target } => (personas.clone(), *target, true),
        _ => (None, Some(300), false),
    };
    let mut pilot = Pilot::new(personas, target.unwrap_or(300))?;
    if status {
        if pilot.personas.is_none() {
            pilot.personas = pilot.state["configuration"]["personas"]
                .as_str()
                .map(str::to_string)
        }
        if target.is_none() {
            pilot.target = pilot.state["configuration"]["target"]
                .as_i64()
                .unwrap_or(300)
        }
    }
    if pilot.target < 1 {
        eprintln!("autopilot: error: --target は正の整数");
        return Ok(2);
    }
    if matches!(
        a,
        Action::Start { .. } | Action::Run { .. } | Action::Once { .. }
    ) && pilot
        .cmd(&["git", "branch", "--show-current"], true)?
        .text
        .trim()
        != "main"
    {
        bail!("自動運転は main で実行する")
    }
    match a {
        Action::Status { .. } => {
            pilot.refresh()?;
            pilot.status()?;
        }
        Action::Answer {
            item,
            text,
            allow_delete,
        } => {
            if item.contains('\t')
                || text.contains('\t')
                || text.contains('\n')
                || text.trim().is_empty()
            {
                bail!("答えはタブと改行を含まない 1 行で書く")
            }
            fs::create_dir_all(&pilot.home)?;
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(pilot.home.join("answers.tsv"))?;
            std::io::Write::write_all(&mut f, format!("{item}\t{text}\n").as_bytes())?;
            if let Some(n) = allow_delete {
                fs::write(pilot.home.join("allow-delete"), n.to_string())?;
            }
            pilot.log("answer", "", "", &format!("{item}: {text}"))?;
            println!("答えを渡した。次の統括の起動で使われる");
        }
        Action::Stop { now } => {
            pilot.refresh()?;
            fs::create_dir_all(&pilot.home)?;
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(pilot.home.join("stop"))?;
            if now {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(pilot.home.join("stop-now"))?;
                for (n, s) in &pilot.jobs {
                    if regex(r"^autopilot(?:-r\d+)?$").is_match(n) && ACTIVE.contains(&s.as_str()) {
                        pilot.job(&argv(&["stop", "personaCorpus", n]))?;
                    }
                }
            }
        }
        Action::Start { .. } => {
            let _lock = lock(&pilot.root, "autopilot-start", true)?;
            pilot.refresh()?;
            if loop_running(&pilot.root)?
                || pilot.jobs.iter().any(|(n, s)| {
                    regex(r"^autopilot(?:-r\d+)?$").is_match(n) && ACTIVE.contains(&s.as_str())
                })
            {
                let stop = pilot.home.join("stop");
                if stop.exists() {
                    fs::remove_file(stop)?;
                    println!("自動運転は起動済み（停止の指示を解除した）");
                } else {
                    println!("自動運転は起動済み");
                }
                return Ok(0);
            }
            for n in ["stop", "stop-now"] {
                let p = pilot.home.join(n);
                if p.exists() {
                    fs::remove_file(p)?;
                }
            }
            let mut cmd = vec![
                pilot
                    .root
                    .join("bin/autopilot")
                    .to_string_lossy()
                    .into_owned(),
                "run".into(),
                "--target".into(),
                pilot.target.to_string(),
            ];
            if let Some(p) = &pilot.personas {
                cmd.extend(["--personas".into(), p.clone()])
            }
            let shell = format!(
                "cd {} && {}",
                shell_words::quote(&pilot.root.to_string_lossy()),
                shell_words::join(cmd)
            );
            pilot.job(&argv(&[
                "start",
                "--repo",
                "personaCorpus",
                "--name",
                &pilot.unique("autopilot"),
                "--command",
                &shell,
            ]))?;
        }
        Action::Run { .. } | Action::Once { .. } => {
            let _lock = match lock(&pilot.root, "autopilot-loop", false) {
                Ok(f) => f,
                Err(e) => {
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::WouldBlock)
                    {
                        eprintln!("別の自動運転が実行中");
                        return Ok(1);
                    }
                    return Err(e);
                }
            };
            while !pilot.home.join("stop-now").exists() {
                let result = pilot.refresh().and_then(|_| pilot.once());
                if let Err(e) = result {
                    pilot.log("error", "", "", &e.to_string())?;
                }
                if matches!(a, Action::Once { .. }) {
                    break;
                }
                if pilot.restart {
                    // A merged coordinator fix rebuilt the tools: continue with the new binary.
                    use std::os::unix::process::CommandExt;
                    let mut args = vec![
                        "run".to_string(),
                        "--target".into(),
                        pilot.target.to_string(),
                    ];
                    if let Some(p) = &pilot.personas {
                        args.extend(["--personas".into(), p.clone()]);
                    }
                    drop(_lock);
                    let e = Command::new(pilot.root.join("bin/autopilot"))
                        .args(args)
                        .exec();
                    bail!("自動運転を新しい道具で起動し直せない: {e}");
                }
                let seconds = std::env::var("AUTOPILOT_INTERVAL_SECONDS")
                    .unwrap_or("60".into())
                    .parse::<f64>()?;
                let end = std::time::Instant::now() + Duration::from_secs_f64(seconds.max(0.));
                while std::time::Instant::now() < end && !pilot.home.join("stop-now").exists() {
                    std::thread::sleep(
                        Duration::from_secs(1)
                            .min(end.saturating_duration_since(std::time::Instant::now())),
                    );
                }
            }
        }
    }
    Ok(0)
}
pub fn run(args: &[String]) -> i32 {
    let cli = match Cli::try_parse_from(
        std::iter::once("autopilot".to_string()).chain(args.iter().cloned()),
    ) {
        Ok(a) => a,
        Err(e) => {
            let code = e.exit_code();
            let _ = e.print();
            return code;
        }
    };
    match execute(cli.action) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("autopilot: {e}");
            1
        }
    }
}
