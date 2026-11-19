mod common;
use common::Fixture;
use std::fs;

#[test]
fn retries_image_only_scan_ocr_against_the_transcribed_pages() {
    let mut fixture = Fixture::new();
    let file = "letter.pdf";
    fixture.bytes(file, b"PDF");
    let manuscript = "## Page 1 — letter number 44\n\n1. Collection 1842 4137 5840 6971 7950 8651 6013\n\n## Paper and scan\n\nShadows along the folded corner and light water marks across both sides of the surface. The edge curls toward the reader where the paper was folded.";
    let source = fixture.source(file, manuscript, "");
    let header = "---\nkind: scan:manuscript.pdf\n---\n";
    fs::write(&source, format!("{header}{manuscript}")).unwrap();

    for (extracted, recognized, passes) in [
        (
            "\x0c",
            "Collection 1842 4137 5840 6971 7950 8651 6013",
            true,
        ),
        (
            "\x0c",
            "Collection 1843 4138 5841 6972 7951 8652 6014",
            false,
        ),
        ("\x0c", "Unrelated report from another archive", false),
        ("\x0c", "", false),
        (
            "\x0c\x0c",
            "Collection 1842 4137 5840 6971 7950 8651 6013",
            false,
        ),
        (
            "unrelated selectable text\x0c",
            "Collection 1842 4137 5840 6971 7950 8651 6013",
            false,
        ),
    ] {
        fixture.pdf_text(extracted, "");
        let commands = fixture.base.join("external-commands");
        fs::write(
            commands.join("pdftoppm"),
            "#!/bin/sh\nprintf '%s' \"$7\" > \"${10}.png\"\n",
        )
        .unwrap();
        fs::write(
            commands.join("tesseract"),
            format!(
                "#!/bin/sh\nif [ \"$1\" = --list-langs ]; then echo eng; elif [ \"$(cat \"$1\")\" = 200 ]; then printf '%s' '{recognized}'; else echo 'Faded fragment along the lower edge'; fi\n"
            ),
        )
        .unwrap();
        let issues = fixture.file_issues(file);
        assert_eq!(!issues.contains_key("source"), passes, "{issues:?}");
    }
}
