use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand};
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime},
};
#[derive(Parser)]
struct JobCli {
    #[command(subcommand)]
    action: JobAction,
}
#[derive(Subcommand)]
enum JobAction {
    Start {
        #[arg(long,value_parser=["kio","personaCorpus"])]
        repo: String,
        #[arg(long,value_parser=parsed_name)]
        name: String,
        #[arg(long, required_unless_present = "command", conflicts_with = "command")]
        brief: Option<PathBuf>,
        #[arg(long, required_unless_present = "brief")]
        command: Option<String>,
        #[arg(long, default_value = "gpt-6.1-terra")]
        model: String,
        #[arg(long, default_value = "medium")]
        effort: String,
        #[arg(long,default_value="workspace-write",value_parser=["read-only","workspace-write","danger-full-access"])]
        sandbox: String,
        #[arg(long)]
        add_dir: Vec<PathBuf>,
    },
    Status {
        #[arg(long,value_parser=["kio","personaCorpus"])]
        repo: Option<String>,
    },
    Tail {
        #[arg(value_parser=["kio","personaCorpus"])]
        repo: String,
        #[arg(value_parser=parsed_name)]
        job: String,
    },
    Stop {
        #[arg(value_parser=["kio","personaCorpus"])]
        repo: String,
        #[arg(value_parser=parsed_name)]
        job: String,
    },
}
#[derive(Parser)]
struct AlertCli {
    #[command(subcommand)]
    action: AlertAction,
}
#[derive(Subcommand)]
enum AlertAction {
    Wait {
        #[arg(long,value_parser=["kio","personaCorpus"])]
        repo: String,
    },
    List {
        #[arg(long,value_parser=["kio","personaCorpus"])]
        repo: String,
        #[arg(long)]
        all: bool,
    },
}
fn base() -> PathBuf {
    let path = std::env::var_os("AGENT_OPS_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::base().join("ops"));
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir().unwrap_or_default().join(path)
    }
}
fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
fn interval(name: &str, default: f64) -> Duration {
    Duration::from_secs_f64(
        std::env::var(name)
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|n| n.is_finite() && *n > 0.)
            .unwrap_or(default),
    )
}
fn tmux(args: &[String]) -> Result<bool> {
    let mut command = Command::new("tmux");
    if let Ok(socket) = std::env::var("AGENT_TMUX_SOCKET") {
        command.args(["-L", &socket]);
    }
    Ok(command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success())
}
fn tmux_str(args: &[&str]) -> bool {
    tmux(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>()).unwrap_or(false)
}
fn alive(session: &str) -> bool {
    tmux_str(&["has-session", "-t", &format!("={session}")])
}
fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
fn self_command(args: &[&str]) -> Result<String> {
    Ok(format!(
        "{} {}",
        quote(&std::env::current_exe()?.to_string_lossy()),
        args.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" ")
    ))
}
fn ensure_watch() -> Result<()> {
    if !alive("agent-watchd") {
        ensure!(
            tmux_str(&[
                "new-session",
                "-d",
                "-s",
                "agent-watchd",
                &self_command(&["codex-job", "_watch"])?
            ]) || alive("agent-watchd"),
            "could not start watch daemon"
        );
    }
    Ok(())
}
fn save(path: &Path, value: &Value) -> Result<()> {
    let tmp = path.with_file_name(format!(
        "{}.tmp.{}",
        path.file_name().unwrap().to_string_lossy(),
        std::process::id()
    ));
    fs::write(&tmp, format!("{}\n", serde_json::to_string(value)?))?;
    fs::rename(tmp, path)?;
    Ok(())
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn parsed_name(value: &str) -> std::result::Result<String, String> {
    valid(value).map_err(|error| error.to_string())?;
    Ok(value.to_owned())
}
fn valid(value: &str) -> Result<()> {
    ensure!(
        regex::Regex::new(r"^[A-Za-z0-9][A-Za-z0-9_-]*$")?.is_match(value),
        "invalid job name: {value}"
    );
    Ok(())
}
fn repo_path(repo: &str) -> Result<PathBuf> {
    ensure!(
        ["kio", "personaCorpus"].contains(&repo),
        "invalid repository: {repo}"
    );
    let home = std::env::var_os("HOME").context("HOME missing")?;
    Ok(PathBuf::from(home)
        .join("dev/github.com/ttokunaga-ja")
        .join(repo))
}
fn jobs(repo: Option<&str>) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let root = base().join("jobs");
    if !root.exists() {
        return Ok(paths);
    }
    for r in fs::read_dir(root)? {
        let r = r?.path();
        if !r.is_dir() || repo.is_some_and(|n| r.file_name().unwrap() != n) {
            continue;
        }
        for job in fs::read_dir(r)? {
            let job = job?.path();
            if job.is_dir() && job.join("meta.json").exists() && job.join(".ready").exists() {
                paths.push(job);
            }
        }
    }
    paths.sort();
    Ok(paths)
}
fn tail_bytes(path: &Path, length: usize) -> Vec<u8> {
    let Ok(data) = fs::read(path) else {
        return vec![];
    };
    data[data.len().saturating_sub(length)..].to_vec()
}
fn exit_status(path: &Path) -> Result<Option<(&'static str, String)>> {
    let path = path.join("exit.json");
    if !path.exists() {
        return Ok(None);
    }
    let exit = read(&path)?;
    let code = exit["code"].as_i64().context("invalid exit code")?;
    Ok(Some((
        if code == 0 { "done" } else { "failed" },
        format!("exit code {code}"),
    )))
}
fn status(path: &Path) -> Result<(&'static str, String)> {
    let meta = read(&path.join("meta.json"))?;
    if let Some(status) = exit_status(path)? {
        return Ok(status);
    }
    if !alive(meta["session"].as_str().context("missing session")?) {
        // Completion can race the tmux query. The runner persists exit.json
        // before its session disappears, so check the terminal record again.
        if let Some(status) = exit_status(path)? {
            return Ok(status);
        }
        return Ok(("failed", "session disappeared without exit.json".into()));
    }
    let log = path.join("log");
    let bytes = tail_bytes(&log, 2048);
    let tail = String::from_utf8_lossy(&bytes);
    if regex::Regex::new(r"(?i)(usage limit|rate limit|\b429\b|\bquota\b)")?.is_match(&tail) {
        return Ok(("limit", "limit indication in recent log".into()));
    }
    let modified = fs::metadata(if log.exists() { &log } else { path })?.modified()?;
    let stalled = SystemTime::now()
        .duration_since(modified)
        .unwrap_or_default()
        >= interval("AGENT_STALL_SECONDS", 1200.);
    if stalled
        && tail
            .to_lowercase()
            .contains("reading additional input from stdin")
    {
        return Ok((
            "stdin-wait",
            "stdin prompt with no subsequent log activity".into(),
        ));
    }
    if stalled {
        return Ok(("stalled", "log has not grown within threshold".into()));
    }
    Ok(("running", "session active".into()))
}
fn scan() -> Result<()> {
    fs::create_dir_all(base().join("alerts"))?;
    for job in jobs(None)? {
        let result = (|| -> Result<()> {
            let (state, detail) = status(&job)?;
            let previous =
                fs::read_to_string(job.join(".watch-state")).unwrap_or_else(|_| "running".into());
            if previous.trim() != state {
                if state != "running" {
                    let bytes = tail_bytes(&job.join("log"), 8192);
                    let text = String::from_utf8_lossy(&bytes);
                    let lines = text.lines().collect::<Vec<_>>();
                    let log = &lines[lines.len().saturating_sub(8)..];
                    let repo = job.parent().unwrap().file_name().unwrap().to_string_lossy();
                    let record = json!({"ts":now(),"repo":repo,"job":job.file_name().unwrap().to_string_lossy(),"event":state,"detail":detail,"log":log});
                    let mut f = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(base().join("alerts").join(format!("{repo}.jsonl")))?;
                    writeln!(f, "{}", serde_json::to_string(&record)?)?;
                }
                fs::write(job.join(".watch-state"), state)?;
            }
            Ok(())
        })();
        if let Err(e) = result {
            eprintln!("watch: {}: {e:#}", job.display());
        }
    }
    Ok(())
}
fn watch() -> Result<()> {
    let folder = base().join("alerts");
    fs::create_dir_all(&folder)?;
    let lock = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(folder.join(".watch.lock"))?;
    if lock.try_lock_exclusive().is_err() {
        return Ok(());
    }
    loop {
        scan()?;
        thread::sleep(interval("AGENT_WATCH_INTERVAL", 60.));
    }
}
fn run_job(path: &Path) -> Result<()> {
    let meta = read(&path.join("meta.json"))?;
    let arguments = meta["command"]
        .as_array()
        .context("invalid command")?
        .iter()
        .map(|v| v.as_str().context("invalid argument"))
        .collect::<Result<Vec<_>>>()?;
    ensure!(!arguments.is_empty(), "empty command");
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.join("log"))?;
    let outcome = Command::new(arguments[0])
        .args(&arguments[1..])
        .env("CORPUS_JOB", path.file_name().unwrap_or_default())
        .current_dir(meta["cwd"].as_str().context("missing cwd")?)
        .stdin(Stdio::null())
        .stdout(log.try_clone()?)
        .stderr(log.try_clone()?)
        .status();
    let code = match outcome {
        Ok(status) => status.code().unwrap_or_else(|| {
            use std::os::unix::process::ExitStatusExt;
            -status.signal().unwrap_or(1)
        }),
        Err(e) => {
            writeln!(log, "{e}")?;
            127
        }
    };
    save(
        &path.join("exit.json"),
        &json!({"code":code,"finished_at":now()}),
    )
}
fn alert(args: &[String]) -> Result<()> {
    let cli = AlertCli::try_parse_from(
        std::iter::once("agent-alert".to_string()).chain(args.iter().cloned()),
    )?;
    let (repo, wait, all) = match cli.action {
        AlertAction::Wait { repo } => (repo, true, false),
        AlertAction::List { repo, all } => (repo, false, all),
    };
    repo_path(&repo)?;
    ensure_watch()?;
    let folder = base().join("alerts");
    fs::create_dir_all(&folder)?;
    let lock = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(folder.join(format!("{repo}.lock")))?;
    lock.lock_exclusive()?;
    let cursor = folder.join(format!("{repo}.cursor"));
    let path = folder.join(format!("{repo}.jsonl"));
    loop {
        let offset = if cursor.exists() {
            fs::read_to_string(&cursor)?.trim().parse::<usize>()?
        } else {
            0
        };
        let data = fs::read(&path).unwrap_or_default();
        ensure!(offset <= data.len(), "alert cursor exceeds file size");
        let chunk = &data[if all { 0 } else { offset }..];
        if let Some(end) = chunk.iter().rposition(|b| *b == b'\n') {
            let end = end + 1;
            std::io::stdout().write_all(&chunk[..end])?;
            std::io::stdout().flush()?;
            if wait {
                fs::write(cursor, (offset + end).to_string())?;
            }
            return Ok(());
        }
        if !wait {
            return Ok(());
        }
        thread::sleep(interval("AGENT_ALERT_INTERVAL", 1.));
    }
}
fn job(args: &[String]) -> Result<()> {
    let cli = JobCli::try_parse_from(
        std::iter::once("codex-job".to_string()).chain(args.iter().cloned()),
    )?;
    match cli.action {
        JobAction::Start {
            repo,
            name,
            brief,
            command,
            model,
            effort,
            sandbox,
            add_dir,
        } => {
            let cwd = repo_path(&repo)?;
            valid(&name)?;
            let brief = brief.map(fs::canonicalize).transpose()?;
            if let Some(path) = &brief {
                ensure!(path.is_file(), "brief does not exist");
            }
            ensure_watch()?;
            let path = base().join("jobs").join(&repo).join(&name);
            fs::create_dir_all(path.parent().unwrap())?;
            fs::create_dir(&path).context("job already exists; choose a new name")?;
            let arguments = if let Some(command) = command {
                vec!["bash".into(), "-c".into(), command]
            } else {
                let mut arguments = vec![
                    "codex".into(),
                    "exec".into(),
                    "-m".into(),
                    model.clone(),
                    "-c".into(),
                    format!("model_reasoning_effort={}", serde_json::to_string(&effort)?),
                    "-s".into(),
                    sandbox,
                    "-c".into(),
                    "sandbox_workspace_write.network_access=true".into(),
                    "-C".into(),
                    cwd.to_string_lossy().into_owned(),
                    "-o".into(),
                    path.join("out.md").to_string_lossy().into_owned(),
                ];
                for dir in add_dir {
                    arguments.push("--add-dir".into());
                    arguments.push(
                        if dir.is_absolute() {
                            dir
                        } else {
                            std::env::current_dir()?.join(dir)
                        }
                        .to_string_lossy()
                        .into_owned(),
                    );
                }
                arguments.push(fs::read_to_string(
                    brief.as_ref().context("brief required")?,
                )?);
                arguments
            };
            let session = format!("cj-{repo}-{name}");
            let meta = json!({"repo":repo,"cwd":cwd,"model":model,"effort":effort,"brief":brief,"started_at":now(),"session":session,"command":arguments});
            save(&path.join("meta.json"), &meta)?;
            fs::File::create(path.join("log"))?;
            if !tmux_str(&[
                "new-session",
                "-d",
                "-s",
                &session,
                &self_command(&["codex-job", "_run", &path.to_string_lossy()])?,
            ]) {
                save(
                    &path.join("exit.json"),
                    &json!({"code":127,"finished_at":now()}),
                )?;
                fs::File::create(path.join(".ready"))?;
                bail!("could not start tmux session");
            }
            fs::File::create(path.join(".ready"))?;
            println!("{session}");
        }
        JobAction::Status { repo } => {
            if let Some(repo) = &repo {
                repo_path(repo)?;
            }
            ensure_watch()?;
            println!("REPO\tJOB\tSTATE\tDETAIL");
            for path in jobs(repo.as_deref())? {
                let (state, detail) = status(&path)?;
                println!(
                    "{}\t{}\t{state}\t{detail}",
                    path.parent()
                        .unwrap()
                        .file_name()
                        .unwrap()
                        .to_string_lossy(),
                    path.file_name().unwrap().to_string_lossy()
                );
            }
        }
        JobAction::Tail { repo, job } | JobAction::Stop { repo, job } => {
            repo_path(&repo)?;
            valid(&job)?;
            ensure_watch()?;
            let path = base().join("jobs").join(&repo).join(&job);
            ensure!(path.join("meta.json").exists(), "job not found");
            if args[0] == "tail" {
                Command::new("tail")
                    .args(["-n", "40", "-f"])
                    .arg(path.join("log"))
                    .status()?;
            } else {
                ensure!(
                    tmux_str(&["kill-session", "-t", &format!("=cj-{repo}-{job}")]),
                    "session not running"
                );
            }
        }
    }
    Ok(())
}
pub fn run(command: &str, args: &[String]) -> i32 {
    let result = if args.first().is_some_and(|s| s == "_watch") {
        watch()
    } else if args.first().is_some_and(|s| s == "_run") {
        args.get(1)
            .context("job path required")
            .and_then(|s| run_job(Path::new(s)))
    } else {
        match command {
            "agent-alert" | "alert" => alert(args),
            "agent-watchd" | "watchd" => watch(),
            _ => job(args),
        }
    };
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
