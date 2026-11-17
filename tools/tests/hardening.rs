use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};
const BIN: &str = env!("CARGO_BIN_EXE_corpus");
fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}
struct Fixture {
    temp: TempDir,
    home: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().join("people/p01-writer");
        fs::create_dir_all(&home).unwrap();
        for d in ["personas", "progress", "sources/p01-writer", "manifests"] {
            fs::create_dir_all(temp.path().join(d)).unwrap();
        }
        fs::write(
            temp.path().join("personas/p01-writer.md"),
            "---\nperiod: 2025-01-01..2026-12-31\n---\n",
        )
        .unwrap();
        Self { temp, home }
    }
    fn source(&self, name: &str, text: &str) {
        fs::write(
            self.temp
                .path()
                .join("sources/p01-writer")
                .join(format!("{name}.md")),
            format!("---\nkind: text\n---\n{text}"),
        )
        .unwrap();
    }
    fn command(&self, tool: &str) -> Command {
        let mut c = Command::new(BIN);
        c.arg(tool);
        c
    }
    fn issues(&self, name: &str) -> BTreeMap<String, String> {
        let report = self.temp.path().join("report.tsv");
        let mut c = self.command("check");
        c.args(["p01", "--report"]).arg(&report);
        for d in ["root", "personas", "progress", "manifests", "sources"] {
            c.arg(format!("--{d}"))
                .arg(
                    self.temp
                        .path()
                        .join(if d == "root" { "people" } else { d }),
                );
        }
        let o = c.output().unwrap();
        assert!(
            matches!(o.status.code(), Some(0 | 1)),
            "{}",
            String::from_utf8_lossy(&o.stderr)
        );
        let mut r = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path(report)
            .unwrap();
        r.records()
            .map(Result::unwrap)
            .filter(|r| &r[1] == name)
            .map(|r| (r[2].into(), r[3].into()))
            .collect()
    }
    fn make(&self, kind: &str, name: &str) {
        let output = Command::new("uv")
            .args(["run", "--project"])
            .arg(project())
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hardening.py"))
            .arg(kind)
            .arg(self.home.join(name))
            .env(
                "UV_CACHE_DIR",
                std::env::temp_dir().join("personacorpus-uv-cache"),
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fn reason(&self, expected: &str, actual: &str, ext: &str) -> Option<String> {
        let name = format!("document.{ext}");
        let p = self.home.join(&name);
        if ["docx", "xlsx", "pptx"].contains(&ext) {
            let (main, tag, ns) = match ext {
                "docx" => (
                    "word/document.xml",
                    "document",
                    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                ),
                "xlsx" => (
                    "xl/workbook.xml",
                    "workbook",
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                ),
                _ => (
                    "ppt/presentation.xml",
                    "presentation",
                    "http://schemas.openxmlformats.org/presentationml/2006/main",
                ),
            };
            let mut z = ZipWriter::new(fs::File::create(&p).unwrap());
            for (n, text) in [
                ("[Content_Types].xml", "<Types/>".into()),
                ("_rels/.rels", "<Relationships/>".into()),
                (
                    main,
                    format!(
                        "<{tag} xmlns=\"{ns}\"><p><t>{}</t></p></{tag}>",
                        html_escape::encode_text(actual)
                    ),
                ),
            ] {
                z.start_file(n, SimpleFileOptions::default()).unwrap();
                z.write_all(text.as_bytes()).unwrap();
            }
            z.finish().unwrap();
        } else if ext == "eml" {
            fs::write(
                &p,
                format!("From: a@example.test\nTo: b@example.test\nSubject: Review\n\n{actual}"),
            )
            .unwrap();
        } else {
            fs::write(&p, actual).unwrap();
        }
        self.source(&name, expected);
        self.issues(&name).remove("source")
    }
}
fn good(o: Output) {
    assert!(
        o.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
}
fn parts(p: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut z = ZipArchive::new(fs::File::open(p).unwrap()).unwrap();
    (0..z.len())
        .map(|i| {
            let mut f = z.by_index(i).unwrap();
            let mut b = Vec::new();
            f.read_to_end(&mut b).unwrap();
            (f.name().into(), b)
        })
        .collect()
}
fn rewrite(p: &Path, data: &BTreeMap<String, Vec<u8>>) {
    let mut z = ZipWriter::new(fs::File::create(p).unwrap());
    for (n, b) in data {
        z.start_file(n, SimpleFileOptions::default()).unwrap();
        z.write_all(b).unwrap();
    }
    z.finish().unwrap();
}
#[test]
fn source_normalization() {
    let f = Fixture::new();
    for label in ["行", "#", "No."] {
        let expected = format!(
            "| {label} | 項目 | 差額 |\n|:---:|---|---:|\n| 7001 | 売上 | `=E7-D7` |\n| 7002 | 費用 | =SUM(E8:D8) |"
        );
        assert!(f.reason(&expected, "項目 差額 売上 費用", "xlsx").is_none());
    }
    for (body, actual, missing) in [
        (
            "| 行 | 金額 |\n|---|---|\n| 7654円 | 12345 |",
            "金額 12345",
            "7654",
        ),
        ("| No. | 金額 |\n|---|---|\n| 1 | 12345 |", "金額", "12345"),
        ("合計9876、式 `=SUM(E1:E2)`", "合計", "9876"),
    ] {
        assert!(f.reason(body, actual, "xlsx").unwrap().contains(missing));
    }
}
#[test]
fn text_thresholds() {
    let f = Fixture::new();
    let body = "alpha bravo charlie delta echo foxtrot golf hotel india juliet";
    let words = body.split_whitespace().collect::<Vec<_>>();
    for (ext, min) in [
        ("docx", 8),
        ("txt", 8),
        ("md", 8),
        ("log", 8),
        ("eml", 8),
        ("html", 8),
        ("pptx", 7),
    ] {
        assert!(
            f.reason(body, &words[..min].join(" "), ext).is_none(),
            "{ext}"
        );
        assert!(
            f.reason(body, &words[..min - 1].join(" "), ext).is_some(),
            "{ext}"
        );
    }
    for ext in ["xlsx", "csv", "tsv"] {
        assert!(f.reason(body, "alpha", ext).is_none());
    }
}
#[test]
fn presentation_notes_are_body() {
    let f = Fixture::new();
    f.make("notes", "deck.pptx");
    f.source("deck.pptx", "決算会議 発表者だけの説明 金額12400000");
    assert!(!f.issues("deck.pptx").contains_key("source"));
    f.make("notes-empty", "deck.pptx");
    assert!(f.issues("deck.pptx").contains_key("source"));
}
#[test]
fn formula_cache_types_and_errors() {
    let f = Fixture::new();
    f.make("formula", "book.xlsx");
    let p = f.home.join("book.xlsx");
    assert!(f.issues("book.xlsx").contains_key("formula"));
    let original = parts(&p);
    for (v, t, fails) in [
        ("2", "n", false),
        ("0", "n", false),
        ("", "str", false),
        ("", "n", true),
        ("false", "b", false),
        ("#REF!", "e", true),
        ("#DIV/0!", "e", true),
        ("#VALUE!", "e", true),
        ("#NAME?", "e", true),
        ("#N/A", "e", true),
    ] {
        let mut data = original.clone();
        let raw = String::from_utf8(data["xl/worksheets/sheet1.xml"].clone()).unwrap();
        let raw = raw
            .replace("<c r=\"A1\">", &format!("<c r=\"A1\" t=\"{t}\">"))
            .replace("<v></v>", &format!("<v>{v}</v>"));
        data.insert("xl/worksheets/sheet1.xml".into(), raw.into_bytes());
        rewrite(&p, &data);
        assert_eq!(
            f.issues("book.xlsx").contains_key("formula"),
            fails,
            "{v}/{t}"
        );
    }
}
fn without_cache(raw: &[u8]) -> xmltree::Element {
    fn strip(e: &mut xmltree::Element) {
        if e.name == "c" && e.get_child("f").is_some() {
            e.attributes.shift_remove("t");
            e.children
                .retain(|n| !matches!(n,xmltree::XMLNode::Element(c) if c.name=="v"||c.name=="is"));
        }
        for n in &mut e.children {
            if let xmltree::XMLNode::Element(c) = n {
                strip(c)
            }
        }
    }
    let mut e = xmltree::Element::parse(raw).unwrap();
    strip(&mut e);
    e
}
#[test]
fn recalc_preserves_everything_except_cache() {
    let f = Fixture::new();
    f.make("layout", "計算.xlsx");
    let p = f.home.join("計算.xlsx");
    let before = parts(&p);
    good(f.command("recalc").arg(&p).output().unwrap());
    let after = parts(&p);
    assert_eq!(
        before.keys().collect::<Vec<_>>(),
        after.keys().collect::<Vec<_>>()
    );
    for (n, b) in &before {
        if n.starts_with("xl/worksheets/") && n.ends_with(".xml") {
            assert_eq!(without_cache(b), without_cache(&after[n]), "{n}");
        } else {
            assert_eq!(b, &after[n], "{n}")
        }
    }
    assert!(!f.issues("計算.xlsx").contains_key("formula"));
    for n in ["xl/worksheets/sheet1.xml", "xl/worksheets/sheet2.xml"] {
        let s = String::from_utf8_lossy(&after[n]);
        assert!(s.contains("12400000"), "{s}");
    }
    f.make("error", "計算.xlsx");
    good(f.command("recalc").arg(&p).output().unwrap());
    assert!(f.issues("計算.xlsx")["formula"].contains("#DIV/0!"));
}
#[test]
fn recalc_refreshes_stale_manual_cache() {
    let f = Fixture::new();
    f.make("stale", "stale.xlsx");
    let p = f.home.join("stale.xlsx");
    good(f.command("recalc").arg(&p).output().unwrap());
    let data = parts(&p);
    let sheet = String::from_utf8_lossy(&data["xl/worksheets/sheet1.xml"]);
    assert!(sheet.contains(">10</"), "{sheet}");
    let book = String::from_utf8_lossy(&data["xl/workbook.xml"]);
    assert!(book.contains("calcMode=\"manual\""), "{book}");
}
#[test]
fn recalc_failure_is_atomic() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    f.make("formula", "bad.xlsx");
    let p = f.home.join("bad.xlsx");
    let before = fs::read(&p).unwrap();
    let stub = f.temp.path().join("stubs");
    fs::create_dir(&stub).unwrap();
    let exe = stub.join("soffice");
    fs::write(&exe, "#!/bin/sh\necho failure >&2\nexit 1\n").unwrap();
    fs::set_permissions(exe, fs::Permissions::from_mode(0o755)).unwrap();
    let o = f
        .command("recalc")
        .arg(&p)
        .env(
            "PATH",
            format!("{}:{}", stub.display(), std::env::var("PATH").unwrap()),
        )
        .output()
        .unwrap();
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(before, fs::read(p).unwrap());
}
