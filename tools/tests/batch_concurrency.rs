mod batch_support;
use batch_support::*;
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    fs,
    fs::{File, OpenOptions},
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Output, Stdio},
    thread,
    time::{Duration, Instant},
};
fn jobs(f: &Fixture, states: &[(u32, &str)]) {
    write(
        &f.root.join("jobs.tsv"),
        states
            .iter()
            .map(|(i, s)| format!("personaCorpus\tp01-b{i:03}\t{s}\tdetail\n"))
            .collect::<String>(),
    );
}
fn predecessor(f: &Fixture, commit: bool) {
    write(
        &f.root.join("progress/batches.tsv"),
        "persona\tbatch\tstatus\np01-test\tb001\taccepted\n",
    );
    if commit {
        f.git(&["add", "progress/batches.tsv"]);
        f.git(&["commit", "-qm", "Accepted predecessor"]);
    }
}
fn request(f: &Fixture, b: &str) {
    write(
        &f.root.join(format!("progress/{P}/{b}.request.md")),
        "# 依頼\n件数の目安：2\n",
    );
}
fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}
fn guard(path: &Path) -> File {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    FileExt::lock_exclusive(&file).unwrap();
    file
}
fn completed(mut child: Child) -> Output {
    let end = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= end {
            let _ = child.kill();
            let o = child.wait_with_output().unwrap();
            panic!(
                "command did not finish: {}{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
        thread::sleep(Duration::from_millis(10));
    }
    let o = child.wait_with_output().unwrap();
    assert!(
        o.status.success(),
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    o
}
#[test]
fn live_states_consume_capacity_and_default_rejects_fourth() {
    for states in [
        vec![
            (1, "failed"),
            (2, "done"),
            (3, "stalled"),
            (4, "limit"),
            (5, "stdin-wait"),
        ],
        vec![(2, "running"), (3, "running"), (4, "running")],
    ] {
        let f = Fixture::new();
        predecessor(&f, true);
        request(&f, "b006");
        jobs(&f, &states);
        let o = f.cli(&["start", "p01", "b006"], "ok", false);
        assert!(stderr(&o).contains("上限: 3"), "{}", stderr(&o));
    }
}
#[test]
fn second_batch_requires_committed_predecessor_and_configured_limit() {
    let mut f = Fixture::new();
    request(&f, "b003");
    jobs(&f, &[(2, "running")]);
    let o = f.cli(&["start", "p01", "b003"], "ok", false);
    assert!(stderr(&o).contains("受入済み"));
    predecessor(&f, false);
    let o = f.cli(&["start", "p01", "b003"], "ok", false);
    assert!(stderr(&o).contains("受入済み"));
    f.git(&["add", "progress/batches.tsv"]);
    f.git(&["commit", "-qm", "Accept predecessor"]);
    f.cli(&["start", "p01", "b003"], "ok", true);
    f.env
        .insert("CORPUS_MAX_BATCHES_PER_PERSONA".into(), "1".into());
    let o = f.cli(&["start", "p01", "b003"], "ok", false);
    assert!(stderr(&o).contains("上限: 1"));
}
#[test]
fn runtime_lock_reserves_capacity_and_prevents_duplicate() {
    let f = Fixture::new();
    request(&f, "b002");
    request(&f, "b003");
    jobs(&f, &[]);
    let lock = guard(&f.root.join(format!("scratch/{P}/.runtime-locks/b002.lock")));
    let o = f.cli(&["start", "p01", "b002"], "ok", false);
    assert!(stderr(&o).contains("同じ batch"));
    let o = f.cli(&["start", "p01", "b003"], "ok", false);
    assert!(stderr(&o).contains("受入済み"));
    FileExt::unlock(&lock).unwrap();
    f.cli(&["start", "p01", "b003"], "ok", true);
}
fn blocked(f: &Fixture, path: &Path, args: &[&str]) {
    let lock = guard(path);
    let mut c = f.command(args, "ok");
    c.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    thread::sleep(Duration::from_millis(300));
    assert!(
        child.try_wait().unwrap().is_none(),
        "command bypassed repository/persona lock"
    );
    FileExt::unlock(&lock).unwrap();
    completed(child);
}
#[test]
fn acceptance_and_planning_wait_for_repository_and_persona_locks() {
    let f = Fixture::new();
    f.run("ok", true);
    blocked(
        &f,
        &f.root.join("scratch/.locks/records.lock"),
        &["accept", "p01", "b001"],
    );
    assert!(
        f.git(&["log", "-1", "--format=%s"])
            .contains("Accept p01-test b001")
    );
    let g = Fixture::new();
    blocked(
        &g,
        &g.root.join(format!("scratch/{P}/.lock")),
        &["run", "p01", "b001", "--plan-only"],
    );
    assert_eq!(g.state()["status"], "planned");
}
fn parallel(f: &Fixture) -> Vec<Value> {
    f.planned();
    let first = f.rows()[0].clone();
    let rows = (0..23)
        .map(|i| {
            let mut r = first.clone();
            r["パス"] = json!(format!("Documents/連絡-{i}.txt"));
            r
        })
        .collect::<Vec<_>>();
    f.set_plan(&rows, true);
    write(&f.root.join("events-enabled"), "");
    rows
}
fn events(f: &Fixture, stage: &str, kind: &str) -> Vec<Value> {
    f.lines("events.jsonl")
        .into_iter()
        .filter(|e| e[2] == stage && e[0] == kind)
        .collect()
}
#[test]
fn real_groups_obey_global_limit_and_review_parallelism() {
    let mut f = Fixture::new();
    parallel(&f);
    f.env.insert("CORPUS_MAX_CODEX".into(), "2".into());
    f.run("ok", true);
    let mut active = 0;
    let mut maximum = 0;
    for e in f.lines("events.jsonl") {
        active += if e[0] == "start" { 1 } else { -1 };
        maximum = maximum.max(active);
        assert!(active >= 0);
    }
    assert_eq!(active, 0);
    assert_eq!(maximum, 2);
    let review = f
        .lines("events.jsonl")
        .into_iter()
        .filter(|e| e[2] == "draft-review")
        .collect::<Vec<_>>();
    assert_eq!(review.iter().filter(|e| e[0] == "start").count(), 3);
    assert_eq!(review[0][0], "start");
    assert_eq!(review[1][0], "start");
    assert!(review[1][4].as_i64().unwrap() < review[2][4].as_i64().unwrap());
    let work = f
        .lines("contexts.jsonl")
        .into_iter()
        .filter(|v| ["draft", "draft-review"].contains(&string(&v["stage"])))
        .map(|v| string(&v["work"]).to_string())
        .collect::<Vec<_>>();
    assert_eq!(work.len(), work.iter().collect::<BTreeSet<_>>().len());
}
#[test]
fn failed_group_joins_peers_and_resume_does_not_repeat_success() {
    let f = Fixture::new();
    parallel(&f);
    f.run("parallel-fail", false);
    let event_rows = f.lines("events.jsonl");
    assert_eq!(
        event_rows.iter().filter(|e| e[0] == "start").count(),
        event_rows.iter().filter(|e| e[0] == "end").count()
    );
    let state = f.state();
    assert_eq!(
        state["completed"]
            .as_object()
            .unwrap()
            .keys()
            .filter(|k| k.starts_with("draft-r0-"))
            .count(),
        2
    );
    f.run("ok", true);
    let starts = events(&f, "draft", "start");
    for rel in ["連絡-10.txt", "連絡-20.txt"] {
        assert_eq!(
            starts
                .iter()
                .filter(|e| string(&e[3][0]["パス"]).ends_with(rel))
                .count(),
            1
        );
    }
}
#[test]
fn dependency_layers_review_before_children_and_review_again_keeps_drafts() {
    let f = Fixture::new();
    let mut rows = parallel(&f);
    rows.truncate(3);
    rows[2]["先に要るもの"] = rows[0]["パス"].clone();
    f.set_plan(&rows, true);
    f.run("ok", true);
    let starts = events(&f, "draft", "start");
    assert_eq!(
        starts
            .iter()
            .map(|e| e[3].as_array().unwrap().len())
            .collect::<Vec<_>>(),
        vec![2, 1]
    );
    let first_review_end = events(&f, "draft-review", "end")[0][4].as_i64().unwrap();
    assert!(first_review_end < starts[1][4].as_i64().unwrap());
    f.cli(&["run", "p01", "b001", "--draft-review-again"], "ok", true);
    assert_eq!(events(&f, "draft", "start").len(), 2);
}
#[test]
fn rejected_parent_still_drafts_all_descendants() {
    let f = Fixture::new();
    let mut rows = parallel(&f);
    rows.truncate(3);
    rows[1]["先に要るもの"] = rows[0]["パス"].clone();
    rows[2]["先に要るもの"] = rows[1]["パス"].clone();
    f.set_plan(&rows, true);
    f.run("draft-reject-always", true);
    assert!(
        f.state()["draft_failures"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v != "先に要る原稿が不合格")
    );
    assert_eq!(
        events(&f, "draft", "start")
            .iter()
            .map(|e| string(&e[3][0]["パス"]).to_string())
            .collect::<BTreeSet<_>>(),
        rows.iter()
            .map(|r| string(&r["パス"]).to_string())
            .collect()
    );
}
#[test]
fn closed_plan_releases_withdrawn_paths_and_keeps_pending_paths_reserved() {
    let f = Fixture::new();
    f.planned();
    let mut rows = f.rows();
    let path = string(&rows[0]["パス"]).to_owned();
    let other = f.plan().with_file_name("b002.md");
    write(
        &other,
        read(&f.plan()).replace(string(&rows[0]["内容"]), "破棄：統括が撤回"),
    );
    let state = f.root.join(format!("scratch/{P}/b002/state.json"));
    write(&state, r#"{"status":"accepted"}"#);
    rows.truncate(1);
    f.set_plan(&rows, false);
    f.reset_scratch();
    f.planned();
    write(&f.source(&path), "保存済みの原稿");
    f.reset_scratch();
    f.cli(&["run", "p01", "b001", "--plan-only"], "ok", false);
    assert!(string(&f.state()["reason"]).contains(&format!("既存または重複パス: {path}")));
    fs::remove_file(f.source(&path)).unwrap();
    for reserved in [
        json!({"status":"failed"}),
        json!({"status":"accepted", "draft_failures":{(path.clone()):"原稿不合格"}}),
    ] {
        write(&state, reserved.to_string());
        f.reset_scratch();
        f.cli(&["run", "p01", "b001", "--plan-only"], "ok", false);
        assert!(
            string(&f.state()["reason"]).contains(&format!("進行中の batch と重複パス: {path}"))
        );
    }
    write(&state, r#"{"status":"accepted"}"#);
    write(
        &other,
        read(&other).replace("破棄：統括が撤回", "成形待ち：図の重なり"),
    );
    f.reset_scratch();
    f.cli(&["run", "p01", "b001", "--plan-only"], "ok", false);
    assert!(string(&f.state()["reason"]).contains(&format!("進行中の batch と重複パス: {path}")));
}
#[test]
fn overlapping_plans_and_invalid_dependency_graphs_are_rejected() {
    let f = Fixture::new();
    f.planned();
    write(&f.plan().with_file_name("b002.md"), read(&f.plan()));
    write(
        &f.root.join(format!("scratch/{P}/b002/state.json")),
        "{\"status\":\"running\"}",
    );
    f.reset_scratch();
    f.run("ok", false);
    assert!(string(&f.state()["reason"]).contains("進行中の batch と重複パス"));
    let g = Fixture::new();
    g.planned();
    let mut rows = g.rows();
    assert_eq!(rows[0]["先に要るもの"], "");
    g.reset_scratch();
    rows[0]["先に要るもの"] = json!("Documents/外.txt");
    g.set_plan(&rows, false);
    g.run("ok", false);
    assert!(string(&g.state()["reason"]).contains("同じ batch"));
    g.reset_scratch();
    rows[0]["先に要るもの"] = rows[1]["パス"].clone();
    rows[1]["先に要るもの"] = rows[0]["パス"].clone();
    g.set_plan(&rows, false);
    g.run("ok", false);
    assert!(string(&g.state()["reason"]).contains("循環"));
}
#[test]
fn production_cannot_change_shared_metadata_after_planning() {
    let f = Fixture::new();
    parallel(&f);
    f.run("metadata-mutate", false);
    assert!(string(&f.state()["reason"]).contains("型・台帳・persona"));
}
#[test]
fn auxiliary_waits_for_slot_and_preserves_astra_cwd_prompt() {
    let mut f = Fixture::new();
    f.env.insert("CORPUS_MAX_CODEX".into(), "1".into());
    let brief = f.root.join("scratch/brief.md");
    let output = f.root.join("scratch/auxiliary-output.json");
    let actual_cwd = f.root.join("scratch/auxiliary-cwd.txt");
    let fake = f.root.join("fake-codex");
    let script = read(&fake).replacen(
        "#!/bin/sh\n",
        &format!(
            "#!/bin/sh\npwd > {}\n",
            shell_words::quote(actual_cwd.to_str().unwrap())
        ),
        1,
    );
    executable(&fake, &script);
    write(&brief, "audit instruction");
    let slot = guard(&f.root.join("scratch/.slots/0"));
    let mut c = f.command(
        &[
            "codex-once",
            "--brief",
            brief.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ],
        "ok",
    );
    c.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    thread::sleep(Duration::from_millis(250));
    assert!(child.try_wait().unwrap().is_none());
    assert!(!output.exists());
    FileExt::unlock(&slot).unwrap();
    completed(child);
    let data = json_file(&output);
    let args = data["args"].as_array().unwrap();
    let flag = |s: &str| {
        args.iter()
            .position(|v| v == s)
            .map(|i| args[i + 1].as_str().unwrap())
    };
    assert_eq!(flag("-m"), Some("gpt-6.2-astra"));
    assert_eq!(flag("-C"), f.root.to_str());
    assert_eq!(data["prompt"], "audit instruction");
    assert_eq!(read(&actual_cwd).trim(), f.root.to_str().unwrap());
    assert!(
        args.iter()
            .any(|s| s == "model_reasoning_effort=\"medium\"")
    );
}
#[test]
fn auxiliary_process_group_death_releases_global_slot() {
    let mut f = Fixture::new();
    f.env.insert("CORPUS_MAX_CODEX".into(), "1".into());
    let brief = f.root.join("scratch/brief.md");
    let output = f.root.join("scratch/auxiliary-output.json");
    write(&brief, "auxiliary instruction");
    let mut c = f.command(
        &[
            "codex-once",
            "--brief",
            brief.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ],
        "hang",
    );
    c.process_group(0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().unwrap();
    let end = Instant::now() + Duration::from_secs(5);
    while !f.root.join("scratch/ready").exists() && Instant::now() < end {
        thread::sleep(Duration::from_millis(10));
    }
    assert!(f.root.join("scratch/ready").exists());
    let slot = OpenOptions::new()
        .create(true)
        .append(true)
        .open(f.root.join("scratch/.slots/0"))
        .unwrap();
    assert!(slot.try_lock_exclusive().is_err());
    unsafe {
        assert_eq!(libc::kill(-(child.id() as i32), libc::SIGKILL), 0);
    }
    child.wait().unwrap();
    let end = Instant::now() + Duration::from_secs(5);
    loop {
        if slot.try_lock_exclusive().is_ok() {
            break;
        }
        assert!(
            Instant::now() < end,
            "slot not released after process group death"
        );
        thread::sleep(Duration::from_millis(10));
    }
}
