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
        let sessions = r
            .join("scratch/codex-sessions")
            .to_string_lossy()
            .into_owned();
        let f = Self {
            _tmp: tmp,
            r,
            remote,
            env: vec![
                ("AUTOPILOT_MIN_FREE_BYTES".into(), "0".into()),
                // Never read the real Codex session logs in tests.
                ("AUTOPILOT_CODEX_SESSIONS".into(), sessions),
            ],
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
fn small_reproduction_row_starts_with_its_actual_count() {
    let f = Fixture::new();
    f.roadmap(Some(
        "| b001 | 再制作：欠品5件 | 5 | 回収条件 | 表と返信 | 既受入分 | |\n| b002 | 次の仕事 | 30 | 重点 | 内容 | 回避 | |\n",
    ));
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
    let request = fs::read_to_string(f.r.join("progress/p01-test/b001.request.md")).unwrap();
    assert!(request.contains("件数の目安：5"));
}

#[test]
fn invalid_roadmap_fields_retry_and_generated_roadmap_uses_next_number() {
    let f = Fixture::new();
    f.roadmap(Some("| b001 | | 30 | 重点 | 内容 | 回避 | |\n"));
    f.once();
    assert!(f.jobs().get("p01-roadmap").is_some());
    f.roadmap(Some(
        "| b001 | 月次 | 5 | 重点 | 内容 | 回避 | |\n| b002 | 次の仕事 | 30 | 重点 | 内容 | 回避 | |\n",
    ));
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
fn coordinator_unholds_roadmap_and_keeps_batch_hold() {
    let f = Fixture::new();
    hold_row(&f);
    let holds = f.r.join("progress/holds.md");
    fs::write(
        &holds,
        format!("{}| p01-test | roadmap | 規模 | 2026-11-19 |\n", f.holds()),
    )
    .unwrap();
    f.once();
    finish_coordinator(
        &f,
        json!({"actions":[{"do":"unhold","persona":"p01-test","batch":"roadmap"}]}),
    );
    f.once();
    assert!(!f.holds().contains("| roadmap |"));
    assert!(f.holds().contains("| b001 |"));
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
fn failed_coordinator_accept_preserves_the_roadmap_hold_and_push_state() {
    for push_pending in [false, true] {
        let f = Fixture::new();
        hold_row(&f);
        f.git(&["push", "-q", "origin", "main"]);
        f.roadmap(Some(
            "| b001 | 月次 | 30 | 表 | メール | 重複 | 進行中 b001 |\n",
        ));
        f.done();
        f.once();
        let roadmap = f.r.join("progress/p01-test/roadmap.md");
        let original = fs::read_to_string(&roadmap).unwrap();
        if push_pending {
            f.git(&["commit", "--allow-empty", "-qm", "Existing pending commit"]);
            let mut state = f.state();
            state["push_pending"] = json!(true);
            f.save_state(&state);
        }
        f.git(&[
            "remote",
            "set-url",
            "origin",
            f.r.join("missing").to_str().unwrap(),
        ]);
        fs::write(f.r.join("accept-fails"), "").unwrap();
        finish_coordinator(
            &f,
            json!({"actions":[
                {"do":"accept","persona":"p01-test","batch":"b001","drop":["ok.txt"]},
                {"do":"unhold","persona":"p01-test","batch":"b001"},
            ]}),
        );
        f.once();

        assert_eq!(f.called("batch", "accept"), 1);
        assert_eq!(fs::read_to_string(&roadmap).unwrap(), original);
        assert!(f.holds().contains("| p01-test | b001 | 監査 hold |"));
        let state = f.state();
        assert_ne!(state["batches"]["p01-test/b001"]["accepted"], true);
        assert_eq!(
            state["push_pending"].as_bool().unwrap_or(false),
            push_pending
        );
        let results = state["coordinator"]["last_result"].as_array().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].as_str().unwrap().contains("後続の操作を中止"));
    }
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
        finish_coordinator(
            &f,
            json!({"actions":[{"do":"merge","message":"bad"},{"do":"unhold","persona":"p01-test","batch":"b001"}]}),
        );
        let before = f.git(&["rev-parse", "main"]);
        f.once();
        assert_eq!(f.git(&["rev-parse", "main"]), before, "{case}");
        let state = f.state();
        assert!(f.holds().contains("| b001 |"));
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
#[test]
fn coordinator_can_leave_an_item_for_the_user_and_gets_the_answer() {
    let f = Fixture::new();
    hold_row(&f);
    f.once();
    assert_eq!(coordinator_briefs(&f).len(), 1);
    finish_coordinator(
        &f,
        json!({"actions":[
            {"do":"ask","item":"hold:p01-test/b001","question":"14 件を撤回して 22,000 枚で作り直すか"}
        ]}),
    );
    for _ in 0..4 {
        f.once();
    }
    assert_eq!(
        coordinator_briefs(&f).len(),
        1,
        "an item waiting for the user is not relaunched"
    );
    let status = f.cli(&["status"]);
    assert!(status.contains("ユーザーの判断待ち"), "{status}");
    assert!(status.contains("22,000 枚で作り直すか"), "{status}");
    assert!(
        !status.contains("3 回で片付かず手が要るもの: hold"),
        "{status}"
    );
    f.cli(&["answer", "hold:p01-test/b001", "22,000 枚で作り直す"]);
    f.once();
    let briefs = coordinator_briefs(&f);
    assert_eq!(briefs.len(), 2);
    let brief = briefs.last().unwrap();
    assert!(brief.contains("\"answers\""), "{brief}");
    assert!(brief.contains("22,000 枚で作り直す"), "{brief}");
    assert!(!f.cli(&["status"]).contains("ユーザーの判断待ち"));
}
#[test]
fn an_answer_can_allow_the_next_merge_to_delete_more_than_three_files() {
    let f = Fixture::new();
    fs::create_dir_all(f.r.join("people/p01-test")).unwrap();
    for i in 0..5 {
        fs::write(f.r.join(format!("people/p01-test/old{i}.txt")), "x").unwrap();
    }
    f.git(&["add", "people"]);
    f.git(&["commit", "-qm", "files"]);
    hold_row(&f);
    f.once();
    finish_coordinator(
        &f,
        json!({"actions":[{"do":"ask","item":"hold:p01-test/b001","question":"5 件を撤回するか"}]}),
    );
    f.once();
    f.cli(&[
        "answer",
        "hold:p01-test/b001",
        "撤回する",
        "--allow-delete",
        "5",
    ]);
    f.once();
    let wt = f.r.join("scratch/autopilot/wt");
    for i in 0..5 {
        fs::remove_file(wt.join(format!("people/p01-test/old{i}.txt"))).unwrap();
    }
    finish_coordinator(
        &f,
        json!({"actions":[{"do":"merge","message":"Withdraw five"}]}),
    );
    f.once();
    let result = f.state()["coordinator"]["last_result"].to_string();
    assert!(!result.contains("取り込めない変更"), "{result}");
    assert!(
        result.contains("manifest"),
        "the next gate is reached: {result}"
    );
    assert!(
        f.r.join("scratch/autopilot/allow-delete").exists(),
        "kept until a merge succeeds"
    );
}
fn limit_failure(f: &Fixture, job: &str) {
    let dir = f.r.join(format!("ops/jobs/personaCorpus/{job}"));
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("log"),
        "ERROR: You've hit your usage limit. Visit https://example/usage or try again at 8:35 PM.\n",
    )
    .unwrap();
    f.job(job, "failed");
}
fn expire_limit(f: &Fixture) {
    let mut s = f.state();
    s["limit_until"] = json!(0.0);
    atomic(&f.r.join("scratch/autopilot/state.json"), &s);
}
#[test]
fn usage_limit_pauses_the_coordinator_without_using_its_attempts() {
    let f = Fixture::new();
    hold_row(&f);
    f.once();
    for round in 0..4 {
        let job = f.state()["coordinator"]["job"]
            .as_str()
            .unwrap()
            .to_string();
        limit_failure(&f, &job);
        f.once();
        f.once();
        assert_eq!(
            coordinator_briefs(&f).len(),
            round + 1,
            "paused while limited"
        );
        assert!(f.cli(&["status"]).contains("利用上限"));
        expire_limit(&f);
        f.once();
    }
    assert_eq!(coordinator_briefs(&f).len(), 5);
    assert!(!f.cli(&["status"]).contains("手が要るもの: hold"));
}
#[test]
fn usage_limit_does_not_turn_a_failed_audit_into_a_hold() {
    let f = Fixture::new();
    f.done();
    f.once();
    for _ in 0..3 {
        let job = f.state()["batches"]["p01-test/b001"]["audit"]["job"]
            .as_str()
            .unwrap()
            .to_string();
        limit_failure(&f, &job);
        f.once();
        let until = f.state()["limit_until"].clone();
        f.once();
        assert_eq!(
            f.state()["limit_until"],
            until,
            "one cooldown per failed job"
        );
        expire_limit(&f);
        f.once();
        assert_ne!(
            f.state()["batches"]["p01-test/b001"]["audit"]["job"],
            json!(job)
        );
    }
    assert!(!f.holds().contains("| b001 |"), "{}", f.holds());
}
#[test]
fn a_stale_limit_message_in_the_tail_does_not_hide_a_plan_failure() {
    let f = Fixture::new();
    f.batch(
        "failed",
        "計画不合格: 進行中の batch と重複パス: a.txt",
        0,
        0,
    );
    let p = f.r.join("scratch/p01-test/b001/state.json");
    let mut s: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    s["failure_tail"] = json!("ERROR: You've hit your usage limit. try again at 8:35 PM.");
    atomic(&p, &s);
    f.once();
    assert_eq!(
        f.called("batch", "resume"),
        1,
        "a plan failure is resumed once"
    );
    f.job("p01-b001", "failed");
    f.once();
    assert!(
        f.holds().contains("再失敗"),
        "then held for the coordinator: {}",
        f.holds()
    );
}

#[test]
fn coordinator_revalidates_when_main_advances_during_validation() {
    for second_check_succeeds in [true, false] {
        let f = Fixture::new();
        hold_row(&f);
        f.once();
        let wt = f.r.join("scratch/autopilot/wt");
        fs::create_dir_all(wt.join("tests")).unwrap();
        let script = wt.join("tests/run.sh");
        let second_exit = u8::from(!second_check_succeeds);
        fs::write(
            &script,
            format!(
                "#!/bin/sh\nset -eu\n\
                 if [ -f advance-marker ]; then\n\
                   touch \"$CORPUS_BASE/validated-new-main\"\n\
                   exit {second_exit}\n\
                 fi\n\
                 touch \"$CORPUS_BASE/advance-marker\"\n\
                 git -C \"$CORPUS_BASE\" add advance-marker\n\
                 git -C \"$CORPUS_BASE\" commit -qm 'Advance main during validation'\n"
            ),
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        finish_coordinator(
            &f,
            json!({"actions":[
                {"do":"merge","message":"Coordinator change"},
                {"do":"unhold","persona":"p01-test","batch":"b001"}
            ]}),
        );
        f.once();

        let result = f.state()["coordinator"]["last_result"].to_string();
        assert!(f.r.join("validated-new-main").exists(), "{result}");
        assert!(
            f.git(&["log", "--format=%s"])
                .contains("Advance main during validation")
        );
        assert_eq!(
            f.r.join("tests/run.sh").exists(),
            second_check_succeeds,
            "{result}"
        );
        assert_eq!(
            f.holds().contains("| b001 |"),
            !second_check_succeeds,
            "{result}"
        );
        if !second_check_succeeds {
            assert!(result.contains("tests/run.sh が失敗した"), "{result}");
        }
    }
}

#[test]
fn coordinator_verifies_candidate_manifest_against_retained_accepted_files() {
    for update_manifest in [false, true] {
        let f = Fixture::new();
        let people = f.r.join("people/p01-test");
        fs::create_dir_all(&people).unwrap();
        for name in ["remove.txt", "keep.txt"] {
            let file = people.join(name);
            fs::write(&file, name).unwrap();
            filetime::set_file_mtime(&file, filetime::FileTime::from_unix_time(1_700_000_000, 0))
                .unwrap();
        }
        fs::write(f.r.join("bin/manifest"), include_str!("../../bin/manifest")).unwrap();
        fs::set_permissions(f.r.join("bin/manifest"), fs::Permissions::from_mode(0o755)).unwrap();
        let release = f.r.join("tools/target/release");
        fs::create_dir_all(&release).unwrap();
        std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_corpus"), release.join("corpus")).unwrap();
        f.git(&["add", "people", "bin/manifest"]);
        f.git(&["commit", "-qm", "Add accepted files"]);
        good(
            &Command::new(f.r.join("bin/manifest"))
                .arg(P)
                .output()
                .unwrap(),
        );
        let manifest = "manifests/p01-test.tsv";
        f.git(&["add", manifest]);
        f.git(&["commit", "-qm", "Add valid manifest"]);
        hold_row(&f);
        f.once();

        let wt = f.r.join("scratch/autopilot/wt");
        fs::remove_file(wt.join("people/p01-test/remove.txt")).unwrap();
        let original = fs::read_to_string(f.r.join(manifest)).unwrap();
        let reduced = original
            .lines()
            .find(|l| l.starts_with("keep.txt\t"))
            .unwrap();
        if update_manifest {
            fs::write(wt.join(manifest), format!("{reduced}\n")).unwrap();
        }
        finish_coordinator(
            &f,
            json!({"actions":[{"do":"merge","message":"Remove an accepted file"}]}),
        );
        f.once();

        let result = f.state()["coordinator"]["last_result"].to_string();
        assert_eq!(
            !people.join("remove.txt").exists(),
            update_manifest,
            "{result}"
        );
        if !update_manifest {
            assert!(result.contains("manifest の照合に通らない"), "{result}");
        }
        good(
            &Command::new(f.r.join("bin/manifest"))
                .args([P, "--verify"])
                .output()
                .unwrap(),
        );
    }
}

#[test]
fn rebuild_after_audit_requires_a_new_audit() {
    let f = Fixture::new();
    f.done();
    f.once();
    f.audit("hold", "[]");
    let mut log = fs::OpenOptions::new()
        .append(true)
        .open(f.r.join("scratch/autopilot/log.tsv"))
        .unwrap();
    writeln!(
        log,
        "2026-11-19T01:01:00Z\tresume\tp01-test\tb001\t--rebuild"
    )
    .unwrap();
    fs::write(f.r.join("people/p01-test/ok.txt"), "rebuilt").unwrap();
    f.once();
    assert!(f.jobs().get("p01-b001-audit-r2").is_some());
    assert_eq!(f.called("batch", "accept"), 0);
}

fn brief_context(f: &Fixture, job: &str) -> Value {
    let brief =
        fs::read_to_string(f.r.join(format!("scratch/autopilot/jobs/{job}/brief.md"))).unwrap();
    serde_json::from_str(brief.rsplit("AUTOPILOT_CONTEXT\n").next().unwrap().trim()).unwrap()
}

#[test]
fn a_used_up_roadmap_is_extended_and_must_keep_its_rows() {
    let f = Fixture::new();
    f.roadmap(Some(
        "| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | 受入済み b001 |\n",
    ));
    fs::write(
        f.r.join("manifests/p01-test.tsv"),
        "x\thash\t1\tdate\n".repeat(10),
    )
    .unwrap();
    f.once();
    assert!(f.jobs().get("p01-roadmap").is_some(), "{:?}", f.jobs());
    let context = brief_context(&f, "p01-roadmap");
    assert_eq!(context["target"], 30);
    assert_eq!(context["accepted"], 10);
    assert!(
        f.r.join("scratch/autopilot/jobs/p01-roadmap/previous.md")
            .exists()
    );
    // A rewrite that loses the finished row is rejected.
    f.roadmap(Some("| b002 | 四半期締め | 30 | 重点 | 内容 | 回避 | |\n"));
    f.job("p01-roadmap", "done");
    f.once();
    assert!(f.jobs().get("p01-roadmap-r2").is_some());
    assert_eq!(f.called("batch", "start"), 0);
    f.roadmap(Some(
        "| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | 受入済み b001 |\n| b002 | 四半期締め | 30 | 重点 | 内容 | 回避 | |\n",
    ));
    f.job("p01-roadmap-r2", "done");
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
}

#[test]
fn every_persona_works_at_once_and_plans_ahead_in_large_chunks() {
    let f = Fixture::new();
    fs::create_dir_all(f.r.join("config")).unwrap();
    fs::write(f.r.join("config/limits.json"), r#"{"roadmap_chunk":50}"#).unwrap();
    for (p, n) in [
        ("p01-test", 30),
        ("p02-test", 0),
        ("p03-test", 5),
        ("p04-test", 0),
    ] {
        fs::write(f.r.join(format!("personas/{p}.md")), "# test").unwrap();
        fs::create_dir_all(f.r.join(format!("progress/{p}"))).unwrap();
        fs::write(
            f.r.join(format!("manifests/{p}.tsv")),
            "x\thash\t1\tdate\n".repeat(n),
        )
        .unwrap();
    }
    f.cli(&["once", "--target", "200"]);
    let jobs = f.jobs();
    for p in ["p01", "p02", "p03", "p04"] {
        assert!(
            jobs.get(format!("{p}-roadmap").as_str()).is_some(),
            "{jobs:?}"
        );
    }
    // The roadmap plans the next chunk, not the whole scale, and personas are not
    // held back to the slowest one.
    assert_eq!(brief_context(&f, "p01-roadmap")["target"], 80);
    assert_eq!(brief_context(&f, "p02-roadmap")["target"], 50);
    assert_eq!(brief_context(&f, "p01-roadmap")["final_target"], 200);
    // A roadmap whose open rows cover the chunk is valid even far below the scale.
    fs::write(
        f.r.join("progress/p01-test/roadmap.md"),
        format!("{HEADER}| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | 受入済み b001 |\n| b002 | 四半期 | 30 | 重点 | 内容 | 回避 | |\n| b003 | 年度 | 30 | 重点 | 内容 | 回避 | |\n"),
    )
    .unwrap();
    f.job("p01-roadmap", "done");
    f.cli(&["once", "--target", "200"]);
    assert!(f.jobs().get("p01-roadmap-r2").is_none(), "{:?}", f.jobs());
    assert!(f.calls().iter().any(|c| c.starts_with(&[
        "batch".into(),
        "start".into(),
        "p01-test".into()
    ])));
    assert!(
        f.cli(&["status", "--target", "200"])
            .contains("p02-test\t0/200")
    );
}

#[test]
fn each_persona_stops_at_the_scale_in_its_definition() {
    let f = Fixture::new();
    fs::write(
        f.r.join("personas/p01-test.md"),
        "# test\n\n## 規模の目安\n\n全体で 40 件。最初の区切りは 30 件で、月次締めを中心にする。\n\n## 書かないもの\n",
    )
    .unwrap();
    fs::write(f.r.join("personas/p02-test.md"), "# test").unwrap();
    fs::create_dir_all(f.r.join("progress/p02-test")).unwrap();
    for p in ["p01-test", "p02-test"] {
        fs::write(
            f.r.join(format!("manifests/{p}.tsv")),
            "x\thash\t1\tdate\n".repeat(31),
        )
        .unwrap();
    }
    let status = f.cli(&["status", "--target", "90"]);
    assert!(status.contains("p01-test\t31/40"), "{status}");
    assert!(status.contains("p02-test\t31/90"), "{status}");
    fs::write(
        f.r.join("manifests/p01-test.tsv"),
        "x\thash\t1\tdate\n".repeat(40),
    )
    .unwrap();
    f.cli(&["once", "--target", "90"]);
    assert!(f.jobs().get("p01-roadmap").is_none());
    assert!(f.jobs().get("p02-roadmap").is_some());
}

#[test]
fn a_roadmap_that_asks_to_review_the_scale_is_held_at_once() {
    let f = Fixture::new();
    f.once();
    f.roadmap(Some("| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | |\n"));
    let path = f.r.join("progress/p01-test/roadmap.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{text}\n規模の見直し：定義の出来事で固有に作れるのは残り 10 件\n"),
    )
    .unwrap();
    f.job("p01-roadmap", "done");
    f.cli(&["once", "--target", "100"]);
    assert!(f.holds().contains("規模の見直し"), "{}", f.holds());
    assert!(f.jobs().get("p01-roadmap-r2").is_none());
    assert_eq!(f.called("batch", "start"), 0);
}

#[test]
fn a_stop_left_by_a_coordinator_is_cleared_when_its_actions_are_applied() {
    let f = Fixture::new();
    hold_row(&f);
    f.once();
    fs::write(f.r.join("scratch/autopilot/stop"), "").unwrap();
    finish_coordinator(&f, json!({"actions":[]}));
    f.once();
    assert!(!f.r.join("scratch/autopilot/stop").exists());
    let log = fs::read_to_string(f.r.join("scratch/autopilot/log.tsv")).unwrap();
    assert!(log.contains("coordinator-stop-cleared"), "{log}");
    // A stop made before the coordinator started (by the user) is kept.
    let g = Fixture::new();
    hold_row(&g);
    fs::create_dir_all(g.r.join("scratch/autopilot")).unwrap();
    fs::write(g.r.join("scratch/autopilot/stop"), "").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(20));
    g.once();
    if g.state()["coordinator"]["job"].as_str().is_some() {
        finish_coordinator(&g, json!({"actions":[]}));
        g.once();
    }
    assert!(g.r.join("scratch/autopilot/stop").exists());
}

#[test]
fn a_gap_smaller_than_one_batch_ends_the_goal_instead_of_holding() {
    // 297 of 300 planned: the remaining 3 cannot fill a batch, so the roadmap is
    // complete for this goal, with or without a scale-review note.
    let f = Fixture::new();
    fs::write(
        f.r.join("manifests/p01-test.tsv"),
        "x\thash\t1\tdate\n".repeat(36),
    )
    .unwrap();
    f.roadmap(Some("| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | 受入済み b001 |\n| b002 | 四半期 | 31 | 重点 | 内容 | 回避 | |\n"));
    let path = f.r.join("progress/p01-test/roadmap.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{text}\n規模の見直し：残り 3 件のために組める仕事が尽きた\n"),
    )
    .unwrap();
    f.cli(&["once", "--target", "70"]);
    assert!(!f.holds().contains("規模の見直し"), "{}", f.holds());
    assert_eq!(f.called("batch", "start"), 1);
    // A large gap is still held for the coordinator.
    let g = Fixture::new();
    g.roadmap(Some("| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | |\n"));
    let path = g.r.join("progress/p01-test/roadmap.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{text}\n規模の見直し：残り 70 件分の仕事が尽きた\n"),
    )
    .unwrap();
    g.cli(&["once", "--target", "100"]);
    assert!(g.holds().contains("規模の見直し"), "{}", g.holds());
}

