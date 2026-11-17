mod common;
use common::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
fn kinds(f: &Fixture) -> BTreeSet<String> {
    f.issues(&[]).1.into_iter().map(|r| r[2].clone()).collect()
}
fn present(f: &Fixture, rel: &str, kind: &str) -> String {
    let m = f.file_issues(rel);
    m.get(kind)
        .unwrap_or_else(|| panic!("missing {kind} for {rel}: {m:?}"))
        .clone()
}
fn absent(f: &Fixture, rel: &str, kind: &str) {
    let m = f.file_issues(rel);
    assert!(!m.contains_key(kind), "{m:?}")
}
#[test]
fn formats_paths_and_timestamps() {
    let f = Fixture::new();
    f.put("a.txt", BODY);
    assert!(!kinds(&f).contains("format"));
    f.put("bad.py", "print(1)");
    present(&f, "bad.py", "format");
    for rel in [".hidden.txt", "hidden/.a.txt", "zero.txt"] {
        f.bytes(rel, b"");
        present(&f, rel, "format");
    }
    for rel in [
        "foundation/notes.txt",
        "imagegen-assets/a.txt",
        "a-p01.txt",
        "memo-s123456.txt",
        "memo-12345.txt",
        "primary/a.txt",
    ] {
        f.put(rel, rel);
        present(&f, rel, "path");
    }
    assert_eq!(f.issues(&[]).1.iter().filter(|r| r[2] == "path").count(), 6);
    f.time(&f.home.join("a.txt"), 0);
    present(&f, "a.txt", "mtime");
    fs::remove_file(f.base.join("personas").join(format!("{}.md", f.name))).unwrap();
    let o = f.cli("check", &["p01"]);
    failure(&o);
    assert!(err(&o).contains("persona not found"));
}
#[test]
fn production_traces_and_secrets() {
    let f = Fixture::new();
    f.put("a.txt","A BATCH PAYMENT under the synthetic lease. Batchwork and scoping are normal. Scope of work.");
    absent(&f, "a.txt", "trace");
    f.put("b.txt", "Batch payment then batch 42.");
    present(&f, "b.txt", "trace");
    let terms = [
        "Kio",
        "kio-eval",
        "personaCorpus",
        "persona",
        "corpus",
        "fixture",
        "content spine",
        "synthetic",
        "fictional",
        "not a real",
        "artifact id",
        "source id",
        "batch",
        "scope X17",
        "p14-finance",
        "/Users/alice",
        "scratch/x",
        "lorem ipsum",
        "placeholder",
    ];
    for (i, t) in terms.iter().enumerate() {
        f.put(&format!("term-{i}.txt"), t);
    }
    let rows = f.issues(&[]).1;
    for i in 0..terms.len() {
        assert!(
            rows.iter()
                .any(|r| r[1] == format!("term-{i}.txt") && r[2] == "trace")
        )
    }
    f.put(
        "normal.txt",
        "【重要】申請書の保存先：共有フォルダ。河野さんの赤字を反映した。",
    );
    absent(&f, "normal.txt", "trace");
    for (i, t) in [
        "添付：受領控え.pdf（保存先：Documents/引渡し。本文への埋め込みなし）",
        "【河野隆之、2026年7月10日 11:40、削除・取り消し線】終了",
        "【同時刻、赤字の挿入】判断案",
        "【河野隆之、2026年7月10日 11:40、H-18への赤字追記】出典",
    ]
    .iter()
    .enumerate()
    {
        let rel = format!("jp-{i}.txt");
        f.put(&rel, t);
        present(&f, &rel, "trace");
    }
    f.put("synthetic-audio/note.txt", BODY);
    present(&f, "synthetic-audio/note.txt", "trace");
    f.put("mail/tagged.eml","From: a@x.example\nTo: b@x.example\nSubject: Delivery\nX-Synthetic-Corpus: yes\n\nThe supplier confirmed the revised delivery date.");
    present(&f, "mail/tagged.eml", "trace");
    let tokens = vec![
        format!("AKIA{}", "A".repeat(16)),
        format!("ghp_{}", "a".repeat(36)),
        format!("github_pat_{}", "a".repeat(30)),
        format!("xoxb-{}", "1".repeat(24)),
        format!("xapp-{}", "1".repeat(24)),
        "-----BEGIN PGP PRIVATE KEY BLOCK-----".into(),
        format!("hvs.{}", "a".repeat(24)),
        "-----BEGIN PRIVATE KEY-----".into(),
        format!("sk-{}", "a".repeat(24)),
        "eyJabc.eyJdef.abcdef".into(),
        "/ASCII85Decode".into(),
    ];
    for (i, t) in tokens.iter().enumerate() {
        f.put(&format!("key-{i}.txt"), t);
    }
    let rows = f.issues(&[]).1;
    for i in 0..tokens.len() {
        assert!(
            rows.iter()
                .any(|r| r[1] == format!("key-{i}.txt") && r[2] == "secret")
        )
    }
}
#[test]
fn office_runs_deleted_text_comments_and_review_coverage() {
    let f = Fixture::new();
    f.docx(
        "<w:p><w:r><w:t>syn</w:t></w:r><w:r><w:t>thetic</w:t></w:r></w:p>",
        "",
    );
    present(&f, "review.docx", "trace");
    f.docx(
        &format!(
            "<w:p><w:r><w:t>ghp_</w:t></w:r><w:r><w:t>{}</w:t></w:r></w:p>",
            "z".repeat(36)
        ),
        "",
    );
    present(&f, "review.docx", "secret");
    f.docx(
        "<w:p><w:r><w:t>fixture</w:t><w:br/></w:r><w:r><w:t>reference</w:t></w:r></w:p>",
        "",
    );
    present(&f, "review.docx", "trace");
    f.docx("<w:p><w:del><w:r><w:delText>original decision</w:delText></w:r></w:del><w:ins><w:r><w:t>revised decision</w:t></w:r></w:ins></w:p>","確認コメント");
    f.source(
        "review.docx",
        "original decision revised decision 確認コメント",
        "",
    );
    absent(&f, "review.docx", "source");
    let expected = "alpha beta gamma delta epsilon zeta eta theta iota kappa";
    for (markup, comments, pass) in [
        (
            "<w:p><w:del><w:r><w:delText>alpha beta gamma delta epsilon zeta</w:delText></w:r></w:del></w:p>",
            "",
            true,
        ),
        (
            "<w:p><w:r><w:t>alpha beta gamma delta epsilon zeta</w:t></w:r></w:p>",
            "review",
            true,
        ),
        (
            "<w:p><w:r><w:t>alpha beta gamma delta epsilon zeta</w:t></w:r></w:p>",
            "",
            false,
        ),
        (
            "<w:p><w:ins><w:r><w:t>alpha beta gamma delta epsilon</w:t></w:r></w:ins></w:p>",
            "",
            false,
        ),
    ] {
        f.docx(markup, comments);
        f.source("review.docx", expected, "");
        assert_eq!(!f.file_issues("review.docx").contains_key("source"), pass);
    }
}
fn source_case(
    f: &mut Fixture,
    expected: &str,
    actual: &str,
    ext: &str,
    scan: bool,
    derived: bool,
    review: bool,
) -> Option<String> {
    let rel = if review {
        "review.docx".into()
    } else {
        format!("document.{ext}")
    };
    if review {
        f.docx(
            &format!("<w:p><w:r><w:t>{actual}</w:t></w:r></w:p>"),
            "review",
        );
    } else {
        f.put(&rel, actual);
    }
    if derived {
        f.source("base.txt", expected, "");
        f.source(&rel, "", "base.txt");
    } else {
        f.source(&rel, expected, "");
    }
    if scan {
        f.bytes(&rel, b"%PDF fixture");
        f.pdf_text("\x0c", actual);
    }
    let issue = f.file_issues(&rel).get("source").cloned();
    f.env.clear();
    issue
}
#[test]
fn source_thresholds_numbers_and_derived_coverage() {
    let mut f = Fixture::new();
    let expected = "alpha beta gamma delta epsilon zeta eta theta iota kappa";
    assert!(
        source_case(
            &mut f,
            expected,
            "alpha beta gamma",
            "pdf",
            true,
            false,
            false
        )
        .is_none()
    );
    assert!(
        source_case(&mut f, expected, "alpha beta", "pdf", true, false, false)
            .unwrap()
            .contains("< 0.3")
    );
    assert!(
        source_case(
            &mut f,
            expected,
            "alpha beta gamma",
            "txt",
            false,
            false,
            false
        )
        .is_some()
    );
    let expected = "alpha beta gamma delta epsilon zeta eta theta iota 9876";
    for (actual, review, pass, threshold) in [
        ("alpha beta gamma delta epsilon", false, true, "0.5"),
        ("alpha beta gamma delta", false, false, "0.5"),
        ("alpha beta gamma delta epsilon", true, false, "0.6"),
        ("alpha beta gamma delta epsilon zeta", true, true, "0.6"),
    ] {
        let issue = source_case(&mut f, expected, actual, "txt", false, true, review);
        assert_eq!(issue.is_none(), pass);
        if let Some(s) = issue {
            assert!(s.contains(threshold))
        }
    }
    assert!(
        source_case(
            &mut f,
            expected,
            "alpha beta gamma",
            "pdf",
            true,
            true,
            false
        )
        .is_none()
    );
    let body = "alpha bravo charlie delta echo foxtrot golf hotel india 9876";
    for (actual, pass) in [
        (body, true),
        ("alpha bravo charlie delta echo foxtrot 9876", false),
        (
            "alpha bravo charlie delta echo foxtrot golf hotel 9876",
            true,
        ),
    ] {
        assert_eq!(
            source_case(&mut f, body, actual, "txt", false, false, false).is_none(),
            pass
        )
    }
    let prose = "The March reconciliation was confirmed by the controller after the bank certificate arrived and the receivable entry was reversed. ";
    assert!(source_case(&mut f,&format!("{prose}Bank 486,200,000円 on 2026年4月6日, invoice AS-2603-041, converted 240,939.60 and ledger 498,600,000円."),&format!("{prose}Bank 486200000 on 2026/4/6, invoice AS-2603-041, converted 240939.6 and ledger 498600000."),"txt",false,false,false).is_none());
    assert!(source_case(&mut f,&format!("{prose}Bank 486,200,000 and ledger 498,600,000 with items 7,200,000 and 3,100,000 and 1,300,000."),&format!("{prose}Bank 486200000 and ledger 498600000 with items 1111111 and 2222222."),"txt",false,false,false).unwrap().contains("7200000"));
    assert!(
        source_case(
            &mut f,
            "The adjustment is 2500円 after review.",
            "The adjustment is 1000円 after review.",
            "txt",
            false,
            false,
            false
        )
        .unwrap()
        .contains("2500")
    );
}
#[test]
fn table_sample_rows_and_prose_totals() {
    let mut f = Fixture::new();
    let manuscript = "列は受注番号,金額。行の作り方：1 日 214 行、合計 7,520,000 円、有効 7,000,000 円、取消 520,000 円。\n\n```csv\n受注番号,金額\nOR-1001,33000\nOR-1002,35000\nOR-1003,40000\nOR-1004,41000\nOR-1005,42000\n```\n";
    let built = "受注番号,金額\nOR-1001,33000\nOR-1002,35000\nOR-1003,40000\nOR-1004,41000\nOR-1005,42000\nOR-1006,50000\n";
    assert!(source_case(&mut f, manuscript, built, "csv", false, false, false).is_none());
    let dropped = built
        .replace("OR-1002,35000\n", "")
        .replace("OR-1003,40000\n", "");
    assert!(
        source_case(&mut f, manuscript, &dropped, "csv", false, false, false)
            .unwrap()
            .contains("missing source values")
    );
    let table = "| 受注番号 | 金額 |\n|---|---:|\n| OR-1001 | 33000 |\n| OR-1002 | 35000 |\n| OR-1003 | 40000 |\n| OR-1004 | 41000 |\n| OR-1005 | 42000 |\n\n合計 7,520,000 円";
    assert!(
        source_case(
            &mut f,
            table,
            &built.replace(',', "\t"),
            "tsv",
            false,
            false,
            false
        )
        .is_none()
    );
    assert!(
        source_case(
            &mut f,
            "合計 7,520,000 円、有効 7,000,000 円",
            built,
            "csv",
            false,
            false,
            false
        )
        .unwrap()
        .contains("missing source values")
    );
}
#[test]
fn visual_manuscripts_and_ocr_image_only_pdf() {
    let mut f = Fixture::new();
    for kind in [
        "photo",
        "handwriting",
        "scan:handwriting",
        "scan:handwriting:sheet",
    ] {
        f.bytes("paper.pdf", b"PDF");
        let p = f.source("paper.pdf", "Visible handwritten marks", "");
        fs::write(
            &p,
            format!("---\nkind: {kind}\n---\nVisible handwritten marks"),
        )
        .unwrap();
        f.pdf_text("\x0c", "unrelated OCR with sufficient characters");
        absent(&f, "paper.pdf", "source");
        fs::remove_file(p).unwrap();
        present(&f, "paper.pdf", "source");
    }
    let p = f.source("paper.pdf", "alpha beta gamma delta epsilon", "");
    fs::write(
        p,
        "---\nkind: scan:tex:sheet.cls\n---\nalpha beta gamma delta epsilon",
    )
    .unwrap();
    present(&f, "paper.pdf", "source");
    f.source(
        "paper.pdf",
        "alpha bravo charlie delta echo foxtrot golf hotel india juliet",
        "",
    );
    for (text, pass) in [("\x0c", true), ("alpha bravo charlie\x0c\x0c", false)] {
        f.pdf_text(text, "alpha bravo charlie");
        assert_eq!(!f.file_issues("paper.pdf").contains_key("source"), pass)
    }
    for rel in ["screen.png", "site.jpg"] {
        image::RgbImage::from_pixel(100, 100, image::Rgb([255, 255, 255]))
            .save(f.home.join(rel))
            .unwrap();
        f.source(
            rel,
            "確認画面。受付番号 R7-20260709-0142 と添付の名前が表示されている。",
            "",
        );
        f.pdf_text("", "unreadable");
        absent(&f, rel, "source");
    }
}
fn email(f: &Fixture, headers: &str, body: &str) -> String {
    let rel = "message.eml";
    f.put(rel,&format!("From: sender@example.test\nTo: recipient@example.test\nSubject: 確認\nMIME-Version: 1.0\nContent-Type: text/plain; charset=utf-8\nContent-Transfer-Encoding: 8bit\n{headers}\n\n{body}"));
    rel.into()
}
#[test]
fn email_identifier_headers() {
    let f = Fixture::new();
    for field in ["Message-ID", "In-Reply-To", "References"] {
        let expected = format!(
            "From: sender@example.test\n{field}: <fixed@example.test>\n\n案件の判断を確認しました。"
        );
        email(&f, &format!("{field}: <changed@example.test>"), &expected);
        f.source("message.eml", &expected, "");
        assert!(present(&f, "message.eml", "email-headers").contains(field));
    }
    email(&f, "Message-ID: <own@example.test>", "本文");
    f.source(
        "message.eml",
        "From: sender@example.test\nIn-Reply-To: <parent@example.test>\n\n本文",
        "",
    );
    assert!(present(&f, "message.eml", "email-headers").contains("In-Reply-To"));
    email(
        &f,
        "Message-ID: <study-postpone-260127@example.test>\nReferences: <first@example.test>\n <second@example.test>",
        "本文",
    );
    f.source("message.eml","From: sender@example.test  \nMessage-ID: <study-postpone-260127@example.test>  \nReferences: <first@example.test> <second@example.test>\n\n本文","");
    absent(&f, "message.eml", "email-headers");
    email(
        &f,
        "References: <second@example.test> <first@example.test>",
        "本文",
    );
    f.source(
        "message.eml",
        "References: <first@example.test> <second@example.test>\n\n本文",
        "",
    );
    assert!(present(&f, "message.eml", "email-headers").contains("References"));
    email(&f, "Message-ID: <own@example.test>", "本文");
    for expected in [
        "From: sender@example.test\nCc: なし\nIn-Reply-To: なし\nReferences: なし\n\nMessage-ID: <quoted@example.test>",
        "本文の説明\n\nMessage-ID: <quoted@example.test>",
    ] {
        f.source("message.eml", expected, "");
        absent(&f, "message.eml", "email-headers");
    }
}
fn attachments(
    f: &Fixture,
    manuscript: &str,
    attachments: &[(&str, &str)],
    forwarded: bool,
    attached_email: bool,
) -> BTreeMap<String, String> {
    let mut raw = format!(
        "From: sender@example.test\nTo: recipient@example.test\nSubject: 資料の送付\nMIME-Version: 1.0\nContent-Type: multipart/mixed; boundary=boundary\n\n--boundary\nContent-Type: text/plain; charset=utf-8\nContent-Transfer-Encoding: 8bit\n\n{manuscript}\n"
    );
    for (name, payload) in attachments {
        raw += &format!(
            "--boundary\nContent-Type: application/octet-stream\nContent-Disposition: attachment; filename=\"{name}\"\n\n{payload}\n"
        );
    }
    if forwarded {
        raw += &format!(
            "--boundary\nContent-Type: message/rfc822\nContent-Disposition: attachment{}\n\nFrom: old@example.test\nTo: next@example.test\nSubject: Earlier\nMIME-Version: 1.0\nContent-Type: multipart/mixed; boundary=nested\n\n--nested\nContent-Type: text/plain; charset=utf-8\n\n前のメール\n--nested\nContent-Type: application/pdf\nContent-Disposition: attachment; filename=report.pdf\n\nold report\n--nested--\n",
            if attached_email {
                "; filename=forwarded.eml"
            } else {
                ""
            }
        );
    }
    raw += "--boundary--\n";
    f.put("message.eml", &raw);
    f.source("message.eml", manuscript, "");
    f.file_issues("message.eml")
}
#[test]
fn mime_attachments_match_declared_current_message() {
    let f = Fixture::new();
    let issues = attachments(&f, "資料を送ります。\n添付: report.pdf", &[], false, false);
    assert!(!issues.contains_key("source"));
    assert!(issues["email-attachments"].contains("report.pdf"));
    let issues = attachments(
        &f,
        "添付：report.pdf、明細.xlsx",
        &[("report.pdf", "PDF")],
        false,
        false,
    );
    assert!(issues["email-attachments"].contains("明細.xlsx"));
    for parts in [vec![("report.pdf", "")], vec![("other.pdf", "PDF")]] {
        assert!(
            attachments(&f, "添付: report.pdf", &parts, false, false)
                .contains_key("email-attachments")
        );
    }
    for manuscript in [
        "添付: Documents/報告.pdf",
        "添付（実ファイル）：\n- Documents/報告.pdf",
        "Attachments:\n- `Monthly Financial Review.pdf`",
    ] {
        let name = if manuscript.contains("Monthly") {
            "Monthly Financial Review.pdf"
        } else {
            "報告.pdf"
        };
        assert!(
            !attachments(&f, manuscript, &[(name, "contents")], false, false)
                .contains_key("email-attachments")
        );
        assert!(attachments(&f, manuscript, &[], false, false).contains_key("email-attachments"));
    }
    for manuscript in [
        "添付: なし\n> 添付: report.pdf",
        "Attachment: none\n> Attachment: report.pdf",
        "report.pdf を共有フォルダで参照してください。",
        "添付: なし\n\n- report.pdf",
        "Attachments: first.pdf, second.pdf",
        "添付: report.pdf は前便で送付済みです。",
    ] {
        assert!(!attachments(&f, manuscript, &[], false, false).contains_key("email-attachments"));
    }
    assert!(
        attachments(&f, "添付: report.pdf", &[], true, false).contains_key("email-attachments")
    );
    assert!(
        !attachments(&f, "添付: forwarded.eml", &[], true, true).contains_key("email-attachments")
    );
}
const WORDS: &str = "alpha bravo charlie delta echo foxtrot golf hotel india juliet kilo lima mike november oscar papa quebec romeo sierra tango uniform victor whiskey xray yankee zulu";
fn duplicate_map(f: &Fixture) -> BTreeMap<String, String> {
    f.issues(&[])
        .1
        .into_iter()
        .filter(|r| r[2] == "duplicate")
        .map(|r| (r[1].clone(), r[3].clone()))
        .collect()
}
#[test]
fn duplicates_exact_normalized_japanese_relations_and_siblings() {
    let f = Fixture::new();
    f.put("only.txt", WORDS);
    assert!(duplicate_map(&f).is_empty());
    fs::remove_file(f.home.join("only.txt")).unwrap();
    f.put("before.txt", WORDS);
    f.put("after.txt", &(WORDS.to_string() + " extra"));
    let m = duplicate_map(&f);
    assert!(m["before.txt"].ends_with("after.txt"));
    assert!(m["after.txt"].ends_with("before.txt"));
    for text in [WORDS.to_string() + " extra", WORDS.to_string() + "\n"] {
        f.put("after.txt", &text);
        f.source("after.txt", "", "before.txt");
        assert!(duplicate_map(&f).is_empty())
    }
    f.put("after.txt", WORDS);
    f.source("after.txt", "", "before.txt");
    let m = duplicate_map(&f);
    assert_eq!(m.len(), 2);
    assert!(m.values().all(|s| s.contains("identical bytes")));
    f.put("after.txt", &(WORDS.to_string() + "\n"));
    f.source("after.txt", "", "before.txt");
    f.put("sibling.md", &(WORDS.to_string() + " \n"));
    f.source("sibling.md", "", "before.txt");
    assert_eq!(
        duplicate_map(&f).into_keys().collect::<BTreeSet<_>>(),
        BTreeSet::from(["after.txt".into(), "sibling.md".into()])
    );
    let g = Fixture::new();
    let body = "三月の月次締めでは銀行残高と帳簿残高の差異を確認し、入金のタイミングと支払のタイミングに分けて原因を整理した。差異は1240万円で、翌月初に解消する見込みである。";
    g.put("ja-a.txt", body);
    g.put("ja-b.txt","静岡工場の棚卸では仕掛品の移動伝票に入力漏れがあり、管理課長と現物を照合して修正仕訳を起こした。再発防止として週次の点検を始める。");
    assert!(duplicate_map(&g).is_empty());
    g.put("ja-c.txt", &body.replace("1240", "380"));
    assert_eq!(
        duplicate_map(&g).into_keys().collect::<BTreeSet<_>>(),
        BTreeSet::from(["ja-a.txt".into(), "ja-c.txt".into()])
    );
    let h = Fixture::new();
    h.put("a.txt", &(WORDS.to_string() + " 2025-03-01 cost 42"));
    h.put(
        "b.txt",
        "The rain damaged the northern retaining wall and delayed the station opening.",
    );
    assert!(duplicate_map(&h).is_empty());
    h.put("c.txt", &(WORDS.to_string() + " 2025-03-01 cost 42"));
    h.put(
        "d.txt",
        &(WORDS.to_string() + " 2026-08-02 cost 97 revised"),
    );
    assert_eq!(
        duplicate_map(&h).into_keys().collect::<BTreeSet<_>>(),
        BTreeSet::from(["a.txt".into(), "c.txt".into(), "d.txt".into()])
    );
}
#[test]
fn duplicate_prefix_join_matches_bruteforce() {
    use rand::{Rng, SeedableRng, seq::SliceRandom};
    let f = Fixture::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let alphabet = (0u32..400)
        .map(|i| {
            format!(
                "token{}{}",
                char::from_u32(97 + i / 26).unwrap(),
                char::from_u32(97 + i % 26).unwrap()
            )
        })
        .collect::<Vec<_>>();
    let mut texts = Vec::new();
    for _ in 0..35 {
        let mut shuffled = alphabet.clone();
        shuffled.shuffle(&mut rng);
        let n = rng.random_range(25..65);
        let words = shuffled[..n].to_vec();
        texts.push(words.clone());
        texts.push([words.clone(), shuffled[n..n + 2].to_vec()].concat());
        texts.push([words[..10].to_vec(), shuffled[n..n + 20].to_vec()].concat());
    }
    let grams = texts
        .iter()
        .map(|words| {
            words
                .windows(5)
                .map(|s| s.join(" "))
                .collect::<BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    let mut expected = BTreeSet::new();
    for (i, a) in grams.iter().enumerate() {
        for (j, b) in grams.iter().enumerate() {
            if i != j && a.intersection(b).count() as f64 / a.union(b).count() as f64 >= 0.8 {
                expected.insert(format!("note-{i}.txt"));
            }
        }
        f.put(&format!("note-{i}.txt"), &texts[i].join(" "));
    }
    assert_eq!(
        duplicate_map(&f).into_keys().collect::<BTreeSet<_>>(),
        expected
    );
}
#[test]
fn plan_pending_discarded_and_cross_batch_relations() {
    let f = Fixture::new();
    f.put("good.txt", BODY);
    f.plan(&[
        ("missing.pdf", "2026-01-01", "成形待ち：失敗。内容"),
        ("removed.pdf", "2026-01-01", "破棄：失敗。内容"),
        ("good.txt", "2026-01-01", "内容"),
    ]);
    let (o, rows) = f.issues(&["--batch", "b001"]);
    success(&o);
    assert!(rows.is_empty());
    let g = Fixture::new();
    g.put("before.txt", WORDS);
    for rel in ["after.txt", "third.txt", "fourth.txt"] {
        for previous in ["after.txt", "third.txt", "fourth.txt"] {
            let _ = fs::remove_file(g.home.join(previous));
        }
        g.put(rel, &(WORDS.to_string() + " extra"));
        for(batch,row)in[("b001","| before.txt | 2026-01-01 | 破棄：previous check | after.txt；third.txt | fourth.txt |".into()),("b002",format!("| {rel} | 2026-01-02 | Text | | |"))]{g.write(&g.base.join("progress").join(&g.name).join(format!("{batch}.md")),format!("| パス | 日付 | 内容 | 関連 | 先に要るもの |\n|---|---|---|---|---|\n{row}\n").as_bytes());}
        assert!(duplicate_map(&g).is_empty());
    }
}
#[test]
fn batch_membership_dates_tracked_v4_and_plan_failures() {
    let f = Fixture::new();
    f.put("a.txt", BODY);
    f.plan(&[
        ("a.txt", "2026-01-01", "Meeting decision"),
        ("discarded.txt", "2026-01-01", "破棄：bad"),
    ]);
    success(&f.issues(&["--batch", "b001"]).0);
    f.put("extra.txt", "Extra notes");
    fs::remove_file(f.home.join("a.txt")).unwrap();
    let rows = f.issues(&["--batch", "b001"]).1;
    assert_eq!(
        rows.iter()
            .filter(|r| r[2] == "plan")
            .map(|r| r[1].clone())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["a.txt".into(), "extra.txt".into()])
    );
    let g = Fixture::new();
    g.put("a.txt", BODY);
    g.put(
        "other.txt",
        "Other report: revised warehouse allocation approved by management.",
    );
    g.plan(&[("a.txt", "2026-01-01", "Decision")]);
    g.write(
        &g.base.join("progress").join(&g.name).join("b002.md"),
        "| パス | 日付 | 内容 | 関連 |\n|---|---|---|---|\n| other.txt | 2026-01-01 | Report | |\n"
            .as_bytes(),
    );
    success(&g.issues(&["--batch", "b001"]).0);
    g.put("other.txt", BODY);
    let rows = g.issues(&["--batch", "b001"]).1;
    assert!(rows.iter().any(|r| r[1] == "a.txt" && r[2] == "duplicate"));
    assert!(!rows.iter().any(|r| r[2] == "plan"));
    let h = Fixture::new();
    h.put("a.txt", BODY);
    h.put("existing.txt", BODY);
    h.git(&["init", "-q"]);
    h.git(&["add", "people"]);
    h.plan(&[("a.txt", "2026-01-02", "Decision")]);
    let rows = h.issues(&["--batch", "b001"]).1;
    assert_eq!(
        rows.iter().map(|r| r[2].clone()).collect::<BTreeSet<_>>(),
        BTreeSet::from(["mtime".into(), "duplicate".into()])
    );
    assert!(rows.iter().all(|r| r[1] == "a.txt"));
    h.git(&["update-index", "--index-version", "4"]);
    assert!(
        !h.issues(&["--batch", "b001"])
            .1
            .iter()
            .any(|r| r[2] == "plan")
    );
}
#[test]
fn manuscript_required_all_batch_photo_and_derived_errors() {
    let f = Fixture::new();
    f.put(
        "a.txt",
        "銀行残高との差異は1,240万円。2026-01-01に確認した。",
    );
    success(&f.issues(&[]).0);
    f.source(
        "a.txt",
        "銀行残高との差異は2,400万円。2026-01-02に確認した。",
        "",
    );
    assert!(present(&f, "a.txt", "source").contains("2400"));
    let p = f.source("a.txt", BODY, "");
    fs::remove_file(&p).unwrap();
    f.plan(&[("a.txt", "2026-01-01", "Decision")]);
    for extra in [vec![], vec!["--batch", "b001"]] {
        assert!(f.issues(&extra).1.iter().any(|r| r[2] == "source"));
    }
    assert!(err(&f.cli("check", &["--all"])).contains("source"));
    f.source("a.txt", "", "missing.txt");
    present(&f, "a.txt", "source");
    f.source("a.txt", "", "a.txt");
    assert!(present(&f, "a.txt", "source").contains("cycle"));
    fs::remove_file(&p).unwrap();
    std::os::unix::fs::symlink(f.home.join("a.txt"), &p).unwrap();
    assert!(present(&f, "a.txt", "source").contains("symbolic"));
    fs::create_dir_all(f.home.join("photos")).unwrap();
    image::RgbImage::from_pixel(100, 100, image::Rgb([255, 255, 255]))
        .save(f.home.join("photos/wall.jpg"))
        .unwrap();
    present(&f, "photos/wall.jpg", "source");
    let p = f.source("photos/wall.jpg", "A blank office wall", "");
    fs::write(p, "---\nkind: photo\n---\nA blank office wall").unwrap();
    absent(&f, "photos/wall.jpg", "source");
    let g = Fixture::new();
    let body = "The quarterly review listed revenue of 800000000 and the controller explained the receivable correction to the finance director. ";
    g.put(
        "deck.txt",
        &(body.to_string() + "Speaker note: accruals 2000000 and freight 900000 stay open."),
    );
    g.put("deck-handout.md", body);
    g.source("deck-handout.md", "No differences.", "deck.txt");
    assert!(!kinds(&g).contains("source"));
}
#[test]
fn opens_supported_and_rejects_corruption() {
    let f = Fixture::new();
    f.generate(r#"
from docx import Document
from openpyxl import Workbook
from pptx import Presentation
from PIL import Image
from reportlab.pdfgen.canvas import Canvas
import wave, zipfile
for ext,text in {'txt':'A useful note','md':'# Notes','log':'Meeting started','csv':'a,b\n1,2\n','tsv':'a\tb\n1\t2\n','json':'{"cost":42}','xml':'<cost>42</cost>','html':'<p>Decision approved</p>','eml':'From: a@example.test\nTo: b@example.test\nSubject: Review\n\nApproved'}.items():
 (home/('a.'+ext)).write_text(text)
Document().save(home/'a.docx')
Workbook().save(home/'a.xlsx')
Presentation().save(home/'a.pptx')
Image.new('RGB',(50,50),'white').save(home/'a.png')
Image.new('RGB',(50,50),'blue').save(home/'a.jpg')
c=Canvas(str(home/'a.pdf'),pageCompression=0); c.drawString(30,700,'Meeting approved for next Tuesday'); c.save()
with wave.open(str(home/'a.wav'),'wb') as w:
 w.setparams((1,2,8000,0,'NONE','not compressed')); w.writeframes(b'\0\0'*800)
with zipfile.ZipFile(home/'a.zip','w') as z: z.writestr('meeting.txt','Decisions')
"#);
    let extensions = [
        "txt", "md", "log", "csv", "tsv", "json", "xml", "html", "eml", "docx", "xlsx", "pptx",
        "png", "jpg", "pdf", "wav", "zip",
    ];
    let rows = f.issues(&[]).1;
    assert!(!rows.iter().any(|r| r[2] == "opens"), "{rows:?}");
    for ext in extensions {
        f.bytes(&format!("a.{ext}"), b"\xffbroken\0");
    }
    let rows = f.issues(&[]).1;
    for ext in extensions {
        assert!(
            rows.iter()
                .any(|r| r[1] == format!("a.{ext}") && r[2] == "opens"),
            "{ext}: {rows:?}"
        )
    }
    f.bytes("wrong.wav", b"not audio");
    present(&f, "wrong.wav", "opens");
}
#[test]
fn readable_blank_and_photo_scans() {
    let f = Fixture::new();
    f.generate(r#"
from PIL import Image,ImageDraw,ImageFont
im=Image.new('RGB',(1300,180),'white')
ImageDraw.Draw(im).text((20,40),'Meeting approved after supplier inspection on Tuesday',font=ImageFont.truetype('DejaVuSans.ttf',32),fill='black')
im.save(home/'readable.png')
Image.new('RGB',(150,150),'white').save(home/'blank.png')
(home/'photos').mkdir();Image.new('RGB',(160,150),'white').save(home/'photos/blank.png')
Image.new('RGB',(150,150),'white').save(home/'blank.pdf','PDF')
"#);
    let rows = f.issues(&[]).1;
    assert!(
        !rows
            .iter()
            .any(|r| r[1] == "readable.png" && r[2] == "empty-scan")
    );
    for rel in ["blank.png", "blank.pdf"] {
        assert!(
            rows.iter().any(|r| r[1] == rel && r[2] == "empty-scan"),
            "{rows:?}"
        );
    }
    assert!(
        !rows
            .iter()
            .any(|r| r[1] == "photos/blank.png" && r[2] == "empty-scan")
    );
    success(&f.cli("check", &["--self-test"]));
}
#[test]
fn missing_ocr_is_pending_not_open_failure() {
    let mut f = Fixture::new();
    image::RgbImage::from_pixel(80, 80, image::Rgb([255, 255, 255]))
        .save(f.home.join("scan.png"))
        .unwrap();
    f.source("scan.png", "Scan of a blank sheet.", "");
    f.env.insert(
        "PATH".into(),
        f.base.join("no-tools").to_string_lossy().into_owned(),
    );
    let (o, rows) = f.issues(&[]);
    failure(&o);
    assert!(
        !rows
            .iter()
            .any(|r| ["opens", "empty-scan"].contains(&r[2].as_str()))
    );
    let out = err(&o);
    assert!(out.contains("未実行：empty-scan"));
    assert!(out.contains("passed: 0"));
    assert!(out.contains("pending: 1"));
}
#[test]
fn source_adjacent_csv_cells_large_tables_and_chart_values() {
    let f = Fixture::new();
    f.put("claims.csv","申請番号,金額,承認日\nEX-2512-041,12400,2025-12-18\nEX-2512-042,11600,2025-12-19\nEX-2512-043,10400,2025-12-22\n");
    f.source("claims.csv","| 申請番号 | 金額 | 承認日 |\n|---|---:|---|\n| EX-2512-041 | 12,400 | 2025-12-18 |\n| EX-2512-042 | 11,600 | 2025-12-19 |\n| EX-2512-043 | 10,400 | 2025-12-22 |","");
    absent(&f, "claims.csv", "source");
    let mut rows = "番号,担当,状態\n".to_string();
    for i in 0..184 {
        rows += &format!("R-{i:03},担当{},確認済み\n", i % 7)
    }
    f.put("rows.csv", &rows);
    f.source("rows.csv","列は番号・担当・状態。R-000 から R-183 までの 184 行。担当は 7 人で順に割り当てる。状態は全件が確認済み。\n\n| 番号 | 担当 | 状態 |\n|---|---|---|\n| R-000 | 担当0 | 確認済み |","");
    absent(&f, "rows.csv", "source");
    let prose = "The controller reported the January close to the finance director and explained the duplicate expense claims found after the system migration. ";
    f.put(
        "report.txt",
        &(prose.to_string() + "Duplicates totalled 2480000 yen."),
    );
    f.source("report.txt",&(prose.to_string()+"Duplicates totalled 2,480,000 yen.\n\n図：棒グラフ。縦軸は 0〜3,500,000 円、500,000 円刻み。棒の値は 2,600,000 と 3,412,300。"),"");
    absent(&f, "report.txt", "source");
    let body = "The import of customer inquiries failed for the equipment dealer and the support lead asked the vendor to review the attached file today.";
    let issues = attachments(
        &f,
        &format!("{body}\n\n添付: 問合せ取込_0710.csv"),
        &[("問合せ取込_0710.csv", "id,name\n1,a")],
        false,
        false,
    );
    assert!(!issues.contains_key("source"), "{issues:?}");
}
#[test]
fn source_formats_text_office_email_and_scan() {
    let f = Fixture::new();
    let body = "The finance committee approved payment of 1240 dollars on 2026-01-01 after reviewing the supplier invoice and delivery inspection.";
    f.generate(r#"
from docx import Document
from openpyxl import Workbook
from reportlab.pdfgen.canvas import Canvas
from PIL import Image
import subprocess, shutil
body='The finance committee approved payment of 1240 dollars on 2026-01-01 after reviewing the supplier invoice and delivery inspection.'
d=Document(); d.add_paragraph(body); d.save(home/'report.docx')
w=Workbook();w.active.append([body]);w.save(home/'report.xlsx')
(home/'report.eml').write_text('From: a@example.test\nTo: b@example.test\nSubject: Payment\n\n'+body)
original=base/'original.pdf';c=Canvas(str(original));c.drawString(20,700,body);c.save();shutil.copyfile(original,home/'report.pdf')
subprocess.run(['pdftoppm','-singlefile','-r','180','-png',str(original),str(base/'scan')],check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
with Image.open(base/'scan.png') as im:im.convert('RGB').save(home/'report.scan.pdf','PDF',resolution=180)
"#);
    for ext in ["docx", "xlsx", "pdf", "eml", "scan.pdf"] {
        let rel = format!("report.{ext}");
        f.source(&rel, body, "");
        absent(&f, &rel, "source");
        f.source(&rel, &body.replace("1240", "8900"), "");
        let issues = f.file_issues(&rel);
        assert_eq!(
            issues.get("source").is_some_and(|s| s.contains("8900")),
            ext != "scan.pdf",
            "{ext}: {issues:?}"
        );
    }
}
#[test]
fn metadata_office_comment_revision_and_pdf_xmp() {
    let f = Fixture::new();
    f.generate(r#"
from docx import Document
import datetime as dt, zipfile, io
p=home/'meeting.docx';d=Document();d.add_paragraph('Meeting approved after the inspection')
cp=d.core_properties;cp.comments='';cp.author='Eleanor';cp.last_modified_by='Eleanor';cp.created=dt.datetime(2025,1,1);cp.modified=dt.datetime(2025,1,2);d.save(p);os.utime(p,(1767225600,1767225600))
"#);
    f.source("meeting.docx", "Meeting approved after the inspection", "");
    absent(&f, "meeting.docx", "metadata");
    f.generate(r#"
from docx import Document
import datetime as dt
p=home/'meeting.docx';d=Document(p);d.core_properties.author='python-docx';d.core_properties.modified=dt.datetime(2026,11,16);d.save(p);os.utime(p,(1767225600,1767225600))
"#);
    present(&f, "meeting.docx", "metadata");
    for (part, xml) in [
        (
            "word/comments.xml",
            "<comments><comment author=\"Codex\" date=\"2025-01-01T00:00:00Z\"/></comments>",
        ),
        (
            "word/revisions.xml",
            "<document><ins author=\"Eleanor\" date=\"2026-11-16T00:00:00Z\"/></document>",
        ),
    ] {
        f.zip(
            "comments.docx",
            &[
                ("[Content_Types].xml", "<Types/>"),
                ("_rels/.rels", "<Relationships/>"),
                ("word/document.xml", "<document/>"),
                (part, xml),
            ],
        );
        present(&f, "comments.docx", "metadata");
    }
    for content in [
        "<metadata><CreateDate>2026-11-16T00:00:00Z</CreateDate></metadata>",
        "<metadata><creator><Seq><li>Codex</li></Seq></creator></metadata>",
    ] {
        let encoded = serde_json::to_string(content).unwrap();
        f.generate(&format!(r#"
from pypdf import PdfWriter
from pypdf.generic import DecodedStreamObject,NameObject
import json
w=PdfWriter();w.add_blank_page(100,100);w.add_metadata({{'/Producer':'Office scanner'}})
stream=DecodedStreamObject();stream.set_data({encoded}.encode());stream[NameObject('/Type')]=NameObject('/Metadata');stream[NameObject('/Subtype')]=NameObject('/XML');w._root_object[NameObject('/Metadata')]=w._add_object(stream);w.write(home/'xmp.pdf');os.utime(home/'xmp.pdf',(1767225600,1767225600))
"#));
        let reason = present(&f, "xmp.pdf", "metadata");
        if content.contains("Codex") {
            assert!(reason.contains("Codex"))
        }
    }
}
#[test]
fn metadata_pdf_png_jpeg_and_persona_timezone() {
    let f = Fixture::new();
    for meta in [
        serde_json::json!({"/Author":"Eleanor","/Producer":"Finance scanner","/CreationDate":"D:20250101100000+09'00'"}),
        serde_json::json!({"/Producer":"Finance scanner","/Author":"ReportLab"}),
        serde_json::json!({"/Producer":"Finance scanner","/CreationDate":"D:20261116120000Z"}),
        serde_json::json!({"/Producer":"Finance scanner","/CreationDate":"D:20240101120000Z"}),
        serde_json::json!({"/Producer":"Finance scanner","/Creator":"/home/alice/work/make.py"}),
    ] {
        let good = meta.get("/Author").is_some_and(|v| v == "Eleanor");
        f.generate(&format!(r#"
from pypdf import PdfWriter
import json
w=PdfWriter();w.add_blank_page(100,100);w.add_metadata({meta});w.write(home/'paper.pdf');os.utime(home/'paper.pdf',(1767225600,1767225600))
"#));
        assert_eq!(!f.file_issues("paper.pdf").contains_key("metadata"), good);
    }
    for ext in ["png", "jpg"] {
        for bad in [false, true] {
            let stamp = if bad {
                "2026-11-16T10:00:00Z"
            } else {
                "2025-01-01T10:00:00Z"
            };
            f.generate(&format!(r#"
from PIL import Image,PngImagePlugin
im=Image.new('RGB',(100,100),'white')
if '{ext}'=='png':
 info=PngImagePlugin.PngInfo();info.add_text('Author','Eleanor');info.add_text('Creation Time','{stamp}');im.save(home/'photo.png',pnginfo=info)
else:
 exif=Image.Exif();exif[315]='Eleanor';exif[306]='{stamp}'.replace('-',':').replace('T',' ').rstrip('Z');im.save(home/'photo.jpg',exif=exif)
os.utime(home/'photo.{ext}',(1767225600,1767225600))
"#));
            assert_eq!(
                f.file_issues(&format!("photo.{ext}"))
                    .contains_key("metadata"),
                bad
            );
        }
        fs::remove_file(f.home.join(format!("photo.{ext}"))).unwrap();
    }
    f.cfg("period: 2025-01-01..2026-12-31\ntimezone: Asia/Tokyo");
    f.generate(r#"
from PIL import Image
import datetime as dt
p=home/'portrait.jpg';im=Image.new('RGB',(100,100),'white');exif=Image.Exif();exif[306]='2026:03:06 12:00:00';im.save(p,exif=exif)
stamp=dt.datetime(2026,3,6,3,tzinfo=dt.timezone.utc).timestamp();os.utime(p,(stamp,stamp))
"#);
    absent(&f, "portrait.jpg", "metadata");
}
#[test]
fn malformed_address_headers_are_open_failures() {
    let f = Fixture::new();
    for address in ["bad", "Eleanor", "abc <nope>"] {
        f.put(
            "invalid.eml",
            &format!("From: {address}\nTo: recipient@example.test\nSubject: Review\n\nApproved"),
        );
        present(&f, "invalid.eml", "opens");
    }
}
#[test]
fn jpeg_xmp_identity_and_dates_are_metadata() {
    let f = Fixture::new();
    for content in [
        "<metadata><creator>Codex</creator></metadata>",
        "<metadata><CreateDate>2026-11-16T00:00:00Z</CreateDate></metadata>",
    ] {
        let encoded = serde_json::to_string(content).unwrap();
        f.generate(&format!(
            r#"
from PIL import Image
import struct
p=home/'xmp.jpg';Image.new('RGB',(100,100),'white').save(p)
raw=p.read_bytes();payload=b'http://ns.adobe.com/xap/1.0/\0'+{encoded}.encode()
p.write_bytes(raw[:2]+b'\xff\xe1'+struct.pack('>H',len(payload)+2)+payload+raw[2:])
os.utime(p,(1767225600,1767225600))
"#
        ));
        let reason = present(&f, "xmp.jpg", "metadata");
        if content.contains("Codex") {
            assert!(reason.contains("Codex"));
        }
    }
}
