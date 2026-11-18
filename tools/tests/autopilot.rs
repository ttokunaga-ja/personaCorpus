use serde_json::{Value, json};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    time::{Duration, Instant},
};
const P: &str = "p01-test";
const HEADER: &str = "| batch | 範囲 | 件数の目安 | 重点 | 含めたいもの | 避けるもの | 状態 |\n|---|---|---|---|---|---|---|\n";
const PLAN: &str =
    "| パス | 日付 | 作り方 | 内容 | 関連 | 先に要るもの |\n|---|---|---|---|---|---|\n";
fn atomic(p: &Path, v: &Value) {
    let t = p.with_extension("tmp");
    fs::write(&t, serde_json::to_vec(v).unwrap()).unwrap();
    fs::rename(t, p).unwrap();
}
#[test]
#[ignore]
fn fake_service() {
    let Some(file) = std::env::var_os("CORPUS_FAKE_ARGS") else {
        return;
    };
    let raw = fs::read(file).unwrap();
    let args = raw
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| String::from_utf8(b.to_vec()).unwrap())
        .collect::<Vec<_>>();
    let tool = std::env::var("CORPUS_FAKE_TOOL").unwrap();
    let r = std::env::current_dir().unwrap();
    let mut calls = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r.join("calls"))
        .unwrap();
    let mut row = vec![tool.clone()];
    row.extend(args.clone());
    writeln!(calls, "{}", serde_json::to_string(&row).unwrap()).unwrap();
    let jp = r.join("jobs.json");
    let mut jobs: Value = serde_json::from_slice(&fs::read(&jp).unwrap()).unwrap();
    if tool == "codex-job" {
        match args[0].as_str() {
            "status" => {
                if r.join("fail-first-enabled").exists() && !r.join("fail-first").exists() {
                    fs::write(r.join("fail-first"), "").unwrap();
                    std::process::exit(4)
                }
                println!("REPO\tJOB\tSTATE\tDETAIL");
                for (n, s) in jobs.as_object().unwrap() {
                    println!("personaCorpus\t{n}\t{}\tdetail", s.as_str().unwrap());
                }
            }
            "start" => {
                let n = &args[args.iter().position(|s| s == "--name").unwrap() + 1];
                jobs[n] = json!("running");
                atomic(&jp, &jobs)
            }
            "stop" => {
                jobs[args.last().unwrap()] = json!("failed");
                atomic(&jp, &jobs)
            }
            _ => panic!("bad job action"),
        }
    } else {
        match args[0].as_str() {
            "start" | "resume" => {
                let base = format!("{}-{}", args[1].split('-').next().unwrap(), args[2]);
                let mut n = base.clone();
                let mut i = 1;
                while jobs.get(&n).is_some() {
                    i += 1;
                    n = format!("{base}-r{i}")
                }
                jobs[&n] = json!("running");
                atomic(&jp, &jobs)
            }
            "accept" => {
                if r.join("accept-fails").exists() {
                    eprintln!("batch: --drop はこの batch の相対パスを指定する");
                    std::process::exit(1)
                }
                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .open(r.join("progress/batches.tsv"))
                    .unwrap();
                writeln!(file, "{}\t{}\taccepted", args[1], args[2]).unwrap();
                let mut paths = vec!["progress/batches.tsv".to_string()];
                let road = format!("progress/{}/roadmap.md", args[1]);
                if r.join(&road).exists() {
                    paths.push(road)
                }
                let status = Command::new("git")
                    .args(["add", "--"])
                    .args(paths)
                    .stdout(Stdio::null())
                    .status()
                    .unwrap();
                assert!(status.success());
                assert!(
                    Command::new("git")
                        .args(["commit", "-qm", "Accept fake batch"])
                        .stdout(Stdio::null())
                        .status()
                        .unwrap()
                        .success()
                );
            }
            _ => panic!("bad batch action"),
        }
    }
}
struct Fixture {
    _tmp: tempfile::TempDir,
    r: PathBuf,
    remote: PathBuf,
    env: Vec<(String, String)>,
}
impl Fixture {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path().join("repo");
        for d in [
            "bin",
            "ops/bin",
            "personas",
            "progress/p01-test",
            "prompts",
            "manifests",
            "scratch",
        ] {
            fs::create_dir_all(r.join(d)).unwrap();
        }
        for t in ["batch", "codex-job"] {
            let p = r.join(if t == "batch" {
                "bin/batch"
            } else {
                "ops/bin/codex-job"
            });
            let exe = shell_words::quote(&std::env::current_exe().unwrap().to_string_lossy())
                .into_owned();
            let script = format!(
                "#!/bin/sh\nargs=$(mktemp \"$PWD/scratch/fake-args.XXXXXX\")\nprintf '%s\\0' \"$@\" > \"$args\"\nCORPUS_FAKE_ARGS=\"$args\" CORPUS_FAKE_TOOL={t} exec {exe} --ignored --exact fake_service --nocapture\n"
            );
            fs::write(&p, script).unwrap();
            fs::set_permissions(p, fs::Permissions::from_mode(0o755)).unwrap();
        }
        for k in ["audit", "roadmap", "coordinator"] {
            fs::write(
                r.join(format!("prompts/{k}.md")),
                format!("{k} instructions"),
            )
            .unwrap();
        }
        fs::write(r.join("personas/p01-test.md"), "# test").unwrap();
        fs::write(r.join("jobs.json"), "{}").unwrap();
        fs::write(r.join("progress/batches.tsv"), "persona\tbatch\tstatus\n").unwrap();
        let remote = tmp.path().join("remote.git");
        let f = Self {
            _tmp: tmp,
            r,
            remote,
            env: vec![("AUTOPILOT_MIN_FREE_BYTES".into(), "0".into())],
        };
        f.git(&["init", "-qb", "main"]);
        f.git(&["config", "user.name", "Test"]);
        f.git(&["config", "user.email", "test@example.invalid"]);
        f.git(&["add", "."]);
        f.git(&["commit", "-qm", "fixture"]);
        assert!(
            Command::new("git")
                .args(["init", "--bare", "-q"])
                .arg(&f.remote)
                .status()
                .unwrap()
                .success()
        );
        f.git(&["remote", "add", "origin", f.remote.to_str().unwrap()]);
        f.git(&["push", "-q", "origin", "main"]);
        f
    }
    fn git(&self, a: &[&str]) -> String {
        let o = Command::new("git")
            .args(a)
            .current_dir(&self.r)
            .output()
            .unwrap();
        good(&o);
        String::from_utf8(o.stdout).unwrap()
    }
    fn command(&self, a: &[&str]) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_corpus"));
        c.arg("autopilot")
            .args(a)
            .current_dir(&self.r)
            .env("CORPUS_BASE", &self.r)
            .envs(self.env.iter().cloned())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        c
    }
    fn cli(&self, a: &[&str]) -> String {
        let o = self.command(a).output().unwrap();
        good(&o);
        String::from_utf8(o.stdout).unwrap()
    }
    fn once(&self) {
        self.cli(&["once", "--target", "30"]);
    }
    fn jobs(&self) -> Value {
        serde_json::from_slice(&fs::read(self.r.join("jobs.json")).unwrap()).unwrap()
    }
    fn job(&self, n: &str, s: &str) {
        let mut jobs = self.jobs();
        jobs[n] = json!(s);
        atomic(&self.r.join("jobs.json"), &jobs);
    }
    fn calls(&self) -> Vec<Vec<String>> {
        fs::read_to_string(self.r.join("calls"))
            .unwrap_or_default()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    }
    fn called(&self, tool: &str, action: &str) -> usize {
        self.calls()
            .iter()
            .filter(|c| c[0] == tool && c[1] == action)
            .count()
    }
    fn roadmap(&self, row: Option<&str>) {
        fs::write(
            self.r.join("progress/p01-test/roadmap.md"),
            format!(
                "{HEADER}{}",
                row.unwrap_or("| b001 | 月次締め | 30 | 差異調査 | 表とメール | 重複 | |\n")
            ),
        )
        .unwrap();
    }
    fn batch(&self, status: &str, reason: &str, pending: usize, dropped: usize) {
        let d = self.r.join("scratch/p01-test/b001");
        fs::create_dir_all(&d).unwrap();
        let failures = (0..dropped)
            .map(|i| (format!("d{i}.txt"), json!("原稿不合格")))
            .collect::<serde_json::Map<_, _>>();
        atomic(
            &d.join("state.json"),
            &json!({"status":status,"stage":"draft","reason":reason,"draft_failures":failures}),
        );
        let mut rows = PLAN.to_string();
        for (prefix, count, content) in [
            ("d", dropped, "破棄：原稿不合格。内容"),
            ("w", pending, "成形待ち：失敗。内容"),
        ] {
            for i in 0..count {
                rows.push_str(&format!(
                    "| {prefix}{i}.txt | 2026-01-01 | text | {content} | | |\n"
                ));
            }
        }
        rows.push_str("| ok.txt | 2026-01-01 | text | 内容 | | |\n");
        fs::write(self.r.join("progress/p01-test/b001.md"), rows).unwrap();
        for (base, ext) in [("people", ""), ("sources", ".md")] {
            let p = self.r.join(format!("{base}/p01-test/ok.txt{ext}"));
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, "original").unwrap();
        }
    }
    fn done(&self) {
        self.batch("done", "", 0, 0);
    }
    fn audit(&self, verdict: &str, drops: &str) {
        self.job("p01-b001-audit", "done");
        let judgment = if drops == "[]" { "合格" } else { "破棄" };
        fs::write(self.r.join("progress/p01-test/b001.audit.md"),format!("verdict: {verdict}\ndrop: {drops}\n\n| パス | 判定 | 理由 |\n|---|---|---|\n| ok.txt | {judgment} | 確認 |\n\n所見：確認\n")).unwrap();
    }
    fn holds(&self) -> String {
        fs::read_to_string(self.r.join("progress/holds.md")).unwrap_or_default()
    }
    fn state(&self) -> Value {
        serde_json::from_slice(&fs::read(self.r.join("scratch/autopilot/state.json")).unwrap())
            .unwrap()
    }
    fn save_state(&self, v: &Value) {
        atomic(&self.r.join("scratch/autopilot/state.json"), v)
    }
}
fn good(o: &Output) {
    assert!(
        o.status.success(),
        "code={:?}\n{}\n{}",
        o.status.code(),
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
}
#[test]
fn roadmap_retry_then_hold() {
    let f = Fixture::new();
    f.once();
    assert!(f.jobs().get("p01-roadmap").is_some());
    f.job("p01-roadmap", "done");
    f.once();
    assert!(f.jobs().get("p01-roadmap-r2").is_some());
    f.job("p01-roadmap-r2", "done");
    f.once();
    assert!(f.holds().contains("roadmap"));
}
#[test]
fn request_is_unique_and_target_prevents_admission() {
    let f = Fixture::new();
    f.roadmap(None);
    f.once();
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
    assert!(
        fs::read_to_string(f.r.join("progress/p01-test/b001.request.md"))
            .unwrap()
            .contains("範囲：月次締め")
    );
    assert!(
        fs::read_to_string(f.r.join("progress/p01-test/roadmap.md"))
            .unwrap()
            .contains("b001")
    );
    let g = Fixture::new();
    g.roadmap(None);
    fs::write(
        g.r.join("manifests/p01-test.tsv"),
        "x\thash\t1\tdate\n".repeat(30),
    )
    .unwrap();
    g.once();
    assert_eq!(g.called("batch", "start"), 0);
}
#[test]
fn redraft_then_two_rebuilds_then_audit() {
    let f = Fixture::new();
    f.batch("done", "", 1, 3);
    f.once();
    assert!(f.calls().iter().any(|c| c.contains(&"--redraft".into())));
    for n in ["p01-b001", "p01-b001-r2"] {
        f.job(n, "done");
        f.once();
        assert!(f.calls().iter().any(|c| c.contains(&"--rebuild".into())));
    }
    f.job("p01-b001-r3", "done");
    f.once();
    assert!(f.jobs().get("p01-b001-audit").is_some());
    assert_eq!(
        f.calls()
            .iter()
            .filter(|c| c.contains(&"--rebuild".into()))
            .count(),
        2
    );
}
#[test]
fn accept_drop_pushes_roadmap() {
    let f = Fixture::new();
    f.roadmap(Some(
        "| b001 | 月次 | 30 | 表 | メール | 重複 | 進行中 b001 |\n",
    ));
    f.done();
    f.once();
    f.audit("accept", "[\"ok.txt\"]");
    f.once();
    assert!(f.calls().iter().any(|c| c[0] == "batch"
        && c[1] == "accept"
        && c.ends_with(&["--drop".into(), "ok.txt".into()])));
    assert!(
        fs::read_to_string(f.r.join("progress/p01-test/roadmap.md"))
            .unwrap()
            .contains("受入済み")
    );
    assert_eq!(
        f.git(&["rev-parse", "main"]).trim(),
        f.git(&["ls-remote", "origin", "refs/heads/main"])
            .split_whitespace()
            .next()
            .unwrap()
    );
}
#[test]
fn failed_accept_holds_instead_of_retrying() {
    let f = Fixture::new();
    f.done();
    f.once();
    f.audit("accept", "[\"ok.txt\"]");
    fs::write(f.r.join("accept-fails"), "").unwrap();
    f.once();
    assert!(f.holds().contains("受入に失敗"), "{}", f.holds());
    f.once();
    assert_eq!(f.called("batch", "accept"), 1);
}
#[test]
fn holds_and_both_hash_trees_protect_acceptance() {
    for mode in ["hold", "people", "sources"] {
        let f = Fixture::new();
        f.done();
        f.once();
        f.audit(if mode == "hold" { "hold" } else { "accept" }, "[]");
        if mode == "people" {
            fs::write(f.r.join("people/p01-test/ok.txt"), "changed").unwrap()
        } else if mode == "sources" {
            fs::remove_file(f.r.join("sources/p01-test/ok.txt.md")).unwrap()
        }
        f.once();
        assert!(
            f.holds()
                .contains(if mode == "hold" { "監査" } else { "sha256" }),
            "{}",
            f.holds()
        );
        assert_eq!(f.called("batch", "accept"), 0);
    }
}
#[test]
fn bad_audit_retries_once_and_bad_drop_never_accepts() {
    for mode in ["missing", "bad-drop"] {
        let f = Fixture::new();
        f.done();
        f.once();
        f.job("p01-b001-audit", "done");
        if mode == "bad-drop" {
            f.audit("accept", "[\"../outside.txt\"]")
        }
        f.once();
        assert!(f.jobs().get("p01-b001-audit-r2").is_some());
        assert_eq!(f.called("batch", "accept"), 0);
        f.job("p01-b001-audit-r2", "done");
        f.once();
        assert!(f.holds().contains("監査"));
    }
}
#[test]
fn failed_resume_once_with_or_without_scratch() {
    for scratch in [false, true] {
        let f = Fixture::new();
        if scratch {
            f.batch("failed", "error", 0, 0)
        } else {
            f.job("p01-b001", "failed")
        }
        f.once();
        assert_eq!(f.called("batch", "resume"), 1);
        let latest = if scratch { "p01-b001" } else { "p01-b001-r2" };
        f.job(latest, "failed");
        f.once();
        assert!(f.holds().contains("再失敗"));
    }
}
#[test]
fn batches_reported_discarded_or_accepted_before_the_ledger_are_finished() {
    for state in ["破棄", "受入済み"] {
        let f = Fixture::new();
        fs::create_dir_all(f.r.join("progress/p01-test")).unwrap();
        fs::write(
            f.r.join("progress/p01-test/b001.result.md"),
            format!("# p01-test b001\n\n状態：{state}\n"),
        )
        .unwrap();
        f.job("p01-b001", "failed");
        f.once();
        assert_eq!(f.called("batch", "resume"), 0, "{state}");
        assert!(!f.holds().contains("b001"), "{state}");
    }
}
#[test]
fn usage_limit_wait_and_live_limit_preserved() {
    let f = Fixture::new();
    f.batch("failed", "利用上限", 0, 0);
    f.once();
    assert_eq!(f.called("batch", "resume"), 0);
    let mut s = f.state();
    s["batches"]["p01-test/b001"]["limit_since"] = json!(
        s["batches"]["p01-test/b001"]["limit_since"]
            .as_f64()
            .unwrap()
            - 1801.
    );
    f.save_state(&s);
    f.once();
    assert_eq!(f.called("batch", "resume"), 1);
    let g = Fixture::new();
    g.batch("failed", "利用上限", 0, 0);
    g.job("p01-b001", "limit");
    g.once();
    assert_eq!(g.called("batch", "resume"), 0);
    assert_eq!(g.called("codex-job", "stop"), 0);
}
#[test]
fn graceful_stop_drains_existing_audits_and_accepts() {
    for before in [false, true] {
        let f = Fixture::new();
        f.done();
        if !before {
            f.once();
            f.audit("accept", "[]");
        }
        f.cli(&["stop"]);
        f.once();
        if before {
            assert!(f.jobs().get("p01-b001-audit").is_some());
            f.audit("accept", "[]");
            f.once();
        }
        assert_eq!(f.called("batch", "accept"), 1);
        assert_eq!(f.called("batch", "start"), 0);
        if before {
            assert!(f.jobs().get("p01-roadmap").is_none());
        }
    }
}
#[test]
fn low_space_and_five_holds_stop_new_admission_but_accept_completed() {
    let mut f = Fixture::new();
    f.roadmap(None);
    f.env.push((
        "AUTOPILOT_MIN_FREE_BYTES".into(),
        "1000000000000000000000000000000".into(),
    ));
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    f.env.pop();
    fs::write(
        f.r.join("progress/holds.md"),
        "| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n".to_string()
            + &(2..7)
                .map(|i| format!("| p{i:02}-test | b001 | 失敗 | now |\n"))
                .collect::<String>(),
    )
    .unwrap();
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    let mut g = Fixture::new();
    g.done();
    g.once();
    g.audit("accept", "[]");
    g.env.push((
        "AUTOPILOT_MIN_FREE_BYTES".into(),
        "1000000000000000000000000000000".into(),
    ));
    g.once();
    assert_eq!(g.called("batch", "accept"), 1);
}
#[test]
fn start_deduplication_and_stop_now_preserves_batches() {
    let f = Fixture::new();
    f.cli(&["start"]);
    f.cli(&["start"]);
    assert_eq!(f.called("codex-job", "start"), 1);
    f.job("p01-b001", "running");
    f.roadmap(None);
    f.cli(&["stop", "--now"]);
    assert!(f.calls().contains(&vec![
        "codex-job".into(),
        "stop".into(),
        "personaCorpus".into(),
        "autopilot".into()
    ]));
    f.once();
    assert_eq!(f.jobs()["p01-b001"], "running");
    assert_eq!(f.called("batch", "start"), 0);
}
#[test]
fn corrupt_state_is_isolated_and_status_keeps_configuration() {
    let f = Fixture::new();
    fs::create_dir_all(f.r.join("scratch/p01-test/b001")).unwrap();
    fs::write(f.r.join("scratch/p01-test/b001/state.json"), "broken").unwrap();
    fs::write(f.r.join("personas/p02-other.md"), "# persona").unwrap();
    f.once();
    assert!(f.jobs().get("p02-roadmap").is_some());
    assert!(
        fs::read_to_string(f.r.join("scratch/autopilot/log.tsv"))
            .unwrap()
            .contains("error")
    );
    let out = f.cli(&["status"]);
    assert!(out.contains("Codex") && out.contains(P) && out.contains("30"));
}
#[test]
fn push_failure_blocks_after_three_then_recovers_without_reaccept() {
    let f = Fixture::new();
    f.git(&[
        "remote",
        "set-url",
        "origin",
        f.r.join("missing").to_str().unwrap(),
    ]);
    f.done();
    f.once();
    f.audit("accept", "[]");
    for _ in 0..3 {
        f.once();
    }
    assert!(f.holds().contains("push"));
    assert!(f.state()["push_failures"].as_i64().unwrap() >= 3);
    f.git(&["remote", "set-url", "origin", f.remote.to_str().unwrap()]);
    f.once();
    assert!(!f.holds().contains("| 全体 | push |"));
    f.once();
    assert_eq!(f.called("batch", "accept"), 1);
    assert_eq!(
        f.git(&["rev-parse", "main"]).trim(),
        f.git(&["ls-remote", "origin", "refs/heads/main"])
            .split_whitespace()
            .next()
            .unwrap()
    );
}
#[test]
fn invalid_roadmap_fields_retry_and_generated_roadmap_uses_next_number() {
    let f = Fixture::new();
    f.roadmap(Some("| b001 | | 30 | 重点 | 内容 | 回避 | |\n"));
    f.once();
    assert!(f.jobs().get("p01-roadmap").is_some());
    f.roadmap(Some("| b001 | 月次 | 20 | 重点 | 内容 | 回避 | |\n"));
    f.job("p01-roadmap", "done");
    f.once();
    assert!(f.jobs().get("p01-roadmap-r2").is_some());
    let g = Fixture::new();
    g.once();
    g.job("p01-roadmap", "done");
    fs::write(
        g.r.join("progress/p01-test/b012.request.md"),
        "# 過去の依頼",
    )
    .unwrap();
    g.roadmap(None);
    g.once();
    assert!(g.r.join("progress/p01-test/b013.request.md").exists());
    assert!(g.calls().iter().any(|c| c.starts_with(&[
        "batch".into(),
        "start".into(),
        P.into(),
        "b013".into()
    ])));
}
#[test]
fn persistent_loop_recovers_after_refresh_failure() {
    let f = Fixture::new();
    fs::write(f.r.join("fail-first-enabled"), "").unwrap();
    let mut c = f.command(&["run"]);
    c.env("AUTOPILOT_INTERVAL_SECONDS", "0.02");
    let mut child = c.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut found = false;
    while Instant::now() < deadline {
        if f.jobs().get("p01-roadmap").is_some() {
            found = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    if !found {
        let _ = child.kill();
        let o = child.wait_with_output().unwrap();
        panic!(
            "roadmap was never launched: {}",
            String::from_utf8_lossy(&o.stderr)
        );
    }
    f.cli(&["start"]);
    assert!(f.jobs().get("autopilot").is_none());
    fs::write(f.r.join("scratch/autopilot/stop-now"), "").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Some(s) = child.try_wait().unwrap() {
            assert!(s.success());
            return;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    child.kill().unwrap();
    panic!("persistent loop failed to stop");
}

#[test]
fn unreadable_manifest_and_runtime_lock_fail_closed() {
    let f = Fixture::new();
    f.roadmap(None);
    fs::write(f.r.join("manifests/p01-test.tsv"), [0xff, 0xfe]).unwrap();
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    assert!(
        fs::read_to_string(f.r.join("scratch/autopilot/log.tsv"))
            .unwrap()
            .contains("error")
    );
    let g = Fixture::new();
    g.done();
    fs::create_dir_all(g.r.join("scratch/p01-test/.runtime-locks/b001.lock")).unwrap();
    g.once();
    assert!(g.jobs().get("p01-b001-audit").is_none());
    assert!(
        fs::read_to_string(g.r.join("scratch/autopilot/log.tsv"))
            .unwrap()
            .contains("error")
    );
}

fn coordinator_briefs(f: &Fixture) -> Vec<String> {
    f.calls()
        .iter()
        .filter(|c| c[0] == "codex-job" && c[1] == "start")
        .filter_map(|c| {
            let n = &c[c.iter().position(|s| s == "--name").unwrap() + 1];
            n.starts_with("coordinator").then(|| {
                fs::read_to_string(f.r.join(format!("scratch/autopilot/jobs/{n}/brief.md")))
                    .unwrap()
            })
        })
        .collect()
}
fn log_lines(f: &Fixture, lines: &[(&str, &str)]) {
    fs::create_dir_all(f.r.join("scratch/autopilot")).unwrap();
    let mut text = fs::read_to_string(f.r.join("scratch/autopilot/log.tsv")).unwrap_or_default();
    for (action, b) in lines {
        text.push_str(&format!(
            "2026-11-18T00:00:00+00:00\t{action}\tp01-test\t{b}\tdetail\n"
        ));
    }
    fs::write(f.r.join("scratch/autopilot/log.tsv"), text).unwrap();
}
fn checks(f: &Fixture, rows: &[(&str, &str)]) {
    let mut text = "persona\tbatch\t判定\t日時\t件数\tメモ\n".to_string();
    for (b, verdict) in rows {
        text.push_str(&format!("p01-test\t{b}\t{verdict}\t2026-11-18\t8\t確認\n"));
    }
    fs::write(f.r.join("progress/checks.tsv"), text).unwrap();
}
#[test]
fn hold_launches_one_coordinator_and_stops_retrying_after_three() {
    let f = Fixture::new();
    fs::write(
        f.r.join("progress/holds.md"),
        "# 保留\n\n| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n| p01-test | b001 | 監査 hold | 2026-11-18 |\n",
    )
    .unwrap();
    f.once();
    f.once();
    let briefs = coordinator_briefs(&f);
    assert_eq!(briefs.len(), 1);
    assert!(briefs[0].starts_with("coordinator instructions"));
    assert!(briefs[0].contains("hold:p01-test/b001"));
    for n in ["coordinator", "coordinator-r2", "coordinator-r3"] {
        f.job(n, "done");
        f.once();
    }
    assert_eq!(coordinator_briefs(&f).len(), 3);
    let log = fs::read_to_string(f.r.join("scratch/autopilot/log.tsv")).unwrap();
    assert!(log.contains("coordinator-stuck"), "{log}");
    assert!(f.cli(&["status"]).contains("hold:p01-test/b001"));
}
#[test]
fn every_accept_is_checked_until_ten_agreements_then_one_in_ten() {
    let f = Fixture::new();
    log_lines(&f, &[("accept", "b001")]);
    f.once();
    assert!(coordinator_briefs(&f)[0].contains("check:p01-test/b001"));
    let f = Fixture::new();
    let agreed: Vec<String> = (1..=10).map(|i| format!("b{i:03}")).collect();
    checks(
        &f,
        &agreed
            .iter()
            .map(|b| (b.as_str(), "一致"))
            .collect::<Vec<_>>(),
    );
    let later: Vec<String> = (11..=19).map(|i| format!("b{i:03}")).collect();
    log_lines(
        &f,
        &agreed
            .iter()
            .chain(&later)
            .map(|b| ("accept", b.as_str()))
            .collect::<Vec<_>>(),
    );
    f.once();
    assert!(coordinator_briefs(&f).is_empty());
    log_lines(&f, &[("accept", "b020")]);
    f.once();
    let brief = &coordinator_briefs(&f)[0];
    assert!(brief.contains("check:p01-test/b020") && brief.contains("check:p01-test/b011"));
    let f = Fixture::new();
    let mut rows: Vec<(&str, &str)> = agreed.iter().map(|b| (b.as_str(), "一致")).collect();
    rows.push(("b011", "不一致"));
    checks(&f, &rows);
    log_lines(&f, &[("accept", "b011"), ("accept", "b012")]);
    f.once();
    let brief = &coordinator_briefs(&f)[0];
    assert!(brief.contains("check:p01-test/b012") && !brief.contains("check:p01-test/b011"));
}
#[test]
fn repeated_errors_launch_the_coordinator() {
    let f = Fixture::new();
    log_lines(&f, &[("error", "b001"), ("error", "b001")]);
    f.once();
    assert!(coordinator_briefs(&f).is_empty());
    log_lines(&f, &[("error", "b001")]);
    f.once();
    assert!(coordinator_briefs(&f)[0].contains("errors:p01-test/b001"));
}
#[test]
fn start_clears_a_graceful_stop_of_a_running_loop() {
    let f = Fixture::new();
    f.job("autopilot", "running");
    fs::create_dir_all(f.r.join("scratch/autopilot")).unwrap();
    fs::write(f.r.join("scratch/autopilot/stop"), "").unwrap();
    f.cli(&["start"]);
    assert!(!f.r.join("scratch/autopilot/stop").exists());
    assert_eq!(f.called("codex-job", "start"), 0);
}
fn finish_coordinator(f: &Fixture, actions: Value) -> String {
    let job = f.state()["coordinator"]["job"]
        .as_str()
        .unwrap()
        .to_string();
    fs::write(
        f.r.join(format!("scratch/autopilot/jobs/{job}/actions.json")),
        serde_json::to_string(&actions).unwrap(),
    )
    .unwrap();
    f.job(&job, "done");
    job
}
fn hold_row(f: &Fixture) {
    fs::write(
        f.r.join("progress/holds.md"),
        "# 保留\n\n| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n| p01-test | b001 | 監査 hold | 2026-11-18 |\n",
    )
    .unwrap();
    f.git(&["add", "progress/holds.md"]);
    f.git(&["commit", "-qm", "hold"]);
}
#[test]
fn coordinator_actions_accept_unhold_and_merge_its_worktree() {
    let f = Fixture::new();
    hold_row(&f);
    f.once();
    let wt = f.r.join("scratch/autopilot/wt");
    assert_eq!(
        f.git(&["-C", wt.to_str().unwrap(), "rev-parse", "HEAD"]),
        f.git(&["rev-parse", "main"])
    );
    fs::write(wt.join("prompts/audit.md"), "audit instructions, stricter").unwrap();
    fs::write(
        wt.join("progress/checks.tsv"),
        "persona\tbatch\t判定\t日時\t件数\tメモ\np01-test\tb001\t一致\t2026-11-18\t8\t確認\n",
    )
    .unwrap();
    finish_coordinator(
        &f,
        json!({"actions":[
            {"do":"accept","persona":"p01-test","batch":"b001","drop":["ok.txt"]},
            {"do":"unhold","persona":"p01-test","batch":"b001"},
            {"do":"merge","message":"Tighten the audit"},
            {"do":"start"}
        ]}),
    );
    fs::write(f.r.join("scratch/autopilot/stop"), "").unwrap();
    f.once();
    assert!(!f.r.join("scratch/autopilot/stop").exists());
    assert!(f.calls().iter().any(|c| c[0] == "batch"
        && c[1] == "accept"
        && c.ends_with(&["--drop".into(), "ok.txt".into()])));
    assert!(!f.holds().contains("| b001 |"));
    assert_eq!(
        fs::read_to_string(f.r.join("prompts/audit.md")).unwrap(),
        "audit instructions, stricter"
    );
    assert!(f.git(&["log", "--format=%s"]).contains("Tighten the audit"));
    assert_eq!(
        f.git(&["rev-parse", "main"]).trim(),
        f.git(&["ls-remote", "origin", "refs/heads/main"])
            .split_whitespace()
            .next()
            .unwrap()
    );
    assert!(f.cli(&["status"]).contains("一致: 1"));
}
#[test]
fn coordinator_merge_refuses_protected_paths_and_failing_tests() {
    for case in ["spec", "tests"] {
        let f = Fixture::new();
        hold_row(&f);
        f.once();
        let wt = f.r.join("scratch/autopilot/wt");
        if case == "spec" {
            fs::write(wt.join("SPEC.md"), "changed").unwrap();
        } else {
            fs::create_dir_all(wt.join("tests")).unwrap();
            fs::create_dir_all(wt.join("tools/src")).unwrap();
            fs::write(wt.join("tools/src/x.rs"), "// change").unwrap();
            fs::write(wt.join("tests/run.sh"), "#!/bin/sh\nexit 1\n").unwrap();
            fs::set_permissions(wt.join("tests/run.sh"), fs::Permissions::from_mode(0o755))
                .unwrap();
        }
        finish_coordinator(&f, json!({"actions":[{"do":"merge","message":"bad"}]}));
        let before = f.git(&["rev-parse", "main"]);
        f.once();
        assert_eq!(f.git(&["rev-parse", "main"]), before, "{case}");
        let state = f.state();
        let result = state["coordinator"]["last_result"].to_string();
        assert!(
            result.contains(if case == "spec" {
                "SPEC.md"
            } else {
                "tests/run.sh"
            }),
            "{result}"
        );
        let brief = coordinator_briefs(&f).pop().unwrap();
        assert!(brief.contains("\"previous\""), "{brief}");
    }
}