#[test]
fn a_shortfall_under_one_batch_does_not_invalidate_a_roadmap_with_open_rows() {
    let f = Fixture::new();
    f.roadmap(Some("| b001 | 月次締め | 30 | 重点 | 内容 | 回避 | |\n| b002 | 四半期 | 35 | 重点 | 内容 | 回避 | |\n"));
    f.cli(&["once", "--target", "70"]);
    assert!(f.jobs().get("p01-roadmap").is_none(), "{:?}", f.jobs());
    assert_eq!(f.called("batch", "start"), 1);
}

#[test]
fn rows_of_a_running_batch_count_toward_the_goal() {
    let f = Fixture::new();
    f.roadmap(Some("| b001 | 月次 | 35 | 重点 | 内容 | 回避 | 進行中 b001 |\n| b002 | 四半期 | 30 | 重点 | 内容 | 回避 | |\n"));
    let path = f.r.join("progress/p01-test/roadmap.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!("{text}\n規模の見直し：残り 5 件のための行は足さない\n"),
    )
    .unwrap();
    f.batch("running", "", 0, 0);
    f.cli(&["once", "--target", "70"]);
    assert!(!f.holds().contains("規模の見直し"), "{}", f.holds());
}

#[test]
fn the_hold_limit_scales_with_the_number_of_personas() {
    let f = Fixture::new();
    for i in 2..=12 {
        fs::write(f.r.join(format!("personas/p{i:02}-test.md")), "# test").unwrap();
    }
    let rows: String = (1..=5)
        .map(|i| format!("| p{i:02}-test | b001 | 監査 hold | 2026-11-19 |\n"))
        .collect();
    fs::write(
        f.r.join("progress/holds.md"),
        format!("# 保留\n\n| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n{rows}"),
    )
    .unwrap();
    let status = f.cli(&["status", "--target", "30"]);
    assert!(status.contains("止める条件: なし"), "{status}");
    let rows: String = (1..=6)
        .map(|i| format!("| p{i:02}-test | b001 | 監査 hold | 2026-11-19 |\n"))
        .collect();
    fs::write(
        f.r.join("progress/holds.md"),
        format!("# 保留\n\n| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n{rows}"),
    )
    .unwrap();
    let status = f.cli(&["status", "--target", "30"]);
    assert!(status.contains("保留6件以上"), "{status}");
}

