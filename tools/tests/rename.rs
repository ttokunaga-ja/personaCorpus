mod common;

use common::{BODY, Fixture, STAMP, err, failure, success};
use filetime::FileTime;
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf, process::Output};

const OLD: &str = "Documents/I wrote down what was decided at the supplier meeting.txt";
const NEW: &str = "Documents/Supplier minutes.txt";
const OTHER: &str = "Desktop/Schedule.txt";
const MAIL: &str = "Documents/Sent.eml";
const OLD_ATTACHMENT: &str = "I wrote down what was decided at the supplier meeting.txt";

struct Rename {
    f: Fixture,
    progress: PathBuf,
    manifest: PathBuf,
}
impl Rename {
    fn new() -> Self {
        let mut f = Fixture::new();
        f.env.insert(
            "AGENT_OPS_HOME".into(),
            f.base.join("ops").display().to_string(),
        );
        f.cfg("period: 2025-01-01..2026-12-31\ndate_prefix: YYYY-MM-DD_ | DD Mon - ");
        let progress = f.base.join("progress").join(&f.name);
        let manifest = f.base.join("manifests").join(format!("{}.tsv", f.name));
        Self {
            f,
            progress,
            manifest,
        }
    }
    fn accept(&self) {
        self.f.accept();
        success(&self.f.cli("manifest", &["p01"]));
        self.f.git(&["config", "user.name", "Fixture"]);
        self.f
            .git(&["config", "user.email", "fixture@example.invalid"]);
        self.f.git(&["add", "."]);
        self.f.git(&["commit", "-qm", "Accepted inventory"]);
    }
    fn map(&self, rows: &str) {
        self.f.write(
            &self.progress.join("rename.tsv"),
            format!("旧\t新\t理由\n{rows}").as_bytes(),
        );
    }
    fn run(&self, action: &str) -> Output {
        self.f.cli("rename", &[action, "p01"])
    }
    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.progress.join(path)).unwrap()
    }
}

#[test]
fn validate_reports_missing_map_and_all_other_findings() {
    let r = Rename::new();
    r.f.put(OLD, BODY);
    r.f.put("Desktop/Oct 24 notes.txt", "Notes");
    r.f.put("Desktop/0220_当日だけならよさそうに見えた.md", "Notes");
    r.f.put(
        "Desktop/2025-02-05_I looked at the button first.txt",
        "Notes",
    );
    r.f.put("Desktop/Do not forward this message.eml", "mail");
    r.f.put("Documents/2025-10-24/24 Oct - agenda.txt", "Agenda");
    r.f.put("Desktop/2025-10-24_Notes.txt", "Notes");
    r.f.put("Desktop/24 Oct - agenda.txt", "Agenda");
    r.f.put(&format!("Desktop/{}.eml", "あ".repeat(60)), "mail");
    r.f.put(&format!("Desktop/{}.eml", "い".repeat(61)), "mail");
    r.accept();
    let o = r.run("validate");
    failure(&o);
    let output = err(&o);
    for reason in [
        "rename.tsv",
        "付け直しが要る",
        "40 文字",
        "60 文字",
        "日付の形",
        "日付のフォルダ",
        "名前が要約・結論・会話の文",
        "0220_当日だけならよさそうに見えた.md",
        "2025-02-05_I looked at the button first.txt",
    ] {
        assert!(output.contains(reason), "{output}");
    }
    for good in [
        "Desktop/2025-10-24_Notes.txt".into(),
        "Desktop/24 Oct - agenda.txt".into(),
        "Desktop/Do not forward this message.eml".into(),
        "あ".repeat(60),
    ] {
        assert!(!output.contains(&good), "{output}");
    }
}

