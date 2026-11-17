use std::{fs, path::Path, process::Command};

#[test]
fn batch_refuses_admission_below_thirty_gigabytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("repo");
    for d in ["personas", "progress/p01-test", "scratch", "ops/bin"] {
        fs::create_dir_all(root.join(d)).unwrap();
    }
    fs::write(
        root.join("personas/p01-test.md"),
        "---\nperiod: 2026-01-01..2026-12-31\ntimezone: UTC\n---\n",
    )
    .unwrap();
    fs::write(
        root.join("progress/p01-test/b001.request.md"),
        "# request\n件数の目安：1\n",
    )
    .unwrap();
    let source = temp.path().join("space.c");
    let library = temp.path().join("space.so");
    fs::write(
        &source,
        r#"
#define _GNU_SOURCE
#include <sys/statvfs.h>
#include <dlfcn.h>
#include <stdlib.h>
#include <string.h>
int statvfs(const char *path, struct statvfs *out) {
    static int (*real_call)(const char *, struct statvfs *);
    if (!real_call) real_call=dlsym(RTLD_NEXT,"statvfs");
    int result=real_call(path,out);
    const char *target=getenv("CORPUS_LOW_SPACE_PATH");
    if (!result && target && !strcmp(path,target)) {
        out->f_bavail=29999999999ULL/out->f_frsize;
        out->f_bfree=out->f_bavail;
    }
    return result;
}
"#,
    )
    .unwrap();
    let output = Command::new("cc")
        .args(["-shared", "-fPIC"])
        .arg(&source)
        .arg("-o")
        .arg(&library)
        .arg("-ldl")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args(["batch", "start", "p01", "b001"])
        .current_dir(&root)
        .env("CORPUS_BASE", &root)
        .env("LD_PRELOAD", &library)
        .env("CORPUS_LOW_SPACE_PATH", &root)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("30,000,000,000"),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!root.join("scratch/p01-test/b001/state.json").exists());
    assert!(!Path::new(&root.join("ops/jobs/personaCorpus/p01-b001")).exists());
}