fn codex_usage(f: &Fixture, used: f64, resets_at: i64) -> PathBuf {
    let dir = f.r.join("scratch/codex-sessions/2026/11/19");
    fs::create_dir_all(&dir).unwrap();
    let line = json!({"timestamp":"2026-11-19T12:00:00.000Z","type":"event_msg","payload":{"type":"token_count","rate_limits":{"primary":{"used_percent":used,"window_minutes":10080,"resets_at":resets_at},"secondary":null}}});
    fs::write(
        dir.join("rollout-test.jsonl"),
        format!("{{\"timestamp\":\"2026-11-19T11:00:00.000Z\",\"type\":\"other\"}}\n{line}\n"),
    )
    .unwrap();
    f.r.join("scratch/codex-sessions")
}

#[test]
fn a_usage_threshold_stops_new_batches_until_the_window_resets() {
    let mut f = Fixture::new();
    let sessions = codex_usage(&f, 92.0, 4_000_000_000);
    f.env.push((
        "AUTOPILOT_CODEX_SESSIONS".into(),
        sessions.to_string_lossy().into_owned(),
    ));
    f.cli(&["usage", "--stop-at", "90"]);
    f.roadmap(None);
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    let status = f.cli(&["status", "--target", "30"]);
    assert!(
        status.contains("利用枠") && status.contains("92"),
        "{status}"
    );
    // Once the window has reset, the stale snapshot no longer stops anything.
    codex_usage(&f, 95.0, 1_000);
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
    // Below the threshold, work continues; --off removes the threshold.
    let g = Fixture::new();
    g.cli(&["usage", "--stop-at", "90"]);
    g.cli(&["usage", "--off"]);
    assert!(!g.r.join("scratch/autopilot/usage-stop").exists());
}

