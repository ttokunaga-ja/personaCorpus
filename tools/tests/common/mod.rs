#![allow(dead_code)]
use filetime::{FileTime, set_file_mtime};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
pub const BODY: &str =
    "Minutes: Eleanor approved the revised delivery schedule after the supplier inspection.";
pub const STAMP: i64 = 1767225600;
pub struct Fixture {
    pub temp: tempfile::TempDir,
    pub base: PathBuf,
    pub home: PathBuf,
    pub name: String,
    pub env: BTreeMap<String, String>,
}
impl Fixture {
    pub fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().to_path_buf();
        let name = "p01-writer".to_string();
        let home = base.join("people").join(&name);
        fs::create_dir_all(&home).unwrap();
        for d in ["personas", "progress", "manifests", "sources"] {
            fs::create_dir_all(base.join(d)).unwrap()
        }
        let f = Self {
            temp,
            base,
            home,
            name,
            env: BTreeMap::new(),
        };
        f.cfg("period: 2025-01-01..2026-12-31\nallowed_terms: batch payment, synthetic lease\nphoto_dirs: photos");
        f
    }
    pub fn cfg(&self, text: &str) {
        self.write(
            &self.base.join("personas").join(format!("{}.md", self.name)),
            format!("---\n{text}\n---\n").as_bytes(),
        );
    }
    pub fn write(&self, p: &Path, bytes: &[u8]) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, bytes).unwrap();
    }
    pub fn put(&self, rel: &str, text: &str) -> PathBuf {
        let p = self.bytes(rel, text.as_bytes());
        self.source(rel, text, "");
        p
    }
    pub fn bytes(&self, rel: &str, bytes: &[u8]) -> PathBuf {
        let p = self.home.join(rel);
        self.write(&p, bytes);
        self.time(&p, STAMP);
        p
    }
    pub fn time(&self, p: &Path, t: i64) {
        set_file_mtime(p, FileTime::from_unix_time(t, 0)).unwrap()
    }
    pub fn source(&self, rel: &str, body: &str, derived: &str) -> PathBuf {
        let p = self
            .base
            .join("sources")
            .join(&self.name)
            .join(format!("{rel}.md"));
        self.write(
            &p,
            format!("---\npath: {rel}\nauthor: Eleanor\nderived_from: {derived}\n---\n{body}")
                .as_bytes(),
        );
        p
    }
    pub fn cli(&self, tool: &str, args: &[&str]) -> Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_corpus"));
        cmd.arg(tool).args(args);
        for n in ["root", "personas", "progress", "manifests"] {
            cmd.arg(format!("--{n}"))
                .arg(self.base.join(if n == "root" { "people" } else { n }));
        }
        if ["check", "pack"].contains(&tool) {
            cmd.arg("--sources").arg(self.base.join("sources"));
        }
        cmd.envs(&self.env);
        cmd.output().unwrap()
    }
    pub fn issues(&self, extra: &[&str]) -> (Output, Vec<[String; 4]>) {
        let report = self.base.join("report.tsv");
        let report = report.to_str().unwrap();
        let mut args = vec!["p01", "--report", report];
        args.extend_from_slice(extra);
        let output = self.cli("check", &args);
        assert!(
            [Some(0), Some(1)].contains(&output.status.code()),
            "{}",
            err(&output)
        );
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_path(report)
            .unwrap();
        let rows = reader
            .records()
            .map(|r| {
                let r = r.unwrap();
                assert_eq!(r.len(), 4);
                std::array::from_fn(|i| r[i].to_string())
            })
            .collect();
        (output, rows)
    }
    pub fn file_issues(&self, rel: &str) -> BTreeMap<String, String> {
        self.issues(&[])
            .1
            .into_iter()
            .filter(|r| r[1] == rel)
            .map(|r| (r[2].clone(), r[3].clone()))
            .collect()
    }
    pub fn git(&self, args: &[&str]) {
        let o = Command::new("git")
            .arg("-C")
            .arg(&self.base)
            .args(args)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", err(&o));
    }
    pub fn accept(&self) {
        self.git(&["init", "-q"]);
        self.git(&["add", "people"]);
        self.git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--allow-empty",
            "-qm",
            "Accepted files",
        ]);
    }
    pub fn plan(&self, rows: &[(&str, &str, &str)]) {
        let mut text = "| パス | 日付 | 内容 | 関連 |\n|---|---|---|---|\n".to_string();
        for (rel, date, body) in rows {
            text += &format!("| {rel} | {date} | {body} | |\n")
        }
        self.write(
            &self.base.join("progress").join(&self.name).join("b001.md"),
            text.as_bytes(),
        );
    }
    pub fn zip(&self, rel: &str, parts: &[(&str, &str)]) -> PathBuf {
        let p = self.home.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        let mut z = zip::ZipWriter::new(fs::File::create(&p).unwrap());
        for (name, text) in parts {
            z.start_file(*name, zip::write::SimpleFileOptions::default())
                .unwrap();
            z.write_all(text.as_bytes()).unwrap()
        }
        z.finish().unwrap();
        self.time(&p, STAMP);
        p
    }
    pub fn docx(&self, markup: &str, comments: &str) -> PathBuf {
        let body = format!(
            "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:body>{markup}</w:body></w:document>"
        );
        let comments = format!(
            "<w:comments xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:comment><w:p><w:r><w:t>{comments}</w:t></w:r></w:p></w:comment></w:comments>"
        );
        let mut parts = vec![
            ("[Content_Types].xml", "<Types/>"),
            ("_rels/.rels", "<Relationships/>"),
            ("word/document.xml", body.as_str()),
        ];
        if !comments.contains("<w:t></w:t>") {
            parts.push(("word/comments.xml", comments.as_str()))
        }
        self.zip("review.docx", &parts)
    }
    pub fn pdf_text(&mut self, extracted: &str, recognized: &str) {
        use std::os::unix::fs::PermissionsExt;
        let folder = self.base.join("external-commands");
        fs::create_dir_all(&folder).unwrap();
        for (name, text) in [
            ("pdftotext", extracted),
            ("pdfinfo", ""),
            ("pdftoppm", ""),
            ("tesseract", recognized),
        ] {
            let p = folder.join(name);
            let script = if name == "tesseract" {
                format!(
                    "#!/bin/sh\nif [ \"$1\" = --list-langs ]; then printf '%s\\n' eng; else printf '%s' {}; fi\n",
                    quote(text)
                )
            } else {
                format!("#!/bin/sh\nprintf '%s' {}\n", quote(text))
            };
            fs::write(&p, script).unwrap();
            fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap()
        }
        self.env.insert(
            "PATH".into(),
            format!(
                "{}:{}",
                folder.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        );
    }
}
pub fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
pub fn err(o: &Output) -> String {
    format!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
pub fn success(o: &Output) {
    assert!(o.status.success(), "{}", err(o));
}
pub fn failure(o: &Output) {
    assert_eq!(o.status.code(), Some(1), "{}", err(o));
}
impl Fixture {
    pub fn generate(&self, body: &str) {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let o=Command::new(repo.join(".venv/bin/python")).arg("-c").arg(format!("from pathlib import Path\nimport os\nhome=Path(os.environ['FIXTURE_HOME'])\nbase=home.parent.parent\n{body}")).env("FIXTURE_HOME",&self.home).output().unwrap();
        assert!(o.status.success(), "fixture generation: {}", err(&o));
    }
}
