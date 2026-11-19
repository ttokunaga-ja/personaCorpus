mod batch_support;
use batch_support::*;
use serde_json::{Value, json};
use std::{fs, path::Path};
fn stdout(o: &std::process::Output) -> String {
    String::from_utf8_lossy(&o.stdout).into()
}
fn count_files(p: &Path) -> usize {
    walkdir::WalkDir::new(p)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .count()
}
#[test]
fn happy_models_dates_and_complete_report() {
    let f = Fixture::new();
    f.run("ok", true);
    let s = f.state();
    assert_eq!(s["status"], "done");
    assert_eq!(s["count"], 2);
    assert_eq!(
        f.calls(),
        json!({"templates":1,"templates-review":1,"plan":1,"draft":1,"draft-review":1,"build":1,"build-review":1})
    );
    assert!(f.scratch().join("review-r0.tsv").exists());
    let argv = f.lines("argv.jsonl");
    let models: Vec<_> = argv
        .iter()
        .map(|a| {
            let a = a.as_array().unwrap();
            a[a.iter().position(|v| v == "-m").unwrap() + 1].clone()
        })
        .collect();
    assert_eq!(
        models,
        json!([
            "gpt-6.2-sol",
            "gpt-6.2-sol",
            "gpt-6.2-sol",
            "gpt-6.1-terra",
            "gpt-6.2-sol",
            "gpt-6.1-terra",
            "gpt-6.2-sol"
        ])
        .as_array()
        .unwrap()
        .clone()
    );
    assert_eq!(
        filetime::FileTime::from_last_modification_time(&fs::metadata(f.artifact(FIRST)).unwrap())
            .unix_seconds(),
        1772766000
    );
    let report = read(&f.root.join(format!("progress/{P}/b001.result.md")));
    for t in ["原稿", "成形", "原稿の所要時間：", "成形の所要時間："] {
        assert!(report.contains(t));
    }
}
#[test]
fn producer_override_and_medium_workspace_flags() {
    let f = Fixture::new();
    f.cli(
        &["run", "p01", "b001", "--producer", "gpt-6.2-sol"],
        "ok",
        true,
    );
    for args in f.lines("argv.jsonl") {
        let a = args.as_array().unwrap();
        assert_eq!(
            a[a.iter().position(|v| v == "-m").unwrap() + 1],
            "gpt-6.2-sol"
        );
        assert!(a.contains(&json!("model_reasoning_effort=\"medium\"")));
        assert!(a.contains(&json!("workspace-write")));
        assert_eq!(a.last().unwrap(), "-");
    }
}
#[test]
fn plan_only_resumes_without_replanning() {
    let f = Fixture::new();
    f.planned();
    assert_eq!(count_files(&f.root.join(format!("people/{P}"))), 0);
    f.run("ok", true);
    assert_eq!(f.calls()["plan"], 1);
}
#[test]
fn invalid_plan_retried_once_and_stops() {
    let f = Fixture::new();
    f.run("badplan", true);
    assert_eq!(f.calls()["plan"], 2);
    let f = Fixture::new();
    f.run("badplan-always", false);
    assert_eq!(f.state()["stage"], "plan");
    assert_eq!(f.calls()["plan"], 2);
}
#[test]
fn plan_dates_to_the_minute_are_accepted() {
    let f = Fixture::new();
    f.planned();
    let plan = read(&f.plan());
    assert!(plan.contains(" | 2026-03-06 | "));
    f.reset_scratch();
    write(
        &f.root.join("fixture-plan.md"),
        plan.replacen(" | 2026-03-06 | ", " | 2026-03-06 18:42 | ", 1),
    );
    f.run("ok", true);
    let state = f.state();
    assert!(!string(&state["reason"]).contains("premature"), "{state}");
    assert_ne!(state["stage"], "plan", "{state}");
}
#[test]
fn plan_path_date_method_and_existing_file_guards() {
    for (old, new) in [
        (FIRST, "../外.txt"),
        (FIRST, "Documents//連絡.txt"),
        (FIRST, SECOND),
        (FIRST, "Documents/.隠し.txt"),
        (FIRST, "Documents/full.txt"),
        (FIRST, "Documents/batch.txt"),
        ("2026-03-06", "2027-03-06"),
        (FIRST, "Documents/記録.docx"),
        (" | text | ", " | 不明 | "),
    ] {
        let f = Fixture::new();
        f.planned();
        let original = read(&f.plan());
        f.reset_scratch();
        write(&f.root.join("fixture-plan.md"), original.replace(old, new));
        f.run("ok", false);
        assert_eq!(f.state()["stage"], "plan");
    }
    let f = Fixture::new();
    f.planned();
    let plan = read(&f.plan());
    f.reset_scratch();
    write(&f.root.join("fixture-plan.md"), plan);
    write(&f.artifact(FIRST), "既存");
    f.run("ok", false);
    assert!(string(&f.state()["reason"]).contains("既存"));
}
#[test]
fn machine_and_review_rejections_retry_only_failed_rows() {
    for (mode, stage) in [
        ("machine-once", "draft"),
        ("draft-reject-once", "draft"),
        ("reject-once", "build"),
        ("auto-reject", "build"),
    ] {
        let f = Fixture::new();
        f.run(mode, true);
        assert_eq!(f.state()["count"], 2);
        assert_eq!(f.calls()[stage], 2);
        if stage == "build" {
            assert_eq!(f.calls()["draft"], 1);
        } else {
            assert_eq!(f.calls()["build"], 1);
        }
    }
}
#[test]
fn draft_rejection_never_builds_and_build_rejection_keeps_source() {
    let f = Fixture::new();
    f.run("draft-reject-always", true);
    assert_eq!(f.calls()["draft"], 3);
    assert!(!f.artifact(FIRST).exists());
    assert!(!f.source(FIRST).exists());
    assert!(f.artifact(SECOND).exists());
    let f = Fixture::new();
    f.run("reject-always", true);
    assert_eq!(f.state()["rebuilds"], 2);
    assert!(!f.artifact(FIRST).exists());
    assert!(f.source(FIRST).exists());
    assert!(read(&f.plan()).contains("成形待ち：数値不整合。"));
}
#[test]
fn draft_frontmatter_empty_trace_missing_path_and_kind_contract() {
    let body = "資金調達の条件について経理部長が確認し、銀行への返済期限を変更する方針を決めた。";
    for (v, reason) in [
        (json!({"body":""}), "本文が空"),
        (json!({"body":"placeholder"}), "制作の痕跡"),
        (
            json!({"body":body,"metadata":{"kind":"other"}}),
            "frontmatter",
        ),
        (
            json!({"body":body,"metadata":{"path":"wrong.txt"}}),
            "frontmatter",
        ),
        (json!({"raw":body}), "frontmatter"),
        (Value::Null, "No such file"),
    ] {
        let f = Fixture::new();
        f.overrides(json!({FIRST:v}));
        f.run("ok", true);
        assert_eq!(f.state()["draft_failures"].as_object().unwrap().len(), 1);
        assert!(string(&f.state()["draft_failures"][FIRST]).contains(reason));
        assert!(f.artifact(SECOND).exists());
    }
}
#[test]
fn japanese_production_note_rejected_but_ordinary_annotation_allowed() {
    for body in [
        "終了区分の扱いを直した。\n【河野隆之、2026年7月10日 11:40、削除・取り消し線】期限到来で削除完了。",
        "添付：受領控え.pdf（保存先：Documents/引渡し。本文への埋め込みなし）",
    ] {
        let f = Fixture::new();
        f.overrides(json!({FIRST:{"body":"【重要】申請書の保存先：共有フォルダ。部長の赤字を反映して版を上げた。"},SECOND:{"body":body}}));
        f.run("ok", true);
        assert_eq!(
            f.state()["draft_failures"]
                .as_object()
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            vec![SECOND]
        );
    }
}
#[test]
fn related_versions_exempt_normalized_duplicates_but_not_identical_bytes() {
    for (related, same, expected) in [(false, false, 2), (true, false, 0), (true, true, 2)] {
        let f = Fixture::new();
        f.planned();
        let mut rows = f.rows();
        if related {
            rows[1]["関連"] = json!(FIRST);
        }
        f.set_plan(&rows, true);
        let body = |n: &str| {
            format!(
                "入金予定の確認結果を記録。銀行照会と取引先回答を照合して入金額{n}円を翌営業日に計上する。"
            )
        };
        f.overrides(
            json!({FIRST:{"body":body("100")},SECOND:{"body":body(if same{"100"}else{"200"})}}),
        );
        f.run("ok", true);
        let state = f.state();
        let failures = state["draft_failures"].as_object().unwrap();
        assert_eq!(failures.len(), expected);
        for (p, r) in failures {
            assert_ne!(string(r).rsplit(": ").next().unwrap(), p);
        }
    }
}
#[test]
fn derived_differences_do_not_count_as_duplicate_full_documents() {
    let f = Fixture::new();
    write(
        &f.source("Documents/元.txt"),
        "---\npath: Documents/元.txt\nkind: text\nauthor: 花子\nderived_from:\n---\n元となる文書の中身",
    );
    write(&f.artifact("Documents/元.txt"), "元となる文書の中身");
    f.overrides(json!({FIRST:{"body":"違いなし","metadata":{"derived_from":"Documents/元.txt"}},SECOND:{"body":"違いなし","metadata":{"derived_from":"Documents/元.txt"}}}));
    f.run("ok", true);
    assert_eq!(f.state()["draft_failures"], json!({}));
    assert_eq!(f.state()["pending"], 0);
}
#[test]
fn dependency_parent_finishes_review_before_child_and_failed_parent_blocks_build() {
    let f = Fixture::new();
    f.run("dependency", true);
    let contexts = f.lines("contexts.jsonl");
    let builds = f.contexts("build");
    assert_eq!(
        builds
            .iter()
            .map(|c| string(&c["rows"][0]["パス"]))
            .collect::<Vec<_>>(),
        vec![FIRST, SECOND]
    );
    let review = contexts
        .iter()
        .position(|c| c["stage"] == "build-review")
        .unwrap();
    let child = contexts
        .iter()
        .position(|c| c["stage"] == "build" && c["rows"][0]["パス"] == SECOND)
        .unwrap();
    assert!(review < child);
    let f = Fixture::new();
    f.run("parent-fail", true);
    assert!(f.contexts("build").iter().all(|c| {
        c["rows"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["パス"] != SECOND)
    }));
    assert_eq!(f.state()["pending"], 2);
    assert!(read(&f.plan()).contains(&format!("元のファイル {FIRST} が未完成。")));
    assert_eq!(count_files(&f.root.join(format!("sources/{P}"))), 2);
    assert!(!read(&f.root.join("progress/findings.tsv")).contains("\tdependency\t"));
}
#[test]
fn draft_parent_rejection_still_writes_child_with_missing_prerequisite_context() {
    let f = Fixture::new();
    f.planned();
    let mut rows = f.rows();
    rows[1]["先に要るもの"] = json!(FIRST);
    f.set_plan(&rows, true);
    f.run("draft-parent-fail", true);
    assert_eq!(f.state()["draft_failures"].as_object().unwrap().len(), 1);
    let child = f
        .contexts("draft")
        .into_iter()
        .find(|c| c["rows"][0]["パス"] == SECOND)
        .unwrap();
    assert_eq!(child["missing_prerequisites"][SECOND], json!([FIRST]));
    assert!(
        read(&f.root.join(format!("progress/{P}/b001.result.md")))
            .split("## 先に要る原稿が欠けたまま書いた行")
            .nth(1)
            .unwrap()
            .contains(SECOND)
    );
}
#[test]
fn derived_missing_parent_waits_at_build() {
    let f = Fixture::new();
    f.planned();
    let mut rows = f.rows();
    rows[1]["先に要るもの"] = json!(FIRST);
    f.set_plan(&rows, true);
    f.overrides(json!({SECOND:{"body":"銀行照会を終えて明日の入金額を確認した。返済期限は翌月末とする。","metadata":{"derived_from":FIRST}}}));
    f.run("draft-parent-fail", true);
    assert!(string(&f.state()["failures"][SECOND]).contains("が未完成"));
}
#[test]
fn review_and_build_source_hash_guards() {
    for mode in [
        "mutate",
        "mutate-draft",
        "mutate-build",
        "mutate-build-review-source",
    ] {
        let f = Fixture::new();
        f.run(mode, false);
        assert!(string(&f.state()["reason"]).contains("変更"));
    }
}
#[test]
fn missing_review_rejected_and_re_review_keeps_manuscripts() {
    let f = Fixture::new();
    f.run("missing-review", true);
    assert!(read(&f.plan()).contains("判定なし"));
    let f = Fixture::new();
    f.run("ok", true);
    let before = fs::read(f.source(FIRST)).unwrap();
    f.cli(
        &["run", "p01", "b001", "--review-again"],
        "reject-once",
        true,
    );
    assert_eq!(fs::read(f.source(FIRST)).unwrap(), before);
    assert_eq!(f.calls()["draft"], 1);
}
#[test]
fn re_review_retains_previous_verdicts_and_produces_no_new_artifacts() {
    let f = Fixture::new();
    f.run("ok", true);
    f.cli(&["run", "p01", "b001", "--review-again"], "ok", true);
    assert_eq!(f.calls()["build"], 1);
    assert_eq!(f.calls()["build-review"], 2);
    let review = f.contexts("build-review").pop().unwrap();
    assert_eq!(review["previous_verdicts"].as_object().unwrap().len(), 2);
    assert!(
        review["previous_verdicts"]
            .as_object()
            .unwrap()
            .values()
            .all(|v| v[0] == "pass")
    );
    assert_eq!(review["reasons"], json!({}));
    let f = Fixture::new();
    f.run("ok", true);
    f.cli(&["run", "p01", "b001", "--draft-review-again"], "ok", true);
    assert_eq!(f.calls()["draft"], 1);
    assert_eq!(f.calls()["draft-review"], 2);
}
#[test]
fn rejected_review_previous_reason_forwarded_and_notes_once() {
    let f = Fixture::new();
    f.run("reject-once", true);
    let review = f
        .contexts("build-review")
        .into_iter()
        .find(|c| c["round"] == 1)
        .unwrap();
    assert_eq!(review["previous_verdicts"][FIRST][0], "reject");
    assert_eq!(review["reasons"][FIRST], "数値不整合");
    assert_eq!(
        read(&f.root.join(format!("progress/{P}/b001.result.md")))
            .matches("全体所見：途中のメモも含め自然。")
            .count(),
        3
    );
    let f = Fixture::new();
    f.run("draft-reject-once", true);
    let review = f
        .contexts("draft-review")
        .into_iter()
        .find(|c| c["round"] == 1)
        .unwrap();
    assert_eq!(review["previous_verdicts"][FIRST][0], "reject");
    assert_eq!(review["reasons"][FIRST], "数値不整合");
}
#[test]
fn process_failure_private_logs_and_resume_without_replanning() {
    let f = Fixture::new();
    let out = f.cli(&["run", "p01", "b001"], "fail", false);
    assert!(!stdout(&out).contains("retry tomorrow"));
    assert_eq!(f.state()["reason"], "利用上限");
    assert!(string(&f.state()["failure_tail"]).contains("retry tomorrow"));
    f.run("ok", true);
    let calls = f.calls();
    assert_eq!(calls["plan"], 1);
    assert_eq!(calls["draft"], 1);
    assert_eq!(calls["draft-review"], 1);
    assert_eq!(calls["build"], 3);
}
#[test]
fn finished_run_is_idempotent_preserving_discard_report() {
    let f = Fixture::new();
    f.run("reject-always", true);
    let report = f.root.join(format!("progress/{P}/b001.result.md"));
    let before = fs::read(&report).unwrap();
    let calls = f.calls();
    f.run("ok", true);
    assert_eq!(fs::read(&report).unwrap(), before);
    assert_eq!(f.calls(), calls);
    f.cli(&["run", "p01", "b001", "--review-again"], "ok", true);
    assert!(read(&report).contains("数値不整合"));
    assert_eq!(f.state()["count"], 1);
}
#[test]
fn final_check_failure_resumes_without_reproducing_or_losing_discards() {
    for mode in ["final-fail", "draft-final-fail", "build-final-fail"] {
        let f = Fixture::new();
        f.run(mode, false);
        assert_eq!(f.state()["stage"], "final-check");
        let calls = f.calls();
        f.run("ok", true);
        assert_eq!(f.calls(), calls);
        assert_eq!(f.state()["count"], if mode == "final-fail" { 2 } else { 1 });
        if mode != "final-fail" {
            assert_eq!(f.source(FIRST).exists(), mode == "build-final-fail");
        }
    }
}
#[test]
fn pending_rebuild_preserves_original_source_and_supports_accept_redo() {
    for accept_first in [false, true] {
        let f = Fixture::new();
        f.run("reject-always", true);
        let before = fs::read(f.source(FIRST)).unwrap();
        if accept_first {
            f.accepted();
            let accepted = f.state();
            f.run("ok", true);
            assert_eq!(f.state(), accepted);
        }
        f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
        assert_eq!(fs::read(f.source(FIRST)).unwrap(), before);
        assert!(!read(&f.plan()).contains("成形待ち："));
        assert!(f.artifact(FIRST).exists());
        f.accepted();
        assert!(!f.scratch().exists());
    }
}
#[test]
fn accepted_redo_does_not_reaccept_already_committed_paths() {
    let f = Fixture::new();
    f.run("reject-always", true);
    f.accepted();
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    let redone = f.state();
    f.accepted();
    fs::create_dir_all(f.scratch()).unwrap();
    f.save(&redone);
    let out = f.cli(&["accept", "p01", "b001"], "ok", false);
    assert!(String::from_utf8_lossy(&out.stderr).contains("受入するものがない"));
}
#[test]
fn unaccepted_rebuild_can_drop_rows_built_earlier() {
    let f = Fixture::new();
    f.run("reject-always", true);
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    f.cli(&["accept", "p01", "b001", "--drop", SECOND], "ok", true);
    assert!(!f.artifact(SECOND).exists());
    let manifest = read(&f.root.join(format!("manifests/{P}.tsv")));
    assert!(manifest.contains(FIRST) && !manifest.contains(SECOND));
}
#[test]
fn accepted_failed_rebuild_new_generation_and_final_check_resume() {
    let f = Fixture::new();
    f.run("reject-always", true);
    f.accepted();
    f.cli(&["run", "p01", "b001", "--rebuild"], "reject-always", true);
    f.accepted();
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    assert_eq!(f.state()["pending"], 0);
    assert!(f.artifact(FIRST).is_file());
    let f = Fixture::new();
    f.run("reject-always", true);
    f.cli(&["run", "p01", "b001", "--rebuild"], "final-fail", false);
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    assert_eq!(f.state()["status"], "done");
    assert!(!read(&f.plan()).contains("成形待ち："));
}
#[test]
fn redraft_only_failed_drafts_preserves_other_work_and_accepted_ledger_recovery() {
    for accepted in [false, true] {
        let f = Fixture::new();
        f.run("draft-reject-always", true);
        let before = fs::read(f.source(SECOND)).unwrap();
        let artifact = fs::read(f.artifact(SECOND)).unwrap();
        if accepted {
            f.accepted();
            assert!(!f.scratch().join("state.json").exists());
        } else {
            let mut s = f.state();
            s["status"] = json!("failed");
            f.save(&s);
        }
        f.cli(&["run", "p01", "b001", "--redraft"], "ok", true);
        assert_eq!(f.state()["status"], "done");
        assert_eq!(f.state()["redraft_paths"], json!([FIRST]));
        assert_eq!(fs::read(f.source(SECOND)).unwrap(), before);
        assert_eq!(fs::read(f.artifact(SECOND)).unwrap(), artifact);
        assert!(!read(&f.plan()).contains("破棄："));
        if accepted {
            f.accepted();
        }
    }
}
#[test]
fn redraft_resumes_after_build_failure_without_new_generation() {
    let f = Fixture::new();
    f.run("draft-parent-fail", true);
    f.cli(&["run", "p01", "b001", "--redraft"], "fail", false);
    assert_eq!(f.state()["redraft_active"], true);
    f.cli(&["run", "p01", "b001", "--redraft"], "ok", true);
    assert_eq!(f.state()["redraft_active"], false);
    assert_eq!(f.state()["redraft_generation"], 1);
    assert_eq!(f.state()["status"], "done");
}
#[test]
fn redraft_excludes_coordinator_and_unknown_drops() {
    let f = Fixture::new();
    f.run("draft-reject-always", true);
    let plan = read(&f.plan())
        .replace("破棄：数値不整合。", "破棄：統括の抜き取りで不合格。")
        .replace(
            "| 案件の判断と相手への連絡。 |",
            "| 破棄：理由不明。案件の判断と相手への連絡。 |",
        );
    write(&f.plan(), &plan);
    let out = f.cli(&["run", "p01", "b001", "--redraft"], "ok", true);
    assert_eq!(read(&f.plan()), plan);
    assert!(stdout(&out).contains(SECOND));
    assert!(f.state().get("redraft_paths").is_none());
}
#[test]
fn redraft_preserves_unrelated_build_pending_artifact_and_source() {
    let f = Fixture::new();
    f.run("draft-parent-fail", true);
    let plan = read(&f.plan()).replace(
        "| 案件の判断と相手への連絡。 |",
        "| 成形待ち：既存の成形待ち。案件の判断と相手への連絡。 |",
    );
    write(&f.plan(), plan);
    let mut s = f.state();
    s["failures"][SECOND] = json!("既存の成形待ち");
    f.save(&s);
    let before = fs::read(f.source(SECOND)).unwrap();
    let artifact = fs::read(f.artifact(SECOND)).unwrap();
    f.cli(&["run", "p01", "b001", "--redraft"], "ok", true);
    assert_eq!(fs::read(f.source(SECOND)).unwrap(), before);
    assert_eq!(fs::read(f.artifact(SECOND)).unwrap(), artifact);
    assert!(read(&f.plan()).contains("成形待ち：既存の成形待ち。"));
}
#[test]
fn acceptance_scopes_batch_files_roadmap_and_drops_only_selected_rows() {
    let f = Fixture::new();
    f.run("ok", true);
    write(
        &f.root.join(format!("progress/{P}/roadmap.md")),
        "受入済み b001\n",
    );
    write(&f.root.join("progress/holds.md"), "無関係な保留\n");
    let judgment = f.root.join(format!("progress/{P}/b001.judgment.md"));
    let section = "## 統括の判断（監査 hold の処理）\n\n確認した図面と数値。";
    write(&judgment, section);
    let original = read(&f.root.join(format!("progress/{P}/b001.result.md")));
    f.cli(&["accept", "p01", "b001", "--drop", FIRST], "ok", true);
    let report = read(&f.root.join(format!("progress/{P}/b001.result.md")));
    assert!(report.ends_with(&format!("{section}\n")));
    assert!(report.contains(original.lines().next().unwrap()));
    assert!(!judgment.exists());
    assert!(!f.scratch().exists());
    assert!(!f.artifact(FIRST).exists());
    assert!(!f.source(FIRST).exists());
    assert!(read(&f.root.join(format!("manifests/{P}.tsv"))).contains(SECOND));
    assert!(
        f.git(&["log", "-1", "--format=%s"])
            .contains("Accept p01-test b001: 月次報告 (1 files)")
    );
    let names = f.git(&["show", "--format=", "--name-only", "HEAD"]);
    assert!(names.contains(&format!("progress/{P}/roadmap.md")));
    assert!(!names.contains("progress/holds.md"));
    assert_eq!(
        f.git(&["show", "HEAD:progress/p01-test/roadmap.md"]),
        "受入済み b001\n"
    );
    assert!(stdout(&f.cli(&["status", "p01"], "ok", true)).contains("受入済み"));
}
#[test]
fn pending_accept_commits_source_and_preserves_waiting_status() {
    let f = Fixture::new();
    f.run("reject-always", true);
    assert!(stdout(&f.cli(&["status", "p01", "b001"], "ok", true)).contains("成形待ち1"));
    f.accepted();
    let out = stdout(&f.cli(&["status", "p01", "b001"], "ok", true));
    assert!(out.contains("成形待ち1"));
    assert!(out.contains("受入済み"));
    assert!(
        f.git(&["show", "--format=", "--name-only", "HEAD"])
            .contains(&format!("sources/{P}/{FIRST}.md"))
    );
}
#[test]
fn unrelated_staged_index_preserved_before_any_drop_or_discard() {
    let f = Fixture::new();
    f.run("ok", true);
    write(&f.root.join("unrelated"), "keep");
    f.git(&["add", "unrelated"]);
    let before = f.git(&["diff", "--cached"]);
    f.cli(&["accept", "p01", "b001", "--drop", FIRST], "ok", false);
    assert_eq!(f.git(&["diff", "--cached"]), before);
    assert!(f.artifact(FIRST).exists());
    f.cli(&["discard", "p01", "b001", "--reason", "品質"], "ok", false);
    assert!(f.artifact(FIRST).exists());
}
#[test]
fn discard_records_reasons_removes_only_unaccepted_files_and_scopes_commit() {
    let f = Fixture::new();
    f.run("ok", true);
    f.cli(&["discard", "p01", "b001", "--reason", "品質"], "ok", true);
    assert!(!f.scratch().exists());
    assert_eq!(count_files(&f.root.join(format!("people/{P}"))), 0);
    assert_eq!(count_files(&f.root.join(format!("sources/{P}"))), 0);
    assert!(read(&f.plan()).contains("破棄：品質。"));
    assert!(read(&f.root.join("progress/findings.tsv")).contains("coordinator"));
    assert!(read(&f.root.join("progress/batches.tsv")).contains("discarded"));
    for p in f
        .git(&["show", "--format=", "--name-only", "HEAD"])
        .lines()
        .filter(|s| !s.is_empty())
    {
        assert!(
            p.starts_with("progress/p01-test/b001.")
                || ["progress/findings.tsv", "progress/batches.tsv"].contains(&p)
        );
    }
}
#[test]
fn accepted_files_protected_from_discard_and_failed_accept_preserves_done_ledger() {
    let f = Fixture::new();
    f.run("ok", true);
    f.accepted();
    f.cli(
        &["discard", "p01", "b001", "--reason", "誤操作"],
        "ok",
        false,
    );
    assert!(f.artifact(FIRST).exists());
    assert!(f.artifact(SECOND).exists());
    let f = Fixture::new();
    f.run("ok", true);
    executable(&f.root.join("bin/check"), "#!/bin/sh\nexit 1\n");
    f.cli(&["accept", "p01", "b001"], "ok", false);
    assert!(!read(&f.root.join("progress/batches.tsv")).contains("accepted"));
}
#[test]
fn template_retry_fix_and_terminal_rejection() {
    let f = Fixture::new();
    f.run("template-once", true);
    assert_eq!(f.calls()["templates"], 2);
    assert_eq!(f.calls()["templates-review"], 2);
    assert!(read(&f.root.join("progress/findings.tsv")).contains("templates-review"));
    f.cli(&["templates", "p01", "--fix", "フッタ欠落"], "ok", true);
    assert!(f.root.join(format!("templates/{P}/README.md")).exists());
    let f = Fixture::new();
    f.run("template-always", false);
    assert_eq!(f.calls()["templates"], 3);
}
#[test]
fn findings_idempotence_filters_and_four_column_criteria() {
    let f = Fixture::new();
    write(&f.root.join("four-column"), "");
    f.run("draft-reject-once", true);
    let path = f.root.join("progress/findings.tsv");
    let before = fs::read(&path).unwrap();
    f.run("ok", true);
    assert_eq!(fs::read(&path).unwrap(), before);
    assert!(read(&path).contains("D3"));
    let report = read(&f.scratch().join("draft-review-r0.tsv"));
    assert!(report.lines().next().unwrap().split('\t').count() == 4);
    assert!(report.contains("\tD3\t"));
    write(
        &f.root.join("progress/improvements.md"),
        "## 2020-01-01\n- D3: 合計検査\n",
    );
    let summary = stdout(&f.cli(
        &["findings", "--since", "2020-01-01", "--persona", "p01"],
        "ok",
        true,
    ));
    assert!(summary.contains("draft-review"));
    assert!(summary.contains("b001"));
    assert!(summary.contains("2020-01-01\tD3\t0\t1"));
    assert!(
        !stdout(&f.cli(&["findings", "--since", "2999-01-01"], "ok", true))
            .contains("draft-review")
    );
    f.cli(&["accept", "p01", "b001", "--drop", FIRST], "ok", true);
    assert!(read(&path).contains("coordinator"));
}
#[test]
fn stamp_failure_isolated_per_artifact_and_device_metadata_forwarded() {
    let f = Fixture::new();
    f.run("stamp-fail", true);
    assert_eq!(f.state()["pending"], 1);
    assert_eq!(count_files(&f.root.join(format!("sources/{P}"))), 2);
    assert!(f.artifact(SECOND).exists());
    assert!(read(&f.root.join("progress/findings.tsv")).contains("metadata"));
    let f = Fixture::new();
    f.run("device", true);
    for entry in read(&f.root.join("tool-calls.jsonl"))
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|v| v[0] == "stamp")
    {
        let args = entry[1].as_array().unwrap();
        let i = args.iter().position(|v| v == "--device").unwrap();
        assert_eq!(args[i + 1], "Office Scanner 42");
    }
}
#[test]
fn formula_recalc_precedes_stamp_value_only_bypasses_and_failure_prevents_stamp() {
    for (formula, fail) in [(true, false), (false, false), (true, true)] {
        let f = Fixture::new();
        f.planned();
        let mut row = f.rows()[0].clone();
        row["パス"] = json!("Documents/計算.xlsx");
        row["作り方"] = json!("office:xlsx");
        f.set_plan(&[row], true);
        if !formula {
            write(&f.root.join("value-only"), "");
        }
        f.overrides(json!({"Documents/計算.xlsx":{"body":"計算結果を確認した。銀行の入金予定を翌月の資金繰りへ反映する。","metadata":if formula{json!({"modified_by":"千田","created":"2026-02-01"})}else{json!({})}}}));
        f.run(if fail { "recalc-fail" } else { "ok" }, true);
        let tools: Vec<Value> = read(&f.root.join("tool-calls.jsonl"))
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .filter(|v: &Value| v[0] == "stamp" || v[0] == "recalc")
            .collect();
        if fail {
            assert!(tools.iter().all(|v| v[0] == "recalc"));
            assert!(
                string(&f.state()["failures"]["Documents/計算.xlsx"]).contains("conversion failed")
            );
        } else if formula {
            assert_eq!(
                tools.iter().map(|v| string(&v[0])).collect::<Vec<_>>(),
                vec!["recalc", "stamp"]
            );
            let args = tools[1][1].as_array().unwrap();
            let i = args.iter().position(|v| v == "--modified-by").unwrap();
            assert_eq!(args[i + 1], "千田");
            let i = args.iter().position(|v| v == "--created").unwrap();
            assert_eq!(args[i + 1], "2026-02-01");
        } else {
            assert_eq!(tools.len(), 1);
            assert_eq!(tools[0][0], "stamp");
            assert!(
                !tools[0][1]
                    .as_array()
                    .unwrap()
                    .contains(&json!("--created"))
            );
        }
    }
}
#[test]
fn prompt_uses_references_not_large_source_bodies_and_other_active_plans() {
    let f = Fixture::new();
    write(
        &f.root.join(format!("progress/{P}/facts.md")),
        "F".repeat(200_000),
    );
    write(
        &f.root.join(format!("templates/{P}/README.md")),
        "PRIVATE_DOCUMENT_BODY",
    );
    write(
        &f.root.join(format!("progress/{P}/b002.request.md")),
        "OTHER_REQUEST_BODY",
    );
    write(
        &f.root.join(format!("progress/{P}/b002.md")),
        format!("{HEADER}| Documents/別.txt | 2026-03-06 | text | OTHER_PLAN_BODY | | |\n"),
    );
    write(&f.root.join(format!("scratch/{P}/b002/state.json")), "{}");
    f.planned();
    let prompts = f.lines("prompts.jsonl");
    let prompt = prompts
        .iter()
        .map(string)
        .find(|p| p.contains("\"stage\":\"plan\""))
        .unwrap();
    for text in [
        "PRIVATE_DOCUMENT_BODY",
        "OTHER_REQUEST_BODY",
        "OTHER_PLAN_BODY",
        &"F".repeat(100),
    ] {
        assert!(!prompt.contains(text));
    }
    for path in [
        format!("personas/{P}.md"),
        format!("templates/{P}/README.md"),
        format!("progress/{P}/facts.md"),
        format!("progress/{P}/b002.request.md"),
        format!("progress/{P}/b002.md"),
    ] {
        assert!(prompt.contains(&path));
    }
    assert!(prompt.contains("範囲：月次報告"));
    assert!(prompt.len() < 60 * 1024);
    for a in f.lines("argv.jsonl") {
        assert!(
            a.as_array()
                .unwrap()
                .iter()
                .all(|v| !string(v).contains("BATCH_CONTEXT"))
        );
    }
}
#[test]
fn oversized_request_and_template_rejected_before_codex() {
    for request in [true, false] {
        let f = Fixture::new();
        if request {
            write(
                &f.root.join(format!("progress/{P}/b001.request.md")),
                "依".repeat(21000),
            );
        } else {
            write(&f.root.join("prompts/templates.md"), "T".repeat(61441));
        }
        let out = f.cli(&["run", "p01", "b001"], "ok", false);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains("60 KiB"));
        assert!(stderr.contains(if request {
            "依頼本文="
        } else {
            "ひな形=61441 bytes"
        }));
        assert!(stderr.contains("BATCH_CONTEXT="));
        assert!(!f.scratch().join("calls.json").exists());
    }
}
#[test]
fn review_prompts_do_not_contain_producer_output() {
    let f = Fixture::new();
    write(&f.root.join("producer-note.md"), "PRODUCER_SECRET");
    f.run("ok", true);
    let stages = f.lines("contexts.jsonl");
    let prompts = f.lines("prompts.jsonl");
    for (stage, prompt) in stages.iter().zip(&prompts) {
        if string(&stage["stage"]).ends_with("-review") {
            assert!(!string(prompt).contains("PRODUCER_SECRET"));
            assert!(string(prompt).contains(&format!("progress/{P}/b001.md")));
        }
    }
}
#[test]
fn real_checker_and_manifest_contract_and_source_repair() {
    let f = Fixture::new();
    f.real("check");
    f.real("manifest");
    f.run("ok", true);
    assert_eq!(f.state()["count"], 2);
    f.accepted();
    let manifest = read(&f.root.join(format!("manifests/{P}.tsv")));
    assert_eq!(manifest.lines().count(), 2);
    assert_eq!(manifest.lines().next().unwrap().split('\t').count(), 4);
    let f = Fixture::new();
    f.real("check");
    f.run("number-once", true);
    assert_eq!(f.calls()["draft"], 1);
    assert_eq!(f.calls()["build"], 2);
    assert!(read(&f.artifact(FIRST)).contains("1240"));
}
#[test]
fn zero_build_pipeline_with_real_check_missing_persona_and_symlink_guard() {
    let f = Fixture::new();
    f.planned();
    let rows = f.rows()[..1].to_vec();
    f.set_plan(&rows, true);
    f.real("check");
    fs::remove_dir_all(f.root.join(format!("people/{P}"))).unwrap();
    f.run("draft-reject-always", true);
    assert_eq!(f.state()["count"], 0);
    assert!(!f.root.join(format!("people/{P}")).exists());
    assert!(read(&f.scratch().join("logs/final-check.log")).contains("passed: 0, failed: 0"));
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args(["check", "p99", "--batch", "b001"])
        .env("CORPUS_BASE", &f.root)
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(
        format!("{}{}", stdout(&out), String::from_utf8_lossy(&out.stderr))
            .contains("persona not found")
    );
    std::os::unix::fs::symlink(f.root.join("progress"), f.root.join(format!("people/{P}")))
        .unwrap();
    let mut s = f.state();
    s["status"] = json!("failed");
    s["completed"]
        .as_object_mut()
        .unwrap()
        .remove("final-check");
    f.save(&s);
    let out = f.cli(&["run", "p01", "b001"], "ok", false);
    assert!(String::from_utf8_lossy(&out.stderr).contains("symlink"));
}
#[test]
fn grouped_extension_targets_and_status_report() {
    let f = Fixture::new();
    write(
        &f.root.join(format!("personas/{P}.md")),
        "---\nperiod: 2026-01-01..2026-12-31\n---\n## 形式の目安\n| pdf（テキスト） | 16% |\n| pdf（スキャン） | 9% |\n| png・jpg | 4% |\n| txt・md | 3% |\n",
    );
    write(
        &f.root.join(format!("manifests/{P}.tsv")),
        "file.pdf\thash\n",
    );
    let out = stdout(&f.cli(&["status", "p01"], "ok", true));
    assert!(out.contains("差 +75.0 pt"));
    assert!(out.contains("jpg・png"));
}
#[test]
fn command_argument_contracts_and_missing_request() {
    let f = Fixture::new();
    for args in [
        vec!["findings", "--since", "2020-01-01", "--persona", "p01"],
        vec!["status"],
    ] {
        f.cli(&args, "ok", true);
    }
    for args in [
        vec!["run", "p01"],
        vec!["status", "p01", "b001", "extra"],
        vec!["accept", "p01", "b001", "--rebuild"],
        vec!["discard", "p01", "b001"],
        vec!["findings", "p01"],
        vec!["start", "p01", "b001", "--wait-lock"],
    ] {
        let out = f.command(&args, "ok").output().unwrap();
        assert_eq!(out.status.code(), Some(2), "{args:?}");
    }
    fs::remove_file(f.root.join(format!("progress/{P}/b001.request.md"))).unwrap();
    let out = f.cli(&["start", "p01", "b001"], "ok", false);
    assert!(String::from_utf8_lossy(&out.stderr).contains("依頼なし"));
}
fn rows_named(f: &Fixture, names: &[String]) -> Vec<Value> {
    let first = f.rows()[0].clone();
    names
        .iter()
        .map(|p| {
            let mut row = first.clone();
            row["パス"] = json!(p);
            row
        })
        .collect()
}
#[test]
fn weighted_build_and_ten_file_draft_review_groups() {
    let f = Fixture::new();
    f.planned();
    let rows = rows_named(
        &f,
        &(0..23)
            .map(|i| format!("Documents/連絡-{i}.txt"))
            .collect::<Vec<_>>(),
    );
    f.set_plan(&rows, true);
    f.run("ok", true);
    let calls = f.calls();
    assert_eq!(calls["build"], 4);
    assert_eq!(calls["build-review"], 3);
    assert_eq!(calls["draft"], 3);
    assert_eq!(calls["draft-review"], 3);
}
#[test]
fn expensive_builds_group_separately_from_lightweight_documents() {
    let f = Fixture::new();
    f.planned();
    let rows = rows_named(
        &f,
        &(0..8)
            .map(|i| format!("Documents/{i}.{}", if i < 2 { "pptx" } else { "eml" }))
            .collect::<Vec<_>>(),
    );
    f.set_plan(&rows, true);
    f.run("ok", true);
    let builds = f.contexts("build");
    let mut sizes: Vec<_> = builds
        .iter()
        .map(|c| c["rows"].as_array().unwrap().len())
        .collect();
    sizes.sort();
    assert_eq!(sizes, vec![2, 6]);
    assert_eq!(f.state()["build_groups"].as_object().unwrap().len(), 2);
    assert_eq!(
        f.contexts("build-review")[0]["rows"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
}
#[test]
fn custom_weights_allow_oversize_singleton_and_fractional_lightweight_group() {
    let f = Fixture::new();
    f.planned();
    let mut names = vec!["Documents/a.pptx".to_string()];
    names.extend((0..6).map(|i| format!("Documents/連絡-{i}.eml")));
    let rows = rows_named(&f, &names);
    f.set_plan(&rows, true);
    write(
        &f.root.join("config/limits.json"),
        json!({"max_build_weight":2,"build_weights":{"pptx":9,"other":0.5}}).to_string(),
    );
    f.run("ok", true);
    let builds = f.contexts("build");
    let mut sizes: Vec<_> = builds
        .iter()
        .map(|c| c["rows"].as_array().unwrap().len())
        .collect();
    sizes.sort();
    assert_eq!(sizes, vec![1, 2, 4]);
    assert!(
        builds
            .iter()
            .any(|c| c["rows"].as_array().unwrap().len() == 1
                && c["rows"][0]["パス"] == "Documents/a.pptx")
    );
}
#[test]
fn default_weights_preserve_method_and_extension_distinctions() {
    let f = Fixture::new();
    f.planned();
    let mut rows = vec![];
    for (i, (ext, method)) in [
        ("xlsx", "office:t.xlsx"),
        ("png", "handwriting"),
        ("pdf", "scan:t.pdf"),
        ("pdf", "tex:t.tex"),
        ("docx", "office:t.docx"),
        ("pdf", "office:a.docx"),
    ]
    .iter()
    .enumerate()
    {
        let mut row = f.rows()[0].clone();
        row["パス"] = json!(format!("Documents/連絡-{i}.{ext}"));
        row["作り方"] = json!(method);
        rows.push(row);
    }
    f.set_plan(&rows, true);
    write(
        &f.root.join("config/limits.json"),
        json!({"max_build_weight":2,"build_weights":{"other":0.5}}).to_string(),
    );
    f.run("ok", true);
    let groups = f.contexts("build");
    assert_eq!(groups.len(), 5);
    assert_eq!(
        groups
            .iter()
            .map(|g| g["rows"].as_array().unwrap().len())
            .collect::<Vec<_>>()
            .iter()
            .sum::<usize>(),
        6
    );
    assert!(
        groups
            .iter()
            .any(|g| g["rows"].as_array().unwrap().len() == 2
                && g["rows"][0]["パス"] == rows[4]["パス"]
                && g["rows"][1]["パス"] == rows[5]["パス"])
    );
}
#[test]
fn partial_success_continues_same_round_with_explanation_and_no_retry() {
    let f = Fixture::new();
    f.run("skip-continuation-once", true);
    let builds = f.contexts("build");
    assert_eq!(
        builds
            .iter()
            .map(|c| c["rows"].as_array().unwrap().len())
            .collect::<Vec<_>>(),
        vec![2, 1]
    );
    assert!(builds.iter().all(|c| c["round"] == 0));
    assert_eq!(builds[1]["continuation"], 1);
    assert!(string(&builds[1]["continuation_output"][FIRST]).contains("写真生成ができなかった"));
    assert_eq!(builds[1]["reasons"], json!({}));
    assert_eq!(f.state()["build_retries"], 0);
    assert_eq!(f.state()["build_history"][0]["failures"], json!({}));
    assert!(
        read(&f.root.join(format!("progress/{P}/b001.result.md")))
            .split("## 続きを使った行")
            .nth(1)
            .unwrap()
            .contains(FIRST)
    );
    assert!(
        stdout(&f.cli(&["status", "p01", "b001"], "ok", true))
            .contains("成形の組 1 / 続きを使った行 1 件")
    );
}
#[test]
fn exhausted_continuations_record_missing_once_and_include_final_output() {
    let f = Fixture::new();
    f.run("skip-build-always", true);
    let builds = f.contexts("build");
    assert_eq!(builds.len(), 9);
    for (i, c) in builds.iter().enumerate() {
        assert_eq!(c["round"], i / 3);
        assert_eq!(c["continuation"], i % 3);
    }
    let state = f.state();
    assert!(string(&state["failures"][FIRST]).contains("planned file missing"));
    let events: Vec<_> = state["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e[5] == "plan")
        .collect();
    assert_eq!(events.len(), 1);
    assert!(string(&events[0][7]).contains("写真生成ができなかった"));
    for p in [
        f.plan(),
        f.root.join(format!("progress/{P}/b001.result.md")),
    ] {
        assert!(read(&p).contains("写真生成ができなかった"));
    }
}
#[test]
fn eventual_build_success_does_not_record_transient_missing_failure_and_rebuild_restores_content() {
    let f = Fixture::new();
    f.run("skip-build-once", true);
    assert_eq!(f.state()["pending"], 0);
    assert!(!read(&f.root.join("progress/findings.tsv")).contains("planned file missing"));
    let f = Fixture::new();
    f.run("skip-build-always", true);
    f.accepted();
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    assert_eq!(f.state()["pending"], 0);
    assert!(!read(&f.plan()).contains("成形待ち："));
    assert_eq!(
        read(&f.plan())
            .matches("案件の判断と相手への連絡。")
            .count(),
        2
    );
}
#[test]
fn continuation_budget_survives_resume_and_global_slot_limit() {
    let mut f = Fixture::new();
    write(&f.root.join("events-enabled"), "");
    write(
        &f.root.join("config/limits.json"),
        "{\"max_build_weight\":1}",
    );
    f.env.insert("CORPUS_MAX_CODEX".into(), "1".into());
    f.run("skip-continuation-once", true);
    let mut active = 0;
    let mut maximum = 0;
    for e in f.lines("events.jsonl") {
        if e[0] == "start" {
            active += 1;
        } else {
            active -= 1;
        }
        maximum = maximum.max(active);
    }
    assert_eq!((maximum, active), (1, 0));
    assert_eq!(f.state()["build_retries"], 0);
    let mut s = f.state();
    s["status"] = json!("failed");
    s["completed"]
        .as_object_mut()
        .unwrap()
        .remove("build-complete");
    s["completed"]
        .as_object_mut()
        .unwrap()
        .remove("build-r0-g0");
    s["build_runs"]["build-r0-g0"]["completed"] = json!(3);
    fs::remove_file(f.artifact(FIRST)).unwrap();
    f.save(&s);
    let before = f.contexts("build").len();
    f.run("skip-build-always", true);
    assert!(
        f.contexts("build")[before..]
            .iter()
            .all(|c| c["round"] != 0)
    );
}
#[test]
fn root_strays_quarantined_while_tracked_ignored_and_existing_paths_preserved() {
    let f = Fixture::new();
    write(
        &f.root.join(".gitignore"),
        "scratch/\n.venv/\nignored.txt\n",
    );
    f.git(&["add", ".gitignore"]);
    f.git(&["commit", "-qm", "Ignore temporary files"]);
    write(&f.root.join("before.txt"), "before");
    let tracked = fs::read(f.root.join("bin/batch")).unwrap();
    f.run("stray", true);
    assert!(!f.root.join("unexpected.txt").exists());
    assert!(!f.root.join("unexpected-dir").exists());
    assert!(f.root.join("before.txt").exists());
    assert!(f.root.join("ignored.txt").exists());
    assert!(f.root.join("config/new.json").exists());
    assert_eq!(fs::read(f.root.join("bin/batch")).unwrap(), tracked);
    assert!(
        walkdir::WalkDir::new(f.scratch().join("work"))
            .into_iter()
            .flatten()
            .any(|e| e.path().ends_with("stray/unexpected.txt"))
    );
    assert!(
        read(&f.root.join(format!("progress/{P}/b001.result.md")))
            .contains("repo の直下に作られたファイル：unexpected.txt（build、")
    );
}
#[test]
fn auxiliary_rejects_scratch_symlink_escaping_repo() {
    let f = Fixture::new();
    let external = tempfile::tempdir().unwrap();
    write(&external.path().join("brief.md"), "instruction");
    std::os::unix::fs::symlink(external.path(), f.root.join("scratch/escape")).unwrap();
    let brief = f.root.join("scratch/escape/brief.md");
    let output = f.root.join("scratch/escape/out.md");
    let out = f.cli(
        &[
            "codex-once",
            "--brief",
            brief.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ],
        "ok",
        false,
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("scratch"));
    assert!(!external.path().join("out.md").exists());
}
#[test]
fn concurrent_stray_records_all_candidate_executions_and_failed_run_preserves_ignored_children() {
    let mut f = Fixture::new();
    write(
        &f.root.join("config/limits.json"),
        "{\"max_build_weight\":1}",
    );
    f.env.insert("CORPUS_MAX_CODEX".into(), "2".into());
    f.run("parallel-stray", true);
    let state = f.state();
    let records: Vec<_> = state["strays"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["name"] == "shared-stray.txt")
        .collect();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["candidates"].as_array().unwrap().len(), 2);
    assert!(f.root.join(string(&records[0]["destination"])).is_file());
    let f = Fixture::new();
    write(&f.root.join(".gitignore"), "scratch/\nmixed/keep.txt\n");
    f.git(&["add", ".gitignore"]);
    f.git(&["commit", "-qm", "Ignore protected child"]);
    write(&f.root.join("mixed-stray-enabled"), "");
    f.run("fail", false);
    assert!(f.root.join("mixed/keep.txt").exists());
    assert!(!f.root.join("mixed/move.txt").exists());
    assert_eq!(f.state()["strays"].as_array().unwrap().len(), 2);
}
#[test]
fn missing_output_extracts_row_section_and_unlabelled_reason() {
    for note in [
        format!(
            "## 作れなかった行\n- {FIRST}\n  画像生成が利用できない。\n- {SECOND}: 完了\n別の所見"
        ),
        "全体が利用上限".into(),
    ] {
        let f = Fixture::new();
        write(&f.root.join("producer-note.md"), &note);
        f.run("skip-build-always", true);
        let continuation = f
            .contexts("build")
            .into_iter()
            .find(|c| c["continuation"] == 1)
            .unwrap();
        let reason = string(&continuation["continuation_output"][FIRST]);
        if note.starts_with('#') {
            assert!(reason.contains("画像生成が利用できない"));
            assert!(!reason.contains(SECOND));
            assert!(!reason.contains("別の所見"));
        } else {
            assert_eq!(reason, note);
        }
    }
}
#[test]
fn large_batch_context_rejected_before_draft_and_unicode_casefold_duplicates_detected() {
    let f = Fixture::new();
    f.planned();
    let mut rows = rows_named(
        &f,
        &(0..10)
            .map(|i| format!("Documents/連絡-{i}.txt"))
            .collect::<Vec<_>>(),
    );
    for row in &mut rows {
        row["内容"] = json!("詳".repeat(7000));
    }
    f.set_plan(&rows, true);
    f.run("ok", false);
    assert!(string(&f.state()["reason"]).contains("BATCH_CONTEXT="));
    assert!(f.calls().get("draft").is_none());
    let f = Fixture::new();
    f.overrides(json!({FIRST:{"body":"Straße alpha bravo charlie delta echo foxtrot golf hotel india juliet"},SECOND:{"body":"STRASSE alpha bravo charlie delta echo foxtrot golf hotel india juliet"}}));
    f.run("ok", true);
    assert_eq!(f.state()["draft_failures"].as_object().unwrap().len(), 2);
}

#[test]
fn coordinator_selects_sol_for_a_pending_rebuild() {
    let f = Fixture::new();
    f.run("reject-always", true);
    let request = f.root.join(format!("progress/{P}/b001.request.md"));
    fs::write(
        &request,
        format!("{}\n- 再制作の担当：gpt-6.2-sol\n", read(&request)),
    )
    .unwrap();
    f.cli(&["run", "p01", "b001", "--rebuild"], "ok", true);
    assert_eq!(f.state()["producer"], "gpt-6.2-sol");
}