#[test]
fn the_next_stage_starts_when_every_persona_is_within_a_batch_of_its_goal() {
    let f = Fixture::new();
    fs::write(f.r.join("personas/p02-test.md"), "# test").unwrap();
    fs::create_dir_all(f.r.join("progress/p02-test")).unwrap();
    fs::write(
        f.r.join("manifests/p01-test.tsv"),
        "x\thash\t1\tdate\n".repeat(100),
    )
    .unwrap();
    fs::write(
        f.r.join("manifests/p02-test.tsv"),
        "x\thash\t1\tdate\n".repeat(60),
    )
    .unwrap();
    f.cli(&["next", "200"]);
    f.cli(&["once", "--target", "100"]);
    assert_eq!(f.state()["configuration"]["target"], 100);
    fs::write(
        f.r.join("manifests/p02-test.tsv"),
        "x\thash\t1\tdate\n".repeat(75),
    )
    .unwrap();
    f.cli(&["once", "--target", "100"]);
    assert_eq!(f.state()["configuration"]["target"], 200);
    assert!(f.state()["next_target"].is_null());
    let log = fs::read_to_string(f.r.join("scratch/autopilot/log.tsv")).unwrap();
    assert!(log.contains("phase-advance"), "{log}");
    // Starting again without --target keeps the advanced target.
    f.cli(&["start"]);
    assert!(
        f.calls()
            .iter()
            .any(|c| c[0] == "codex-job" && c.iter().any(|a| a.contains("--target 200"))),
        "{:?}",
        f.calls()
    );
}