#[test]
fn validate_rejects_bad_rows_and_existing_source_destination() {
    let r = Rename::new();
    r.f.put(OLD, BODY);
    r.accept();
    r.f.put("Not accepted.txt", BODY);
    r.f.source(NEW, "A manuscript already exists here", "");
    r.map(&format!(
        "Not accepted.txt\t{NEW}\t分類\n{OLD}\t{NEW}\t短縮\n{OLD}\tMinutes.pdf\t短縮\n"
    ));
    let o = r.run("validate");
    failure(&o);
    let output = err(&o);
    for reason in [
        "受入済みにない",
        "旧が重複",
        "新が重複",
        "拡張子が変わる",
        "原稿",
    ] {
        assert!(output.contains(reason), "{output}");
    }
    failure(&r.run("apply"));
    assert!(r.f.home.join(OLD).is_file());
    assert!(!r.f.home.join(NEW).exists());
}

fn email(name: &str) -> String {
    format!(
        "From: eleanor@example.invalid\nTo: sam@example.invalid\nSubject: Schedule\n\nThe attached file is {name}."
    )
}

#[test]
fn apply_updates_references_and_verify_can_be_repeated() {
    let r = Rename::new();
    r.f.put(OLD, BODY);
    let narrative = format!(
        "The previous record is {OLD}. Its attachment is {OLD_ATTACHMENT}. Keep Copy-{OLD_ATTACHMENT} as written."
    );
    let other = r.f.put(OTHER, &narrative);
    let source = r.f.source(OTHER, &narrative, OLD);
    let mail_source = r.f.put(MAIL, &email(OLD_ATTACHMENT));
    // The message itself, rather than the manuscript, is the evidence of the old name.
    r.f.source(MAIL, &email("Supplier minutes.txt"), "");
    r.f.plan(&[(OLD, "2026-01-01", "Supplier minutes")]);
    r.accept();
    r.map(&format!("{OLD}\t{NEW}\t短縮\n"));
    r.f.time(&other, STAMP + 30);

    success(&r.run("apply"));
    assert!(!r.f.home.join(OLD).exists());
    assert!(
        !r.f.base
            .join("sources")
            .join(&r.f.name)
            .join(format!("{OLD}.md"))
            .exists()
    );
    assert_eq!(fs::read_to_string(r.f.home.join(NEW)).unwrap(), BODY);
    let moved =
        r.f.base
            .join("sources")
            .join(&r.f.name)
            .join(format!("{NEW}.md"));
    assert!(
        fs::read_to_string(moved)
            .unwrap()
            .contains(&format!("path: {NEW}"))
    );
    assert!(
        fs::read_to_string(source)
            .unwrap()
            .contains(&format!("derived_from: {NEW}"))
    );
    let new_body = fs::read_to_string(&other).unwrap();
    assert!(new_body.contains(NEW) && new_body.contains("is Supplier minutes.txt."));
    assert!(new_body.contains(&format!("Copy-{OLD_ATTACHMENT}")));
    assert!(r.read("b001.md").contains(NEW));
    assert_eq!(
        FileTime::from_last_modification_time(&fs::metadata(&other).unwrap()).unix_seconds(),
        STAMP + 30
    );
    assert!(
        r.read("rename-refs.tsv")
            .contains(&format!("{MAIL}\t{OLD_ATTACHMENT}"))
    );
    let original_manifest = fs::read(&r.manifest).unwrap();
    failure(&r.run("verify"));
    assert_eq!(fs::read(&r.manifest).unwrap(), original_manifest);

    fs::write(&mail_source, email("Supplier minutes.txt")).unwrap();
    r.f.time(&r.f.home.join(NEW), STAMP + 200);
    success(&r.run("verify"));
    success(&r.run("verify"));
    assert_eq!(
        FileTime::from_last_modification_time(&fs::metadata(other).unwrap()).unix_seconds(),
        STAMP
    );
    let manifest = fs::read_to_string(&r.manifest).unwrap();
    assert!(manifest.contains(&format!("{NEW}\t")));
    assert!(!manifest.contains(OLD));

    r.f.write(&r.f.base.join("unrelated"), b"keep staged");
    r.f.git(&["add", "unrelated"]);
    let o = r.run("commit");
    failure(&o);
    assert!(err(&o).contains("stage の範囲外"), "{}", err(&o));
    let git = |args: &[&str]| {
        let o = std::process::Command::new("git")
            .arg("-C")
            .arg(&r.f.base)
            .args(args)
            .output()
            .unwrap();
        success(&o);
        String::from_utf8(o.stdout).unwrap()
    };
    assert_eq!(git(&["diff", "--cached", "--name-only"]), "unrelated\n");
    r.f.git(&["reset", "-q", "--", "unrelated"]);
    let state_path =
        r.f.base
            .join("scratch/rename")
            .join(&r.f.name)
            .join("state.json");
    r.f.write(&state_path, br#"{"stage":"hold"}"#);
    success(&r.run("commit"));
    let state: serde_json::Value = serde_json::from_slice(&fs::read(state_path).unwrap()).unwrap();
    assert_eq!(state["stage"], "done");
    let paths = git(&["show", "--format=", "--name-only", "HEAD"]);
    assert!(
        paths.contains(NEW) && !paths.contains("unrelated"),
        "{paths}"
    );
    success(&r.f.cli("manifest", &["p01", "--verify"]));
}

#[test]
fn run_applies_fixes_and_commits_only_an_accepted_audit() {
    for verdict in ["accept", "hold"] {
        let r = Rename::new();
        r.f.put(OLD, BODY);
        r.f.put(MAIL, &email(OLD_ATTACHMENT));
        r.accept();
        r.map(&format!("{OLD}\t{NEW}\t短縮\n"));
        for prompt in ["rename", "rename-fix", "rename-audit"] {
            r.f.write(
                &r.f.base.join(format!("prompts/{prompt}.md")),
                prompt.as_bytes(),
            );
        }
        let script = r.f.base.join("ops/bin/codex-job");
        let body = format!(
            r#"#!/bin/sh
set -eu
cd {base}
case "$1" in
    start)
        [ "$2" = --repo ] && [ "$4" = --name ] && [ "$6" = --model ]
        case "${{11}}" in *"codex-once --model $7 --brief "*) ;; *) exit 3 ;; esac
        printf '%s\t%s\t%s\n' "$3" "$5" done >> scratch/jobs.tsv
        printf '%s\t%s\n' "$5" "$7" >> scratch/models.tsv
        case "$5" in
            *-fix) sed -i 's/{old}/Supplier minutes.txt/g' people/{persona}/{mail} ;;
            *-audit) printf 'verdict: {verdict}\n' > progress/{persona}/rename.audit.md ;;
        esac
        ;;
    status) cat scratch/jobs.tsv ;;
