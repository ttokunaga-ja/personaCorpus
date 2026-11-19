#![allow(dead_code)]
//! Public batch command regressions. Every repository and fake executor is disposable.
use fs2::FileExt;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};
pub const P: &str = "p01-test";
pub const FIRST: &str = "Documents/記録.txt";
pub const SECOND: &str = "Documents/連絡.txt";
pub const HEADER: &str =
    "| パス | 日付 | 作り方 | 内容 | 関連 | 先に要るもの |\n|---|---|---|---|---|---|\n";
pub fn string(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}
pub fn write(p: &Path, s: impl AsRef<[u8]>) {
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, s).unwrap();
}
pub fn read(p: &Path) -> String {
    fs::read_to_string(p).unwrap()
}
pub fn executable(p: &Path, s: &str) {
    write(p, s);
    fs::set_permissions(p, fs::Permissions::from_mode(0o755)).unwrap();
}
pub fn log(p: &Path, v: &Value) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(p)
        .unwrap();
    FileExt::lock_exclusive(&f).unwrap();
    writeln!(f, "{v}").unwrap();
}
pub fn json_file(p: &Path) -> Value {
    serde_json::from_str(&read(p)).unwrap()
}
pub fn args() -> Vec<String> {
    fs::read(std::env::var_os("CORPUS_FAKE_ARGS").unwrap())
        .unwrap()
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| String::from_utf8(b.to_vec()).unwrap())
        .collect()
}
pub fn mode() -> String {
    std::env::var("FAKE_MODE").unwrap_or("ok".into())
}
pub fn manuscript(root: &Path, row: &Value, body: &str, metadata: &Value) {
    let relative = string(&row["パス"]);
    let mut header =
        json!({"path":relative,"kind":row["作り方"],"author":"山田 花子","derived_from":""});
    if let Some(m) = metadata.as_object() {
        for (k, v) in m {
            header[k] = v.clone();
        }
    }
    let h = header
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| format!("{k}: {}\n", string(v)))
        .collect::<String>();
    write(
        &root.join(format!("sources/{P}/{relative}.md")),
        format!("---\n{h}---\n{body}\n"),
    );
}
#[test]
#[ignore]
pub fn fake_service() {
    let Some(tool) = std::env::var_os("CORPUS_FAKE_TOOL") else {
        return;
    };
    let tool = tool.to_string_lossy();
    let root = std::env::current_dir().unwrap();
    let args = args();
    let mode = mode();
    let scratch = root.join(format!("scratch/{P}/b001"));
    if tool != "codex" {
        log(
            &root.join("tool-calls.jsonl"),
            &json!([tool.to_string(), args]),
        );
    }
    match tool.as_ref() {
        "check" => {
            if let Some(i) = args.iter().position(|s| s == "--report") {
                let p = Path::new(&args[i + 1]);
                let mut report = String::new();
                if mode.starts_with("skip-build")
                    && !root.join(format!("people/{P}/{FIRST}")).is_file()
                {
                    report = format!("{P}\t{FIRST}\tplan\tplanned file missing\n");
                }
                if mode == "auto-reject" && p.file_name().unwrap() == "check-r0.tsv" {
                    report = format!("{P}\t{FIRST}\topens\t壊れた文書\n");
                }
                write(p, &report);
                if !report.is_empty() {
                    std::process::exit(1);
                }
            } else if mode.ends_with("final-fail") {
                println!("final failure");
                std::process::exit(1);
            }
            println!("passed");
        }
        "stamp" => {
            if mode == "stamp-fail" && args.first().unwrap().contains("記録") {
                println!("stamp conversion failed");
                std::process::exit(1);
            }
        }
        "recalc" => {
            if mode == "recalc-fail" {
                println!("conversion failed");
                std::process::exit(1);
            }
        }
        "manifest" => {
            let folder = root.join(format!("people/{}", args[0]));
            let mut entries = vec![];
            for p in walkdir::WalkDir::new(&folder)
                .into_iter()
                .flatten()
                .filter(|e| e.file_type().is_file())
            {
                entries.push(format!(
                    "{}\thash\t1\t2026-03-06T12:00:00Z\n",
                    p.path().strip_prefix(&folder).unwrap().display()
                ));
            }
            entries.sort();
            write(
                &root.join(format!("manifests/{}.tsv", args[0])),
                entries.join(""),
            );
        }
        "job" => {
            if args[0] == "status" {
                println!("REPO\tJOB\tSTATE\tDETAIL");
                if root.join("jobs.tsv").exists() {
                    print!("{}", read(&root.join("jobs.tsv")));
                } else {
                    println!(
                        "personaCorpus\tp01-b001\tdone\tfinished\npersonaCorpus\tp01-b001-r2\tfailed\tfailed"
                    );
                }
            } else {
                write(&root.join("job-args.json"), json!(args).to_string());
                if mode == "launch-child" {
                    let i = args.iter().position(|a| a == "--command").unwrap();
                    let words = shell_words::split(&args[i + 1]).unwrap();
                    let mut child = Command::new(&words[0]);
                    child
                        .args(&words[1..])
                        .stdout(Stdio::from(File::create(root.join("child.log")).unwrap()))
                        .stderr(Stdio::null());
                    child.spawn().unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
        "codex" => {
            let mut prompt = String::new();
            std::io::stdin().read_to_string(&mut prompt).unwrap();
            let output = PathBuf::from(&args[args.iter().position(|a| a == "-o").unwrap() + 1]);
            if !prompt.contains("BATCH_CONTEXT\n") {
                write(
                    &output,
                    json!({"args":args,"prompt":prompt,"cwd":root}).to_string(),
                );
                if mode == "hang" {
                    write(&root.join("scratch/ready"), "");
                    thread::sleep(Duration::from_secs(30));
                }
                return;
            }
            assert_eq!(args.last().unwrap(), "-");
            let meta: Value = serde_json::from_str(
                prompt
                    .split_once("BATCH_CONTEXT\n")
                    .unwrap()
                    .1
                    .split_once("\nEND_BATCH_CONTEXT")
                    .unwrap()
                    .0,
            )
            .unwrap();
            let stage = string(&meta["stage"]);
            let round = meta["round"].as_u64().unwrap();
            fs::create_dir_all(&scratch).unwrap();
            log(&scratch.join("argv.jsonl"), &json!(args));
            log(&scratch.join("prompts.jsonl"), &json!(prompt));
            log(&scratch.join("contexts.jsonl"), &meta);
            let lock = OpenOptions::new()
                .create(true)
                .append(true)
                .open(scratch.join("calls.lock"))
                .unwrap();
            FileExt::lock_exclusive(&lock).unwrap();
            let counter = scratch.join("calls.json");
            let mut calls = if counter.exists() {
                json_file(&counter)
            } else {
                json!({})
            };
            let count = calls[stage].as_u64().unwrap_or(0) + 1;
            calls[stage] = json!(count);
            write(&counter, calls.to_string());
            drop(lock);
            if root.join("events-enabled").exists() {
                log(
                    &scratch.join("events.jsonl"),
                    &json!([
                        "start",
                        std::process::id(),
                        stage,
                        meta["rows"],
                        chrono::Utc::now().timestamp_micros()
                    ]),
                );
                if stage == "draft-review" && root.join("review-rendezvous").exists() && count <= 2
                {
                    // Keep each of the first two reviews active until its peer has started.
                    write(&scratch.join(format!("review-started-{count}")), "");
                    let peer = scratch.join(format!("review-started-{}", 3 - count));
                    let deadline = Instant::now() + Duration::from_secs(10);
                    while !peer.exists() {
                        assert!(
                            Instant::now() < deadline,
                            "second concurrent review never started"
                        );
                        thread::sleep(Duration::from_millis(10));
                    }
                } else {
                    thread::sleep(Duration::from_millis(90));
                }
            }
            if mode == "parallel-fail"
                && stage == "draft"
                && string(&meta["rows"][0]["パス"]).ends_with("連絡-0.txt")
            {
                log(
                    &scratch.join("events.jsonl"),
                    &json!([
                        "end",
                        std::process::id(),
                        stage,
                        meta["rows"],
                        chrono::Utc::now().timestamp_micros()
                    ]),
                );
                std::process::exit(3);
            }
            if mode == "fail" && stage == "build" {
                if root.join("mixed-stray-enabled").exists() {
                    write(&root.join("mixed/keep.txt"), "ignored");
                    write(&root.join("mixed/move.txt"), "stray");
                }
                println!("rate limit: retry tomorrow");
                std::process::exit(3);
            }
            match stage {
                "templates" => write(&root.join(format!("templates/{P}/README.md")), "型の一覧\n"),
                "plan" => {
                    let mut names = vec![FIRST.to_string(), SECOND.to_string()];
                    if ["dependency", "parent-fail"].contains(&mode.as_str()) {
                        names.reverse();
                    }
                    if mode == "badplan" && count == 1 {
                        names[0] = "Documents/不正.py".into();
                    }
                    if mode == "badplan-always" {
                        names[0] = "../外.txt".into();
                    }
                    let mut plan = HEADER.to_string();
                    for name in names {
                        let related = if ["dependency", "parent-fail"].contains(&mode.as_str())
                            && name.contains("連絡")
                        {
                            FIRST
                        } else {
                            ""
                        };
                        plan += &format!(
                            "| {name} | 2026-03-06 | text | 案件の判断と相手への連絡。 | {related} | |\n"
                        );
                    }
                    if root.join("fixture-plan.md").exists() {
                        plan = read(&root.join("fixture-plan.md"));
                    }
                    write(&root.join(string(&meta["plan"])), plan);
                }
                "draft" => {
                    for (i, row) in meta["rows"].as_array().unwrap().iter().enumerate() {
                        let relative = string(&row["パス"]);
                        let mut body = if relative == FIRST {
                            "月次の資金繰りについて山田が承認。前月比の差額1240円は入金遅延で説明がついた。".into()
                        } else if relative == SECOND {
                            "研修担当の木下へ参加日程の変更を通知した。火曜日から水曜日へ移すことで回答を得た。".into()
                        } else {
                            let mut value = relative
                                .bytes()
                                .fold(17u64, |a, b| a.wrapping_mul(37).wrapping_add(b as u64));
                            (0..40)
                                .map(|_| {
                                    (0..9)
                                        .map(|_| {
                                            value = value
                                                .wrapping_mul(6364136223846793005)
                                                .wrapping_add(1);
                                            (b'a' + (value >> 32) as u8 % 26) as char
                                        })
                                        .collect::<String>()
                                })
                                .collect::<Vec<_>>()
                                .join(" ")
                        };
                        if i == 0 && mode == "machine-once" && round == 0 {
                            body = "placeholder".into();
                        }
                        if ["dependency", "parent-fail"].contains(&mode.as_str())
                            && relative.contains("連絡")
                        {
                            body += &format!("\n添付: {FIRST}");
                        }
                        let metadata = if mode == "device" {
                            json!({"device":"Office Scanner 42"})
                        } else {
                            json!({})
                        };
                        manuscript(&root, row, &body, &metadata);
                    }
                    if root.join("draft-overrides.json").exists() {
                        let overrides = json_file(&root.join("draft-overrides.json"));
                        for row in meta["rows"].as_array().unwrap() {
                            let relative = string(&row["パス"]);
                            let Some(value) = overrides.get(relative) else {
                                continue;
                            };
                            let path = root.join(format!("sources/{P}/{relative}.md"));
                            if value.is_null() {
                                let _ = fs::remove_file(path);
                            } else if let Some(raw) = value.get("raw") {
                                write(&path, string(raw));
                            } else {
                                manuscript(&root, row, string(&value["body"]), &value["metadata"]);
                            }
                        }
                    }
                    if mode == "metadata-mutate" {
                        write(&root.join(format!("progress/{P}/facts.md")), "unauthorized");
                    }
                }
                "build" => {
                    if mode == "stray" {
                        write(&root.join("unexpected.txt"), "stray");
                        write(&root.join("unexpected-dir/inside"), "stray");
                        write(&root.join("ignored.txt"), "ignored");
                        write(&root.join("config/new.json"), "{}");
                    }
                    if mode == "parallel-stray" {
                        write(&root.join(string(&meta["work"])).join("ready"), "ready");
                        let start = Instant::now();
                        loop {
                            let ready = walkdir::WalkDir::new(scratch.join("work"))
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_name() == "ready")
                                .count();
                            if ready >= 2 {
                                break;
                            }
                            assert!(start.elapsed() < Duration::from_secs(5));
                            thread::sleep(Duration::from_millis(5));
                        }
                        if string(&meta["rows"][0]["パス"]) == FIRST {
                            write(&root.join("shared-stray.txt"), "shared");
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                    for row in meta["rows"].as_array().unwrap() {
                        let relative = string(&row["パス"]);
                        if relative == FIRST
                            && (mode == "skip-build-always"
                                || mode == "skip-build-once" && round == 0
                                || mode == "skip-continuation-once" && meta["continuation"] == 0)
                        {
                            continue;
                        }
                        let source = root.join(format!("sources/{P}/{relative}.md"));
                        let dest = root.join(format!("people/{P}/{relative}"));
                        let body = read(&source).split_once("\n---\n").unwrap().1.to_string();
                        write(&dest, &body);
                        if relative.ends_with(".xlsx") {
                            let formula = if root.join("value-only").exists() {
                                ""
                            } else {
                                "<f>1+1</f>"
                            };
                            let mut zip = zip::ZipWriter::new(File::create(&dest).unwrap());
                            zip.start_file(
                                "xl/worksheets/sheet1.xml",
                                zip::write::SimpleFileOptions::default(),
                            )
                            .unwrap();
                            write!(zip,"<worksheet><sheetData><row><c>{formula}<v>2</v></c></row></sheetData></worksheet>").unwrap();
                            zip.finish().unwrap();
                        }
                        if mode == "number-once" && round == 0 && relative == FIRST {
                            write(&dest, body.replace("1240", ""));
                        }
                        if mode == "mutate-build" {
                            write(&source, read(&source) + "改変");
                        }
                    }
                }
                "templates-review" => {
                    let reject = mode == "template-always" || mode == "template-once" && count == 1;
                    write(
                        &root.join(string(&meta["review_output"])),
                        format!(
                            "README.md\t{}\tother\t{}\n",
                            if reject { "reject" } else { "pass" },
                            if reject {
                                "フッタ欠落"
                            } else {
                                "説明と実物を確認"
                            }
                        ),
                    );
                }
                "draft-review" | "build-review" => {
                    let mut lines = vec![];
                    for (i, row) in meta["rows"].as_array().unwrap().iter().enumerate() {
                        let relative = string(&row["パス"]);
                        let draft = stage == "draft-review";
                        let reject = i == 0
                            && if draft {
                                ["draft-reject-always", "draft-final-fail"].contains(&mode.as_str())
                                    || mode == "draft-parent-fail" && relative == FIRST
                                    || mode == "draft-reject-once" && round == 0
                            } else {
                                ["reject-always", "build-final-fail"].contains(&mode.as_str())
                                    || mode == "parent-fail" && relative == FIRST
                                    || mode == "reject-once" && round == 0
                            };
                        if mode == "mutate" && !draft {
                            write(&root.join(format!("people/{P}/{relative}")), "改変");
                        }
                        if mode == "mutate-build-review-source" && !draft
                            || mode == "mutate-draft" && draft
                        {
                            write(&root.join(format!("sources/{P}/{relative}.md")), "改変");
                        }
                        if mode == "missing-review" && i == 0 && !draft {
                            continue;
                        }
                        let criterion =
                            if mode == "four-column" || root.join("four-column").exists() {
                                if draft { "D3\t" } else { "V2\t" }
                            } else {
                                ""
                            };
                        lines.push(format!(
                            "{relative}\t{}\t{criterion}{}",
                            if reject { "reject" } else { "pass" },
                            if reject {
                                "数値不整合"
                            } else {
                                "本文と計画を確認"
                            }
                        ));
                    }
                    write(
                        &root.join(string(&meta["review_output"])),
                        lines.join("\n") + "\n",
                    );
                }
                _ => panic!("unexpected stage: {stage}"),
            }
            let note = if stage == "build" && mode.starts_with("skip-") {
                format!("{FIRST}: 写真生成ができなかった")
            } else {
                "全体所見：途中のメモも含め自然。".into()
            };
            let note = if root.join("producer-note.md").exists() {
                read(&root.join("producer-note.md"))
            } else {
                note
            };
            write(&output, note);
            if root.join("events-enabled").exists() {
                log(
                    &scratch.join("events.jsonl"),
                    &json!([
                        "end",
                        std::process::id(),
                        stage,
                        meta["rows"],
                        chrono::Utc::now().timestamp_micros()
                    ]),
                );
            }
        }
        _ => panic!("unknown fake tool: {tool}"),
    }
}
pub struct Fixture {
    _tmp: tempfile::TempDir,
    pub root: PathBuf,
    pub env: BTreeMap<String, String>,
}
impl Fixture {
    pub fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        for d in [
            "bin",
            "ops/bin",
            "prompts",
            "personas",
            "progress/p01-test",
            "people/p01-test",
            "scratch",
        ] {
            fs::create_dir_all(root.join(d)).unwrap();
        }
        let f = Self {
            _tmp: tmp,
            root,
            env: BTreeMap::new(),
        };
        for tool in ["codex", "check", "stamp", "recalc", "manifest", "job"] {
            let path = match tool {
                "codex" => f.root.join("fake-codex"),
                "job" => f.root.join("ops/bin/codex-job"),
                _ => f.root.join(format!("bin/{tool}")),
            };
            let exe = std::env::current_exe().unwrap();
            executable(
                &path,
                &format!(
                    "#!/bin/sh\nargs=$(mktemp \"$PWD/scratch/fake-args.XXXXXX\")\nprintf '%s\\0' \"$@\" > \"$args\"\nCORPUS_FAKE_ARGS=\"$args\" CORPUS_FAKE_TOOL={} exec {} --ignored --exact batch_support::fake_service --nocapture\n",
                    shell_words::quote(tool),
                    shell_words::quote(exe.to_str().unwrap())
                ),
            );
        }
        f.real("batch");
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        for stage in [
            "templates",
            "templates-review",
            "plan",
            "draft",
            "draft-review",
            "build",
            "build-review",
        ] {
            write(
                &f.root.join(format!("prompts/{stage}.md")),
                fs::read(repo.join(format!("prompts/{stage}.md"))).unwrap(),
            );
        }
        write(
            &f.root.join(format!("personas/{P}.md")),
            "---\npersona: p01-test\nperiod: 2026-01-01..2026-12-31\ntimezone: Asia/Tokyo\n---\n## 形式の目安\n- txt：100%\n",
        );
        write(
            &f.root.join(format!("progress/{P}/b001.request.md")),
            "# 依頼\n- 範囲：月次報告\n- 件数の目安：2\n",
        );
        write(&f.root.join("jobs.tsv"), "");
        f.git(&["init", "-q"]);
        f.git(&["config", "user.name", "Batch Test"]);
        f.git(&["config", "user.email", "test@example.invalid"]);
        f.git(&["config", "core.quotePath", "false"]);
        f.git(&["add", "."]);
        f.git(&["commit", "-qm", "Fixture"]);
        f
    }
    pub fn real(&self, name: &str) {
        executable(
            &self.root.join(format!("bin/{name}")),
            &format!(
                "#!/bin/sh\nexec {} {} \"$@\"\n",
                shell_words::quote(env!("CARGO_BIN_EXE_corpus")),
                shell_words::quote(name)
            ),
        );
    }
    pub fn command(&self, args: &[&str], mode: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_corpus"));
        c.arg("batch")
            .args(args)
            .current_dir(&self.root)
            .env("CORPUS_BASE", &self.root)
            .env("CODEX_BIN", self.root.join("fake-codex"))
            .env("FAKE_MODE", mode)
            .envs(&self.env);
        c
    }
    pub fn cli(&self, args: &[&str], mode: &str, ok: bool) -> Output {
        let out = self.command(args, mode).output().unwrap();
        assert_eq!(
            out.status.success(),
            ok,
            "args={args:?} mode={mode}\n{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        out
    }
    pub fn run(&self, mode: &str, ok: bool) {
        self.cli(&["run", "p01", "b001"], mode, ok);
    }
    pub fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap()
    }
    pub fn scratch(&self) -> PathBuf {
        self.root.join(format!("scratch/{P}/b001"))
    }
    pub fn plan(&self) -> PathBuf {
        self.root.join(format!("progress/{P}/b001.md"))
    }
    pub fn source(&self, p: &str) -> PathBuf {
        self.root.join(format!("sources/{P}/{p}.md"))
    }
    pub fn artifact(&self, p: &str) -> PathBuf {
        self.root.join(format!("people/{P}/{p}"))
    }
    pub fn state(&self) -> Value {
        json_file(&self.scratch().join("state.json"))
    }
    pub fn save(&self, s: &Value) {
        write(&self.scratch().join("state.json"), s.to_string());
    }
    pub fn lines(&self, file: &str) -> Vec<Value> {
        read(&self.scratch().join(file))
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    }
    pub fn contexts(&self, stage: &str) -> Vec<Value> {
        self.lines("contexts.jsonl")
            .into_iter()
            .filter(|v| v["stage"] == stage)
            .collect()
    }
    pub fn calls(&self) -> Value {
        json_file(&self.scratch().join("calls.json"))
    }
    pub fn planned(&self) {
        self.cli(&["run", "p01", "b001", "--plan-only"], "ok", true);
    }
    pub fn rows(&self) -> Vec<Value> {
        self.state()["completed"]["plan"]
            .as_array()
            .unwrap()
            .clone()
    }
    pub fn set_plan(&self, rows: &[Value], checked: bool) {
        let mut plan = HEADER.to_string();
        for row in rows {
            plan += &format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                string(&row["パス"]),
                string(&row["日付"]),
                string(&row["作り方"]),
                string(&row["内容"]),
                string(&row["関連"]),
                string(&row["先に要るもの"])
            );
        }
        write(&self.plan(), &plan);
        write(&self.root.join("fixture-plan.md"), plan);
        write(
            &self.root.join(format!("progress/{P}/b001.request.md")),
            format!("# 依頼\n範囲：月次報告\n件数の目安：{}\n", rows.len()),
        );
        if checked {
            let mut s = self.state();
            s["completed"]["plan"] = json!(rows);
            self.save(&s);
        }
    }
    pub fn overrides(&self, v: Value) {
        write(&self.root.join("draft-overrides.json"), v.to_string());
    }
    pub fn accepted(&self) {
        self.cli(&["accept", "p01", "b001"], "ok", true);
    }
    pub fn reset_scratch(&self) {
        fs::remove_dir_all(self.scratch()).unwrap();
    }
}