fn old_usage(f: &Fixture, used: f64) -> PathBuf {
    let dir = f.r.join("scratch/codex-sessions/2026/01/01");
    fs::create_dir_all(&dir).unwrap();
    let line = json!({"timestamp":"2026-01-01T00:00:00.000Z","type":"event_msg","payload":{"type":"token_count","rate_limits":{"primary":{"used_percent":used,"window_minutes":10080,"resets_at":4_000_000_000i64}}}});
    fs::write(dir.join("rollout-old.jsonl"), format!("{line}\n")).unwrap();
    f.r.join("scratch/codex-sessions")
}

#[test]
fn switching_the_codex_account_ignores_the_old_accounts_usage() {
    let f = Fixture::new();
    old_usage(&f, 95.0);
    f.cli(&["usage", "--stop-at", "90"]);
    f.roadmap(None);
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    // Logging in to another account rewrites auth.json next to the sessions.
    fs::write(f.r.join("scratch/auth.json"), "{}").unwrap();
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
}

#[test]
fn a_recheck_ignores_usage_recorded_before_it() {
    let f = Fixture::new();
    old_usage(&f, 95.0);
    f.cli(&["usage", "--stop-at", "90"]);
    f.roadmap(None);
    f.once();
    assert_eq!(f.called("batch", "start"), 0);
    let out = f.cli(&["usage", "--recheck"]);
    assert!(out.contains("記録待ち"), "{out}");
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
}