esac
"#,
            base = common::quote(r.f.base.to_str().unwrap()),
            old = OLD_ATTACHMENT,
            persona = r.f.name,
            mail = MAIL,
        );
        r.f.write(&script, body.as_bytes());
        fs::set_permissions(script, fs::Permissions::from_mode(0o755)).unwrap();
        let output = r.f.cli("rename", &["run", "p01", "--interval", "0"]);
        if verdict == "accept" {
            success(&output);
            success(&r.f.cli("manifest", &["p01", "--verify"]));
        } else {
            failure(&output);
            assert!(err(&output).contains("監査 hold"), "{}", err(&output));
            failure(&r.f.cli("manifest", &["p01", "--verify"]));
            failure(&r.run("commit"));
        }
        assert_eq!(
            fs::read_to_string(r.f.base.join("scratch/models.tsv")).unwrap(),
            "p01-rename\tgpt-6.2-sol\np01-rename-fix\tgpt-6.2-sol\np01-rename-audit\tgpt-6.2-astra\n"
        );
    }
}

#[test]
fn apply_does_not_move_files_when_one_extraction_fails() {
    let r = Rename::new();
    r.f.put(OLD, BODY);
    r.f.put("Documents/Broken.eml", "This is not an email.");
    r.accept();
    r.map(&format!("{OLD}\t{NEW}\t短縮\n"));
    let o = r.run("apply");
    failure(&o);
    assert!(err(&o).contains("Broken.eml"), "{}", err(&o));
    assert!(r.f.home.join(OLD).is_file());
    assert!(!r.f.home.join(NEW).exists());
    assert!(!r.progress.join("rename-refs.tsv").exists());
}
