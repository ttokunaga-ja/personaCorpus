use std::{
    fs,
    path::Path,
    process::{Command, Output, Stdio},
    thread,
    time::Duration,
};
struct Ops {
    root: tempfile::TempDir,
    socket: String,
}
impl Ops {
    fn command(&self, name: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_corpus"));
        c.arg(name)
            .env("AGENT_OPS_HOME", self.root.path())
            .env("AGENT_WATCH_INTERVAL", "0.1")
            .env("AGENT_ALERT_INTERVAL", "0.1")
            .env("AGENT_STALL_SECONDS", "1")
            .env("AGENT_TMUX_SOCKET", &self.socket);
        c
    }
    fn run(&self, name: &str, args: &[&str]) -> Output {
        let out = self.command(name).args(args).output().unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        out
    }
    fn tmux(&self, args: &[&str]) {
        assert!(
            Command::new("tmux")
                .args(["-L", &self.socket])
                .args(args)
                .status()
                .unwrap()
                .success()
        );
    }
}
impl Drop for Ops {
    fn drop(&mut self) {
        let _ = Command::new("tmux")
            .args(["-L", &self.socket, "kill-server"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}
#[test]
fn job_states_alert_cursor_and_recovery() {
    let ops = Ops {
        root: tempfile::tempdir().unwrap(),
        socket: format!("corpus-test-{}", std::process::id()),
    };
    let mut wait = ops
        .command("agent-alert")
        .args(["wait", "--repo", "personaCorpus"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(200));
    assert!(wait.try_wait().unwrap().is_none());
    ops.run(
        "codex-job",
        &[
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            "success",
            "--command",
            "echo success",
        ],
    );
    let output = wait.wait_with_output().unwrap();
    assert!(output.status.success());
    let first = String::from_utf8_lossy(&output.stdout);
    assert!(
        first.contains("success") && first.contains("done"),
        "{first}"
    );
    assert!(
        ops.run("agent-alert", &["list", "--repo", "personaCorpus"])
            .stdout
            .is_empty()
    );
    for (name, command) in [
        ("failure", "exit 4"),
        ("stalled", "sleep 15"),
        ("limit", "echo 'rate limit exceeded'; sleep 15"),
        (
            "stdin",
            "echo 'Reading additional input from stdin...'; sleep 15",
        ),
        ("vanished", "sleep 15"),
    ] {
        ops.run(
            "codex-job",
            &[
                "start",
                "--repo",
                "personaCorpus",
                "--name",
                name,
                "--command",
                command,
            ],
        );
    }
    ops.tmux(&["kill-session", "-t", "cj-personaCorpus-vanished"]);
    thread::sleep(Duration::from_secs(3));
    let output = ops.run("agent-alert", &["wait", "--repo", "personaCorpus"]);
    let rows = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    for (job, event) in [
        ("failure", "failed"),
        ("stalled", "stalled"),
        ("limit", "limit"),
        ("stdin", "stdin-wait"),
        ("vanished", "failed"),
    ] {
        assert!(
            rows.iter()
                .any(|row| row["job"] == job && row["event"] == event),
            "{rows:?}"
        );
    }
    assert!(
        ops.run("agent-alert", &["list", "--repo", "personaCorpus"])
            .stdout
            .is_empty()
    );
    let all = ops.run("agent-alert", &["list", "--repo", "personaCorpus", "--all"]);
    assert!(!all.stdout.is_empty());
    ops.tmux(&["kill-session", "-t", "agent-watchd"]);
    ops.run("codex-job", &["status", "--repo", "personaCorpus"]);
    ops.tmux(&["has-session", "-t", "agent-watchd"]);
    for job in ["stalled", "limit", "stdin"] {
        ops.run("codex-job", &["stop", "personaCorpus", job]);
    }
    let data = fs::read_to_string(
        ops.root
            .path()
            .join(Path::new("alerts/personaCorpus.jsonl")),
    )
    .unwrap();
    let mut seen = std::collections::HashSet::new();
    for line in data.lines() {
        let value: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(seen.insert((value["job"].to_string(), value["event"].to_string())));
    }
}
#[test]
fn invalid_job_invocations_do_not_start() {
    let ops = Ops {
        root: tempfile::tempdir().unwrap(),
        socket: format!("corpus-invalid-{}", std::process::id()),
    };
    for args in [
        vec!["start", "--repo", "personaCorpus", "--name", "missing"],
        vec![
            "start",
            "--repo",
            "invalid",
            "--name",
            "valid",
            "--command",
            "true",
        ],
        vec!["status", "--repo", "invalid"],
        vec!["tail", "invalid", "valid"],
        vec!["stop", "invalid", "valid"],
        vec!["tail", "personaCorpus", "bad/name"],
        vec!["stop", "personaCorpus", "bad/name"],
        vec![
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            "bad/name",
            "--command",
            "true",
        ],
        vec![
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            "both",
            "--brief",
            "brief",
            "--command",
            "true",
        ],
        vec![
            "start",
            "--repo",
            "personaCorpus",
            "--name",
            "retired",
            "--test-command",
            "true",
        ],
    ] {
        let result = ops.command("codex-job").args(&args).output().unwrap();
        assert_eq!(
            result.status.code(),
            Some(2),
            "{args:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for action in ["wait", "list"] {
        let result = ops
            .command("agent-alert")
            .args([action, "--repo", "invalid"])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2));
    }
    assert!(!ops.root.path().join("jobs").exists());
    assert!(!ops.root.path().join("alerts").exists());
}