#[test]
fn sessions_started_under_the_old_login_are_ignored_even_when_they_write_later() {
    let f = Fixture::new();
    f.cli(&["usage", "--stop-at", "90"]);
    f.roadmap(None);
    fs::write(f.r.join("scratch/auth.json"), "{}").unwrap();
    let dir = f.r.join("scratch/codex-sessions/2026/11/19");
    fs::create_dir_all(&dir).unwrap();
    let limits = |at: &str, used: f64| json!({"timestamp":at,"type":"event_msg","payload":{"type":"token_count","rate_limits":{"primary":{"used_percent":used,"window_minutes":10080,"resets_at":4_000_000_000i64}}}});
    // A session that began before the login change keeps reporting the old account.
    fs::write(
        dir.join("rollout-old.jsonl"),
        format!(
            "{}\n{}\n",
            json!({"timestamp":"2026-01-01T00:00:00.000Z","type":"session_meta"}),
            limits("2031-01-01T00:00:00.000Z", 95.0)
        ),
    )
    .unwrap();
    fs::write(
        dir.join("rollout-new.jsonl"),
        format!(
            "{}\n{}\n",
            json!({"timestamp":"2030-01-01T00:00:00.000Z","type":"session_meta"}),
            limits("2030-01-01T00:01:00.000Z", 1.0)
        ),
    )
    .unwrap();
    f.once();
    assert_eq!(f.called("batch", "start"), 1);
    assert!(f.cli(&["usage"]).contains("週 1%"));
}

#[test]
fn an_account_switch_ends_the_usage_limit_wait_and_drops_old_readings() {
    let f = Fixture::new();
    f.roadmap(None);
    fs::write(f.r.join("scratch/auth.json"), "{}").unwrap();
    f.once();
    // The first account hit its limit: new Codex jobs wait.
    let mut state = f.state();
    state["limit_until"] = json!(4_000_000_000.0);
    f.save_state(&state);
    f.cli(&["usage", "--recheck"]);
    assert!(f.cli(&["status", "--target", "30"]).contains("利用上限"));
    // Logging in to another account resumes at once.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    fs::write(f.r.join("scratch/auth.json"), "{\"other\":1}").unwrap();
    f.once();
    assert!(f.state()["limit_until"].as_f64().unwrap_or(0.0) < 1.0);
    assert!(!f.r.join("scratch/autopilot/usage-since").exists());
    let log = fs::read_to_string(f.r.join("scratch/autopilot/log.tsv")).unwrap();
    assert!(log.contains("account-switch"), "{log}");
    assert!(!f.cli(&["status", "--target", "30"]).contains("利用上限"));
}
