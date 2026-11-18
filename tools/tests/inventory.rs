mod common;
use common::*;
use std::fs;
fn manifest(f: &Fixture) -> Vec<csv::StringRecord> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(f.base.join("manifests").join(format!("{}.tsv", f.name)))
        .unwrap()
        .records()
        .map(Result::unwrap)
        .collect()
}
#[test]
fn inventory_verify_restore_and_index_only_exclusion() {
    let f = Fixture::new();
    f.put("b.txt", BODY);
    f.put(".hidden", "hidden");
    f.put("a.py", "code");
    f.accept();
    success(&f.cli("manifest", &["p01"]));
    let rows = manifest(&f);
    assert_eq!(
        rows.iter().map(|r| &r[0]).collect::<Vec<_>>(),
        vec![".hidden", "a.py", "b.txt"]
    );
    assert_eq!(&rows[2][3], "2026-01-01T00:00:00Z");
    success(&f.cli("manifest", &["p01", "--verify"]));
    for change in ["content", "mtime", "missing", "size"] {
        let original = fs::read(f.home.join("b.txt")).unwrap();
        match change {
            "content" => {
                f.put("b.txt", "Changed");
            }
            "mtime" => f.time(&f.home.join("b.txt"), STAMP + 100),
            "missing" => fs::remove_file(f.home.join("b.txt")).unwrap(),
            _ => {
                let p = f.base.join("manifests").join(format!("{}.tsv", f.name));
                let text = fs::read_to_string(&p).unwrap();
                let changed = text
                    .lines()
                    .map(|l| {
                        let mut row = l.split('\t').collect::<Vec<_>>();
                        if row[0] == "b.txt" {
                            row[2] = "999"
                        }
                        row.join("\t")
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n";
                fs::write(p, changed).unwrap();
            }
        }
        failure(&f.cli("manifest", &["p01", "--verify"]));
        f.bytes("b.txt", &original);
    }
    let g = Fixture::new();
    g.put("accepted.txt", BODY);
    g.accept();
    g.put("current.txt", "Current approved report");
    g.put("other.txt", "Other work in progress");
    g.git(&["add", "people"]);
    success(&g.cli("manifest", &["p01", "--include", "current.txt"]));
    assert_eq!(
        manifest(&g)
            .iter()
            .map(|r| r[0].to_string())
            .collect::<Vec<_>>(),
        vec!["accepted.txt", "current.txt"]
    );
    failure(&g.cli("manifest", &["p01", "--verify"]));
    let h = Fixture::new();
    for n in ["b.txt", "c.txt"] {
        h.put(n, &format!("{BODY} {n}"));
    }
    h.accept();
    success(&h.cli("manifest", &["p01"]));
    for n in ["b.txt", "c.txt"] {
        h.time(&h.home.join(n), 1900000000)
    }
    failure(&h.cli("manifest", &["p01", "--verify"]));
    success(&h.cli("manifest", &["p01", "--restore"]));
    success(&h.cli("manifest", &["p01", "--verify"]));
    h.put("c.txt", "Changed content");
    h.time(&h.home.join("c.txt"), 1900000000);
    failure(&h.cli("manifest", &["p01", "--restore"]));
}
fn entries(path: &std::path::Path) -> Vec<(String, u64, u64, u64)> {
    let gz = flate2::read::GzDecoder::new(fs::File::open(path).unwrap());
    tar::Archive::new(gz)
        .entries()
        .unwrap()
        .map(|e| {
            let e = e.unwrap();
            (
                e.path().unwrap().to_string_lossy().into_owned(),
                e.header().uid().unwrap(),
                e.header().gid().unwrap(),
                e.header().mtime().unwrap(),
            )
        })
        .collect()
}
#[test]
fn pack_reproducible_accepted_only_and_gate() {
    use sha2::{Digest, Sha256};
    let f = Fixture::new();
    f.put("notes/a.txt", BODY);
    f.accept();
    success(&f.cli("manifest", &["p01"]));
    for n in ["out", "out2"] {
        success(&f.cli("pack", &[f.base.join(n).to_str().unwrap()]));
    }
    let archive = f.base.join("out").join(format!("{}.tar.gz", f.name));
    let bytes = fs::read(&archive).unwrap();
    assert_eq!(
        bytes,
        fs::read(f.base.join("out2").join(archive.file_name().unwrap())).unwrap()
    );
    assert_eq!(&bytes[4..8], &[0; 4]);
    assert_eq!(
        entries(&archive),
        vec![(format!("{}/notes/a.txt", f.name), 0, 0, STAMP as u64)]
    );
    let digest = format!("{:x}", Sha256::digest(&bytes));
    assert!(
        fs::read_to_string(f.base.join("out/SHA256SUMS"))
            .unwrap()
            .contains(&digest)
    );
    assert!(
        fs::read_to_string(f.base.join("out/README.md"))
            .unwrap()
            .contains("synthetic")
    );
    fs::create_dir_all(f.base.join("people/p03-new")).unwrap();
    fs::write(
        f.base.join("people/p03-new/unfinished.py"),
        "work in progress",
    )
    .unwrap();
    f.put("bad.txt", "placeholder");
    let dest = f.base.join("bad");
    success(&f.cli("pack", &[dest.to_str().unwrap()]));
    assert_eq!(entries(&dest.join(format!("{}.tar.gz", f.name))).len(), 1);
    fs::remove_file(f.home.join("bad.txt")).unwrap();
    f.put("notes/a.txt", "A changed decision");
    let dest = f.base.join("mismatch");
    failure(&f.cli("pack", &[dest.to_str().unwrap()]));
    assert!(!dest.exists());
}
#[test]
fn pack_locations_manifests_and_all_persona_checks() {
    let mut f = Fixture::new();
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    f.env
        .insert("CORPUS_BASE".into(), repo.display().to_string());
    f.put("a.txt", BODY);
    f.accept();
    let dest = f.base.join("missing");
    failure(&f.cli("pack", &[dest.to_str().unwrap()]));
    assert!(!dest.exists());
    success(&f.cli("manifest", &["p01"]));
    let nested = f.home.join("nested");
    failure(&f.cli("pack", &[nested.to_str().unwrap()]));
    assert!(!nested.exists());
    let protected = repo.join("bad-distribution");
    failure(&f.cli("pack", &[protected.to_str().unwrap()]));
    assert!(!protected.exists());
    f.write(&f.base.join("people/p02-editor/a.txt"), b"placeholder");
    f.accept();
    let out = f.base.join("out");
    failure(&f.cli("pack", &[out.to_str().unwrap(), "--persona", "p01"]));
    let p = f.base.join("people/p02-editor/a.txt");
    f.write(&p, b"The editor chose a new title after the review.");
    f.time(&p, STAMP);
    f.write(
        &f.base.join("sources/p02-editor/a.txt.md"),
        b"---\nkind: text\n---\nThe editor chose a new title after the review.",
    );
    f.accept();
    success(&f.cli("manifest", &["p02"]));
    success(&f.cli("pack", &[out.to_str().unwrap(), "--persona", "p01"]));
    let archives = fs::read_dir(out)
        .unwrap()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|s| s == "gz"))
        .count();
    assert_eq!(archives, 1);
}

#[test]
fn accepted_check_ignores_work_in_progress_and_rejects_missing_accepted_files() {
    let f = Fixture::new();
    f.put("accepted.txt", BODY);
    f.accept();
    f.bytes("in-progress.exe", b"incomplete");
    failure(&f.cli("check", &["--all"]));
    success(&f.cli("check", &["--all", "--accepted"]));
    fs::remove_file(f.home.join("accepted.txt")).unwrap();
    failure(&f.cli("check", &["--all", "--accepted"]));
}
