use lopdf::{Document, Object, Stream, dictionary};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
fn run(command: &str, args: &[&str]) -> Output {
    let out = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .arg(command)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{command}: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    out
}
fn string(path: &Path) -> &str {
    path.to_str().unwrap()
}
fn pdf(path: &Path) {
    let mut d = Document::with_version("1.5");
    let pages = d.new_object_id();
    let f = d.add_object(dictionary! {"Type"=>"Font","Subtype"=>"Type1","BaseFont"=>"Helvetica"});
    let contents = d.add_object(Stream::new(
        dictionary! {},
        b"BT /F1 24 Tf 50 600 Td (MONTHLY REPORT 2026 sales 12345) Tj ET".to_vec(),
    ));
    let page=d.add_object(dictionary!{"Type"=>"Page","Parent"=>pages,"MediaBox"=>vec![0.into(),0.into(),600.into(),800.into()],"Resources"=>dictionary!{"Font"=>dictionary!{"F1"=>f}},"Contents"=>contents});
    d.objects.insert(
        pages,
        Object::Dictionary(
            dictionary! {"Type"=>"Pages","Kids"=>vec![Object::Reference(page)],"Count"=>1},
        ),
    );
    let root = d.add_object(dictionary! {"Type"=>"Catalog","Pages"=>pages});
    let info=d.add_object(dictionary!{"Author"=>Object::string_literal("/home/producer"),"Producer"=>Object::string_literal("ReportLab"),"CreationDate"=>Object::string_literal("D:20261118000000Z")});
    d.trailer.set("Root", root);
    d.trailer.set("Info", info);
    d.save(path).unwrap();
}
#[test]
fn pdf_scan_render_and_stamp() {
    let work = tempfile::tempdir().unwrap();
    let source = work.path().join("monthly.pdf");
    pdf(&source);
    let scanned = work.path().join("scan.pdf");
    let other = work.path().join("other.pdf");
    for output in [&scanned, &other] {
        run(
            "scan",
            &[string(&source), string(output), "--seed", "42", "--gray"],
        );
    }
    assert_eq!(fs::read(&scanned).unwrap(), fs::read(&other).unwrap());
    let d = Document::load(&scanned).unwrap();
    assert!(d.trailer.get(b"Info").is_err());
    assert_eq!(d.extract_text(&[1]).unwrap().trim(), "");
    let image = d.get_page_images(*d.get_pages().get(&1).unwrap()).unwrap();
    assert_eq!(image.len(), 1);
    let rendered = work.path().join("render");
    run(
        "render",
        &[
            string(&scanned),
            string(&rendered),
            "--max-pages",
            "1",
            "--dpi",
            "72",
        ],
    );
    assert!(rendered.join("scan-1.png").exists());
    let ocr = Command::new("tesseract")
        .arg(rendered.join("scan-1.png"))
        .args(["stdout", "-l", "eng"])
        .output()
        .unwrap();
    assert!(ocr.status.success());
    assert!(String::from_utf8_lossy(&ocr.stdout).contains("12345"));
    let before = Document::load(&source).unwrap().extract_text(&[1]).unwrap();
    run(
        "stamp",
        &[
            string(&source),
            "--author",
            "山田 花子",
            "--date",
            "2026-03-06T18:42:10+09:00",
        ],
    );
    let stamped = Document::load(&source).unwrap();
    assert_eq!(stamped.extract_text(&[1]).unwrap(), before);
    let info = stamped
        .get_dictionary(
            stamped
                .trailer
                .get(b"Info")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        info.get(b"CreationDate").unwrap().as_str().unwrap(),
        b"D:20260306094210Z"
    );
    let bytes = fs::read(&source).unwrap();
    let invalid = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args([
            "stamp",
            string(&source),
            "--author",
            "A",
            "--date",
            "2026-03-06",
            "--created",
            "2027-01-01",
        ])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
    assert_eq!(fs::read(source).unwrap(), bytes);
}
#[test]
fn png_screenshot_records_author_and_document_times() {
    let work = tempfile::tempdir().unwrap();
    let path = work.path().join("screen.png");
    image::RgbImage::new(12, 12).save(&path).unwrap();
    run(
        "stamp",
        &[
            string(&path),
            "--author",
            "田中 美由",
            "--created",
            "2026-03-05T10:00:00+09:00",
            "--date",
            "2026-03-06T18:42:10+09:00",
        ],
    );
    fixture(
        work.path(),
        r#"from pathlib import Path
from PIL import Image
import sys
info = Image.open(Path(sys.argv[1]) / 'screen.png').info
assert info['Author'] == '田中 美由'
assert info['Creation Time'] == '2026-03-05T10:00:00+09:00'
assert info['Modification Time'] == '2026-03-06T18:42:10+09:00'
"#,
    );
}
#[test]
fn image_pixels_and_capture_dates() {
    let work = tempfile::tempdir().unwrap();
    for extension in ["png", "jpg"] {
        let source = work.path().join(format!("photo.{extension}"));
        image::RgbImage::from_pixel(20, 20, image::Rgb([40, 80, 120]))
            .save(&source)
            .unwrap();
        let before = image::open(&source).unwrap().to_rgb8();
        run(
            "stamp",
            &[
                string(&source),
                "--author",
                "山田",
                "--date",
                "2026-03-06T18:42:10+09:00",
                "--device",
                "Canon EOS R6",
            ],
        );
        assert_eq!(image::open(&source).unwrap().to_rgb8(), before);
        if extension == "jpg" {
            let exif = exif::Reader::new()
                .read_from_container(&mut std::io::BufReader::new(
                    fs::File::open(&source).unwrap(),
                ))
                .unwrap();
            assert!(
                exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
                    .is_some()
            );
            assert!(
                exif.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
                    .is_none()
            );
        }
        let out = work.path().join("render");
        run("render", &[string(&source), string(&out)]);
        assert_eq!(
            fs::read(out.join(source.file_name().unwrap())).unwrap(),
            fs::read(&source).unwrap()
        );
    }
}
#[test]
fn office_recalc_and_conversion() {
    let work = tempfile::tempdir().unwrap();
    let out=Command::new("uv").args(["run","python","-c",r#"import sys
from pathlib import Path
from docx import Document
from openpyxl import Workbook
from pptx import Presentation
p=Path(sys.argv[1]);d=Document();d.add_paragraph('Monthly sales 12345');d.save(p/'report.docx')
b=Workbook();b.active['A1']=7;b.active['A2']='=A1*3';b.save(p/'report.xlsx')
s=Presentation();sl=s.slides.add_slide(s.slide_layouts[0]);sl.shapes.title.text='Monthly sales 12345';s.save(p/'report.pptx')
"#,string(work.path())]).current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()).env("UV_CACHE_DIR","/tmp/personacorpus-uv-cache").output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let book = work.path().join("report.xlsx");
    run("recalc", &[string(&book)]);
    let mut archive = zip::ZipArchive::new(fs::File::open(&book).unwrap()).unwrap();
    let mut sheet = String::new();
    use std::io::Read;
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut sheet)
        .unwrap();
    assert!(sheet.contains("21"));
    drop(archive);
    for extension in ["docx", "xlsx", "pptx"] {
        let path = work.path().join(format!("report.{extension}"));
        let original = fs::read(&path).unwrap();
        run(
            "stamp",
            &[
                string(&path),
                "--author",
                "山田",
                "--date",
                "2026-03-06T18:42:10+09:00",
            ],
        );
        let first = fs::read(&path).unwrap();
        fs::write(&path, &original).unwrap();
        run(
            "stamp",
            &[
                string(&path),
                "--author",
                "山田",
                "--date",
                "2026-03-06T18:42:10+09:00",
            ],
        );
        assert_eq!(fs::read(&path).unwrap(), first);
        // Plan dates are written to the minute ("2026-03-06 18:42"), as in runbooks/PRODUCTION.md.
        fs::write(&path, &original).unwrap();
        run(
            "stamp",
            &[
                string(&path),
                "--author",
                "山田",
                "--date",
                "2026-03-06 18:42",
                "--timezone",
                "Asia/Tokyo",
            ],
        );
        fs::write(&path, &original).unwrap();
        run(
            "stamp",
            &[
                string(&path),
                "--author",
                "山田",
                "--date",
                "2026-03-06T18:42:10+09:00",
            ],
        );
        let output = work.path().join(format!("{extension}.pdf"));
        run("office-pdf", &[string(&path), string(&output)]);
        let doc = Document::load(output).unwrap();
        assert!(doc.trailer.get(b"Info").is_err());
        assert_eq!(doc.get_pages().len(), 1);
    } // Version numbers in file names ("説明_v0.2.docx") must not lose the conversion output.
    for extension in ["docx", "xlsx", "pptx"] {
        let dotted = work.path().join(format!("説明_v0.2.{extension}"));
        fs::copy(work.path().join(format!("report.{extension}")), &dotted).unwrap();
        let output = work.path().join(format!("dotted-{extension}.pdf"));
        run("office-pdf", &[string(&dotted), string(&output)]);
        assert_eq!(Document::load(output).unwrap().get_pages().len(), 1);
        let pages = work.path().join(format!("pages-{extension}"));
        run("render", &[string(&dotted), string(&pages)]);
        assert!(pages.join("説明_v0.2-1.png").is_file());
    }
}
#[test]
fn tex_local_inputs_and_failure() {
    let work = tempfile::tempdir().unwrap();
    fs::write(
        work.path().join("body.tex"),
        r"Monthly report\label{monthly}\newpage Page \pageref{monthly}",
    )
    .unwrap();
    fs::write(
        work.path().join("localreport.cls"),
        r"\ProvidesClass{localreport}\LoadClass{article}",
    )
    .unwrap();
    let source = work.path().join("report.tex");
    fs::write(
        &source,
        r"\documentclass{localreport}\begin{document}\input{body}\end{document}",
    )
    .unwrap();
    let output = work.path().join("report.pdf");
    run("tex", &[string(&source), string(&output)]);
    let d = Document::load(output).unwrap();
    assert_eq!(d.get_pages().len(), 2);
    assert!(pdf_text(&work.path().join("report.pdf")).contains("Page 1"));
    assert!(d.trailer.get(b"Info").is_err());
    let artifacts = fs::read_dir(work.path())
        .unwrap()
        .filter_map(|p| p.ok())
        .map(|p| p.path())
        .filter(|p| {
            p.file_stem().is_some_and(|s| s == "report")
                && p.extension().is_some_and(|s| s != "pdf")
        })
        .collect::<Vec<_>>();
    assert_eq!(artifacts, vec![source.clone()]);
    fs::write(
        &source,
        r"\documentclass{article}\begin{document}\undefinedcommand\end{document}",
    )
    .unwrap();
    let bad = work.path().join("bad.pdf");
    let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args(["tex", string(&source), string(&bad)])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Undefined control sequence"));
    assert!(!bad.exists());
}

fn pdf_text(path: &Path) -> String {
    let out = Command::new("pdftotext")
        .args([string(path), "-"])
        .output()
        .unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}
fn fixture(work: &Path, script: &str) {
    let out = Command::new("uv")
        .args(["run", "python", "-c", script, work.to_str().unwrap()])
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .env("UV_CACHE_DIR", "/tmp/personacorpus-uv-cache")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "fixture: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
fn zip_parts(path: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    use std::io::Read;
    let mut archive = zip::ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let mut parts = std::collections::BTreeMap::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        parts.insert(entry.name().to_owned(), data);
    }
    parts
}
fn property(path: &Path, name: &str) -> String {
    let parts = zip_parts(path);
    let raw = std::str::from_utf8(&parts["docProps/core.xml"]).unwrap();
    let doc = roxmltree::Document::parse(raw).unwrap();
    doc.descendants()
        .find(|n| n.is_element() && n.tag_name().name() == name)
        .unwrap()
        .text()
        .unwrap_or_default()
        .to_owned()
}
fn stamped(path: &Path, extra: &[&str]) {
    let mut args = vec![
        string(path),
        "--author",
        "山田 花子",
        "--date",
        "2026-03-06T18:42:10+09:00",
    ];
    args.extend(extra);
    run("stamp", &args);
}
fn deterministic_stamp(path: &Path, extra: &[&str]) {
    let original = fs::read(path).unwrap();
    stamped(path, extra);
    let first = fs::read(path).unwrap();
    fs::write(path, original).unwrap();
    stamped(path, extra);
    assert_eq!(fs::read(path).unwrap(), first);
    assert!(!checker_findings(path).iter().any(|r| r[2] == "metadata"));
}
fn checker_findings(path: &Path) -> Vec<Vec<String>> {
    let work = tempfile::tempdir().unwrap();
    let root = work.path();
    let persona = "p01-test";
    let people = root.join("people").join(persona);
    let definitions = root.join("personas");
    fs::create_dir_all(&people).unwrap();
    fs::create_dir(&definitions).unwrap();
    fs::write(
        definitions.join(format!("{persona}.md")),
        "---\nperiod: 2026-01-01..2026-12-31\ntimezone: Asia/Tokyo\n---\n",
    )
    .unwrap();
    let file = people.join(format!(
        "report.{}",
        path.extension().unwrap().to_string_lossy()
    ));
    fs::copy(path, &file).unwrap();
    let stamp = chrono::DateTime::parse_from_rfc3339("2026-03-06T18:42:10+09:00")
        .unwrap()
        .timestamp();
    filetime::set_file_mtime(file, filetime::FileTime::from_unix_time(stamp, 0)).unwrap();
    let report = root.join("report.tsv");
    let out = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args([
            "check",
            persona,
            "--root",
            string(&root.join("people")),
            "--personas",
            string(&definitions),
            "--sources",
            string(&root.join("sources")),
            "--progress",
            string(&root.join("progress")),
            "--manifests",
            string(&root.join("manifests")),
            "--report",
            string(&report),
        ])
        .output()
        .unwrap();
    assert!(
        [Some(0), Some(1)].contains(&out.status.code()),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(report)
        .unwrap()
        .records()
        .map(|r| r.unwrap().iter().map(str::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert!(!rows.iter().any(|r| r[2] == "opens"), "{rows:?}");
    rows
}
fn exif_text(exif: &exif::Exif, tag: exif::Tag) -> Option<String> {
    exif.get_field(tag, exif::In::PRIMARY).and_then(|field| {
        if let exif::Value::Ascii(values) = &field.value {
            values
                .first()
                .map(|v| String::from_utf8_lossy(v).into_owned())
        } else {
            None
        }
    })
}
fn read_exif(path: &Path) -> exif::Exif {
    exif::Reader::new()
        .read_from_container(&mut std::io::BufReader::new(fs::File::open(path).unwrap()))
        .unwrap()
}
fn assert_clean_pdf(path: &Path) -> Document {
    let d = Document::load(path).unwrap();
    assert!(d.trailer.get(b"Info").is_err());
    assert!(d.catalog().unwrap().get(b"Metadata").is_err());
    d
}
fn stamp_office_fixture(work: &Path) {
    fixture(
        work,
        r#"import sys
from pathlib import Path
from docx import Document
from openpyxl import Workbook
from pptx import Presentation
p=Path(sys.argv[1]);d=Document();d.add_paragraph('Monthly sales 12345');d.core_properties.comments='generated using python-docx';d.save(p/'report.docx')
b=Workbook();b.active['A1']='Monthly sales 12345';b.save(p/'report.xlsx')
s=Presentation();s.core_properties.author='python-pptx';sl=s.slides.add_slide(s.slide_layouts[0]);sl.shapes.title.text='Monthly sales 12345';s.save(p/'report.pptx')
"#,
    );
}
#[test]
fn stamp_office_properties_and_content() {
    let work = tempfile::tempdir().unwrap();
    stamp_office_fixture(work.path());
    for (extension, application, body) in [
        ("docx", "Microsoft Office Word", "word/document.xml"),
        ("xlsx", "Microsoft Excel", "xl/worksheets/sheet1.xml"),
        (
            "pptx",
            "Microsoft Office PowerPoint",
            "ppt/slides/slide1.xml",
        ),
    ] {
        let path = work.path().join(format!("report.{extension}"));
        let before = zip_parts(&path)[body].clone();
        assert!(checker_findings(&path).iter().any(|r| r[2] == "metadata"));
        deterministic_stamp(&path, &[]);
        assert_eq!(property(&path, "creator"), "山田 花子");
        assert_eq!(property(&path, "lastModifiedBy"), "山田 花子");
        assert_eq!(property(&path, "created"), "2026-03-06T09:42:10Z");
        assert_eq!(property(&path, "modified"), property(&path, "created"));
        let parts = zip_parts(&path);
        assert!(!String::from_utf8_lossy(&parts["docProps/core.xml"]).contains("python"));
        assert!(String::from_utf8_lossy(&parts["docProps/app.xml"]).contains(application));
        let text = |raw: &[u8]| {
            let s = std::str::from_utf8(raw).unwrap();
            roxmltree::Document::parse(s)
                .unwrap()
                .descendants()
                .filter(|n| n.is_text())
                .filter_map(|n| n.text())
                .collect::<String>()
        };
        assert_eq!(text(&before), text(&parts[body]));
        let mut archive = zip::ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
        for i in 0..archive.len() {
            let date = archive.by_index(i).unwrap().last_modified().unwrap();
            assert_eq!(
                (
                    date.year(),
                    date.month(),
                    date.day(),
                    date.hour(),
                    date.minute(),
                    date.second()
                ),
                (2026, 3, 6, 18, 42, 10)
            );
        }
    }
}
#[test]
fn stamp_separate_creator_dates_and_timezone() {
    let work = tempfile::tempdir().unwrap();
    stamp_office_fixture(work.path());
    for (extension, timezone, expected) in [
        ("docx", None, "2026-02-01T12:00:00Z"),
        ("xlsx", Some("Asia/Tokyo"), "2026-02-01T03:00:00Z"),
        ("pptx", Some("Asia/Tokyo"), "2026-02-01T03:00:00Z"),
    ] {
        let path = work.path().join(format!("report.{extension}"));
        let mut flags = vec!["--modified-by", "千田", "--created", "2026-02-01"];
        if let Some(timezone) = timezone {
            flags.extend(["--timezone", timezone]);
        }
        stamped(&path, &flags);
        assert_eq!(property(&path, "creator"), "山田 花子");
        assert_eq!(property(&path, "lastModifiedBy"), "千田");
        assert_eq!(property(&path, "created"), expected);
        assert_eq!(property(&path, "modified"), "2026-03-06T09:42:10Z");
    }
    let path = work.path().join("revision.pdf");
    pdf(&path);
    stamped(&path, &["--modified-by", "千田", "--created", "2026-02-01"]);
    let d = Document::load(&path).unwrap();
    let info = d
        .get_dictionary(d.trailer.get(b"Info").unwrap().as_reference().unwrap())
        .unwrap();
    assert_eq!(
        info.get(b"CreationDate").unwrap().as_str().unwrap(),
        b"D:20260201120000Z"
    );
    assert_eq!(
        info.get(b"ModDate").unwrap().as_str().unwrap(),
        b"D:20260306094210Z"
    );
    let xmp = d
        .get_object(
            d.catalog()
                .unwrap()
                .get(b"Metadata")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap()
        .as_stream()
        .unwrap();
    assert!(String::from_utf8_lossy(&xmp.content).contains("2026-02-01T12:00:00Z"));
}
#[test]
fn stamp_pdf_properties_xmp_and_content() {
    let work = tempfile::tempdir().unwrap();
    let path = work.path().join("report.pdf");
    pdf(&path);
    let before = Document::load(&path).unwrap().extract_text(&[1]).unwrap();
    assert!(checker_findings(&path).iter().any(|r| r[2] == "metadata"));
    deterministic_stamp(&path, &["--producer", "Microsoft® Word for Microsoft 365"]);
    let d = Document::load(&path).unwrap();
    assert_eq!(d.extract_text(&[1]).unwrap(), before);
    let info = d
        .get_dictionary(d.trailer.get(b"Info").unwrap().as_reference().unwrap())
        .unwrap();
    assert_eq!(
        info.get(b"Producer").unwrap().as_str().unwrap(),
        [
            vec![254, 255],
            "Microsoft® Word for Microsoft 365"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect()
        ]
        .concat()
    );
    assert_eq!(
        info.get(b"Author").unwrap().as_str().unwrap(),
        [
            vec![254, 255],
            "山田 花子"
                .encode_utf16()
                .flat_map(u16::to_be_bytes)
                .collect()
        ]
        .concat()
    );
    let xmp = d
        .get_object(
            d.catalog()
                .unwrap()
                .get(b"Metadata")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap()
        .as_stream()
        .unwrap();
    let xmp = String::from_utf8_lossy(&xmp.content);
    assert!(xmp.contains("山田 花子"));
    for name in ["CreateDate", "ModifyDate", "MetadataDate"] {
        let doc = roxmltree::Document::parse(&xmp).unwrap();
        assert_eq!(
            doc.descendants()
                .find(|n| n.is_element() && n.tag_name().name() == name)
                .unwrap()
                .text(),
            Some("2026-03-06T09:42:10Z")
        );
    }
}
#[test]
fn stamp_image_pixels_orientation_and_capture_replacement() {
    let work = tempfile::tempdir().unwrap();
    fixture(
        work.path(),
        r#"import sys
from pathlib import Path
from PIL import Image,PngImagePlugin
p=Path(sys.argv[1])
for ext in ('png','jpg'):
 im=Image.new('RGB',(12,12),'red');exif=Image.Exif();exif[305]='Pillow';exif[306]='2026:11:16 12:00:00';exif[274]=6
 if ext=='png':
  info=PngImagePlugin.PngInfo();info.add_text('Software','matplotlib');im.save(p/('photo.'+ext),pnginfo=info,exif=exif)
 else: im.save(p/('photo.'+ext),exif=exif)
for ext in ('jpg','jpeg'): Image.new('RGB',(12,12),'blue').save(p/('capture.'+ext))
exif=Image.Exif();exif[274]=6;exif[271]='Old make';exif[272]='Old model';exif[305]='Pillow';exif[306]='2026:11:16 12:00:00';exif[34665]={36867:'2026:11:16 12:00:00',36868:'2026:11:16 12:00:00',37510:b'Codex',36881:'+00:00'};exif[34853]={1:'N',2:(35.,0.,0.)}
Image.new('RGB',(12,12),'red').save(p/'old-capture.jpg',exif=exif)
Image.new('RGB',(12,12),'blue').save(p/'plain.png')
"#,
    );
    for extension in ["png", "jpg"] {
        let path = work.path().join(format!("photo.{extension}"));
        let before = image::open(&path).unwrap().to_rgb8();
        assert!(checker_findings(&path).iter().any(|r| r[2] == "metadata"));
        deterministic_stamp(&path, &[]);
        assert_eq!(image::open(&path).unwrap().to_rgb8(), before);
        let exif = read_exif(&path);
        assert_eq!(
            exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                .unwrap()
                .value
                .get_uint(0),
            Some(6)
        );
        assert!(
            exif.get_field(exif::Tag::Software, exif::In::PRIMARY)
                .is_none()
        );
    }
    for extension in ["jpg", "jpeg"] {
        let path = work.path().join(format!("capture.{extension}"));
        deterministic_stamp(&path, &[]);
        let exif = read_exif(&path);
        for tag in [
            exif::Tag::DateTime,
            exif::Tag::DateTimeOriginal,
            exif::Tag::DateTimeDigitized,
        ] {
            assert_eq!(
                exif_text(&exif, tag).as_deref(),
                Some("2026:03:06 18:42:10")
            );
        }
        for tag in [exif::Tag::Make, exif::Tag::Model, exif::Tag::GPSLatitude] {
            assert!(exif.get_field(tag, exif::In::PRIMARY).is_none());
        }
    }
    let path = work.path().join("old-capture.jpg");
    let original = fs::read(&path).unwrap();
    for device in [None, Some("Canon EOS R6")] {
        fs::write(&path, &original).unwrap();
        deterministic_stamp(
            &path,
            &device.map(|v| vec!["--device", v]).unwrap_or_default(),
        );
        let exif = read_exif(&path);
        for tag in [
            exif::Tag::DateTime,
            exif::Tag::DateTimeOriginal,
            exif::Tag::DateTimeDigitized,
        ] {
            assert_eq!(
                exif_text(&exif, tag).as_deref(),
                Some("2026:03:06 18:42:10")
            );
        }
        assert_eq!(
            exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                .unwrap()
                .value
                .get_uint(0),
            Some(6)
        );
        for tag in [
            exif::Tag::Software,
            exif::Tag::GPSLatitude,
            exif::Tag::OffsetTimeOriginal,
            exif::Tag::UserComment,
        ] {
            assert!(exif.get_field(tag, exif::In::PRIMARY).is_none());
        }
        assert_eq!(
            exif.fields()
                .filter(|field| field.tag.context() == exif::Context::Exif)
                .count(),
            2
        );
        assert!(
            exif.get_field(exif::Tag::GPSInfoIFDPointer, exif::In::PRIMARY)
                .is_none()
        );
        for tag in [exif::Tag::Make, exif::Tag::Model] {
            assert_eq!(exif_text(&exif, tag).as_deref(), device);
        }
    }
    let path = work.path().join("plain.png");
    deterministic_stamp(&path, &["--device", "Canon EOS R6"]);
    assert!(
        exif::Reader::new()
            .read_from_container(&mut std::io::BufReader::new(fs::File::open(path).unwrap()))
            .is_err()
    );
}
#[test]
fn stamp_office_revision_dates_comments_and_nonmetadata() {
    let work = tempfile::tempdir().unwrap();
    fixture(
        work.path(),
        r#"import sys,zipfile
from pathlib import Path
from docx import Document
from openpyxl import Workbook
from openpyxl.comments import Comment
from pptx import Presentation
p=Path(sys.argv[1]);d=Document();d.add_paragraph('Monthly report');d.save(p/'history.docx')
with zipfile.ZipFile(p/'history.docx') as z: entries={i.filename:z.read(i) for i in z.infolist()}
entries['word/comments.xml']=b'<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:comment w:id="0" w:author="Reviewer" w:date="2026-01-01T08:00:00Z"/><w:comment w:id="1" w:author="Editor" w:date="2026-11-16T08:00:00Z"/></w:comments>'
entries['word/document.xml']=entries['word/document.xml'].replace(b'<w:body>',b'<w:body><w:ins w:id="1" w:author="Editor" w:date="2026-11-16T08:00:00Z"/>')
with zipfile.ZipFile(p/'history.docx','w') as z:
 for name,data in entries.items():z.writestr(name,data)
b=Workbook();b.active['A1']='Monthly report';b.active['A1'].comment=Comment('Review this number','openpyxl');b.save(p/'comments.xlsx')
s=Presentation();sl=s.slides.add_slide(s.slide_layouts[0]);sl.shapes.title.text='Monthly report';sl.shapes.title.name='Python library discussion';s.save(p/'shapes.pptx')
"#,
    );
    let path = work.path().join("history.docx");
    run(
        "stamp",
        &[
            string(&path),
            "--author",
            "山田 花子",
            "--date",
            "2026-03-06 18:42:10",
            "--timezone",
            "Asia/Tokyo",
        ],
    );
    let parts = zip_parts(&path);
    let comments = String::from_utf8_lossy(&parts["word/comments.xml"]);
    assert!(
        comments.contains("2026-01-01T08:00:00Z")
            && comments.contains("2026-03-06T09:42:10Z")
            && comments.contains("Reviewer")
    );
    assert!(!String::from_utf8_lossy(&parts["word/document.xml"]).contains("2026-11-16"));
    assert!(!checker_findings(&path).iter().any(|r| r[2] == "metadata"));
    let path = work.path().join("comments.xlsx");
    deterministic_stamp(&path, &[]);
    let parts = zip_parts(&path);
    let comment = parts
        .iter()
        .find(|(name, _)| name.contains("comments") && name.ends_with(".xml"))
        .unwrap();
    assert!(String::from_utf8_lossy(comment.1).contains("山田 花子"));
    assert!(!String::from_utf8_lossy(comment.1).contains("openpyxl"));
    let path = work.path().join("shapes.pptx");
    deterministic_stamp(&path, &[]);
    assert!(
        String::from_utf8_lossy(&zip_parts(&path)["ppt/slides/slide1.xml"])
            .contains("Python library discussion")
    );
}
#[test]
fn stamp_preserves_xmp_context() {
    let work = tempfile::tempdir().unwrap();
    let path = work.path().join("metadata.pdf");
    pdf(&path);
    let mut d = Document::load(&path).unwrap();
    let metadata=d.add_object(Stream::new(dictionary!{"Type"=>"Metadata","Subtype"=>"XML"},br#"<x:xmpmeta xmlns:x="adobe:ns:meta/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:xmp="http://ns.adobe.com/xap/1.0/"><rdf:RDF><rdf:Description dc:title="Monthly report" xmp:CreatorTool="ReportLab"><dc:description>Important context</dc:description></rdf:Description></rdf:RDF></x:xmpmeta>"#.to_vec()));
    d.catalog_mut().unwrap().set("Metadata", metadata);
    d.save(&path).unwrap();
    deterministic_stamp(&path, &[]);
    let d = Document::load(&path).unwrap();
    let xmp = d
        .get_object(
            d.catalog()
                .unwrap()
                .get(b"Metadata")
                .unwrap()
                .as_reference()
                .unwrap(),
        )
        .unwrap()
        .as_stream()
        .unwrap();
    let text = String::from_utf8_lossy(&xmp.content);
    assert!(text.contains("Monthly report") && text.contains("Important context"));
    assert!(!text.contains("ReportLab"));
}
#[test]
fn stamp_zip_and_unchanged_text() {
    let work = tempfile::tempdir().unwrap();
    let path = work.path().join("bundle.zip");
    {
        use std::io::Write;
        let mut zip = zip::ZipWriter::new(fs::File::create(&path).unwrap());
        zip.start_file("memo.txt", zip::write::SimpleFileOptions::default())
            .unwrap();
        zip.write_all(b"Monthly sales 12345").unwrap();
        zip.finish().unwrap();
    }
    deterministic_stamp(&path, &[]);
    let mut zip = zip::ZipArchive::new(fs::File::open(path).unwrap()).unwrap();
    let date = zip.by_name("memo.txt").unwrap().last_modified().unwrap();
    assert_eq!(
        (
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
            date.second()
        ),
        (2026, 3, 6, 18, 42, 10)
    );
    let path = work.path().join("memo.txt");
    fs::write(&path, "Monthly sales 12345").unwrap();
    deterministic_stamp(&path, &[]);
    assert_eq!(fs::read_to_string(path).unwrap(), "Monthly sales 12345");
}

#[test]
fn stamp_real_pdf_converters_and_repair() {
    let work = tempfile::tempdir().unwrap();
    stamp_office_fixture(work.path());
    let tex = work.path().join("document.tex");
    fs::write(
        &tex,
        r"\documentclass{article}\begin{document}Monthly sales 12345\end{document}",
    )
    .unwrap();
    let image = work.path().join("document.png");
    image::RgbImage::from_pixel(100, 100, image::Rgb([255, 255, 255]))
        .save(&image)
        .unwrap();
    for (tool, input) in [
        ("office-pdf", work.path().join("report.docx")),
        ("tex", tex),
        ("scan", image),
    ] {
        let path = work.path().join(format!("{tool}.pdf"));
        run(tool, &[string(&input), string(&path)]);
        let before = pdf_text(&path);
        stamped(&path, &[]);
        assert!(!checker_findings(&path).iter().any(|r| r[2] == "metadata"));
        let mut doc = Document::load(&path).unwrap();
        assert_eq!(pdf_text(&path), before);
        let dirty=doc.add_object(dictionary!{"Author"=>Object::string_literal("python"),"Producer"=>Object::string_literal("reportlab"),"CreationDate"=>Object::string_literal("D:20261116080000Z")});
        doc.trailer.set("Info", dirty);
        doc.save(&path).unwrap();
        assert!(checker_findings(&path).iter().any(|r| r[2] == "metadata"));
        deterministic_stamp(&path, &[]);
        let d = Document::load(&path).unwrap();
        assert_eq!(pdf_text(&path), before);
        if tool == "tex" {
            let info = d
                .get_dictionary(d.trailer.get(b"Info").unwrap().as_reference().unwrap())
                .unwrap();
            assert_eq!(
                info.get(b"Producer").unwrap().as_str().unwrap(),
                b"Adobe PDF Library 23.0"
            );
        }
        if tool == "scan" {
            let info = d
                .get_dictionary(d.trailer.get(b"Info").unwrap().as_reference().unwrap())
                .unwrap();
            assert_eq!(
                info.get(b"Producer").unwrap().as_str().unwrap(),
                b"Canon iR-ADV C3830"
            );
        }
    }
}
fn image_fixture(work: &Path) {
    fixture(
        work,
        r#"import sys
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
p=Path(sys.argv[1]);im=Image.new('RGB',(1240,1754),'white');draw=ImageDraw.Draw(im);font=ImageFont.truetype('DejaVuSans.ttf',48);draw.text((110,180),'MONTHLY REPORT 2026',font=font,fill='black');draw.text((110,270),'Total sales 12345',font=font,fill='black');im.save(p/'original page.png')
"#,
    );
}
fn scan_check(input: &Path, output: &Path, extra: &[&str]) {
    let mut args = vec![string(input), string(output), "--seed", "42"];
    args.extend(extra);
    run("scan", &args);
    let d = assert_clean_pdf(output);
    let bytes = fs::read(output).unwrap();
    assert!(!bytes.windows(14).any(|b| b == b"/ASCII85Decode"));
    for (_, id) in d.get_pages() {
        assert_eq!(d.extract_text(&[1]).unwrap().trim(), "");
        let images = d.get_page_images(id).unwrap();
        assert!(!images.is_empty());
        let resources = d.get_page_resources(id).unwrap();
        let direct = resources.0.unwrap();
        let objects = direct.get(b"XObject").unwrap().as_dict().unwrap();
        for (_, id) in objects.iter() {
            let image = d
                .get_object(id.as_reference().unwrap())
                .unwrap()
                .as_stream()
                .unwrap();
            assert_eq!(
                image.dict.get(b"Filter").unwrap().as_name().unwrap(),
                b"DCTDecode"
            );
        }
    }
    assert!(
        !checker_findings(output)
            .iter()
            .any(|row| row[2] == "secret")
    );
}
#[test]
fn scan_image_determinism_gray_changed_seed_and_readability() {
    let work = tempfile::tempdir().unwrap();
    image_fixture(work.path());
    let input = work.path().join("original page.png");
    let first = work.path().join("scan.pdf");
    let other = work.path().join("other.pdf");
    scan_check(&input, &first, &[]);
    scan_check(&input, &other, &[]);
    assert_eq!(fs::read(&first).unwrap(), fs::read(&other).unwrap());
    assert_eq!(Document::load(&first).unwrap().get_pages().len(), 1);
    let gray = work.path().join("gray.pdf");
    scan_check(&input, &gray, &["--gray", "--quality", "70"]);
    let d = Document::load(&gray).unwrap();
    let resources = d
        .get_page_resources(*d.get_pages().get(&1).unwrap())
        .unwrap();
    let object = resources
        .0
        .unwrap()
        .get(b"XObject")
        .unwrap()
        .as_dict()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .1
        .as_reference()
        .unwrap();
    assert_eq!(
        d.get_object(object)
            .unwrap()
            .as_stream()
            .unwrap()
            .dict
            .get(b"ColorSpace")
            .unwrap()
            .as_name()
            .unwrap(),
        b"DeviceGray"
    );
    let changed = work.path().join("changed.pdf");
    run("scan", &[string(&input), string(&changed), "--seed", "43"]);
    assert_ne!(fs::read(&first).unwrap(), fs::read(&changed).unwrap());
    let prefix = work.path().join("ocr");
    assert!(
        Command::new("pdftoppm")
            .args([
                "-singlefile",
                "-r",
                "150",
                "-png",
                string(&first),
                string(&prefix)
            ])
            .status()
            .unwrap()
            .success()
    );
    let out = Command::new("tesseract")
        .args([string(&prefix.with_extension("png")), "stdout", "-l", "eng"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("MONTHLY REPORT 2026") && text.contains("12345"));
}
#[test]
fn scan_multiple_pdf_pages_and_rotation() {
    let work = tempfile::tempdir().unwrap();
    let source = work.path().join("two pages.pdf");
    pdf(&source);
    let mut d = Document::load(&source).unwrap();
    let first = *d.get_pages().get(&1).unwrap();
    let mut second = d.get_dictionary(first).unwrap().clone();
    let parent = second.get(b"Parent").unwrap().as_reference().unwrap();
    second.set("Rotate", 90);
    let second = d.add_object(second);
    d.get_dictionary_mut(parent).unwrap().set(
        "Kids",
        vec![Object::Reference(first), Object::Reference(second)],
    );
    d.get_dictionary_mut(parent).unwrap().set("Count", 2);
    d.save(&source).unwrap();
    let first = work.path().join("scan.pdf");
    let again = work.path().join("again.pdf");
    scan_check(&source, &first, &[]);
    scan_check(&source, &again, &[]);
    assert_eq!(fs::read(&first).unwrap(), fs::read(&again).unwrap());
    let d = Document::load(&first).unwrap();
    assert_eq!(d.get_pages().len(), 2);
    let page = d.get_dictionary(*d.get_pages().get(&2).unwrap()).unwrap();
    let rect = page.get(b"MediaBox").unwrap().as_array().unwrap();
    let number = |v: &Object| match v {
        Object::Integer(v) => *v as f32,
        Object::Real(v) => *v,
        _ => panic!("invalid number"),
    };
    assert_eq!((number(&rect[2]), number(&rect[3])), (800., 600.));
}
#[test]
fn render_page_names_dimensions_and_limits() {
    let work = tempfile::tempdir().unwrap();
    let source = work.path().join("two pages.pdf");
    fixture(
        work.path(),
        r#"import sys
from pathlib import Path
from reportlab.pdfgen.canvas import Canvas
p=Path(sys.argv[1]);c=Canvas(str(p/'two pages.pdf'),invariant=1)
for text in ('First monthly report','Second monthly report'):c.drawString(72,720,text);c.showPage()
c.save()
"#,
    );
    let output = work.path().join("renders");
    run("render", &[string(&source), string(&output), "--dpi", "72"]);
    let mut names = fs::read_dir(&output)
        .unwrap()
        .map(|p| p.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["two pages-1.png", "two pages-2.png"]);
    for name in names {
        assert_eq!(image::open(output.join(name)).unwrap().width(), 596);
    }
    let limited = work.path().join("limited");
    run(
        "render",
        &[string(&source), string(&limited), "--max-pages", "1"],
    );
    assert_eq!(fs::read_dir(limited).unwrap().count(), 1);
}
#[test]
fn media_rejects_bad_arguments_without_outputs() {
    let work = tempfile::tempdir().unwrap();
    let bad = work.path().join("bad.txt");
    fs::write(&bad, "not a supported document").unwrap();
    let missing = work.path().join("missing.pdf");
    let out = work.path().join("out");
    for tool in ["render", "tex", "office-pdf", "scan"] {
        for args in [
            vec![string(&bad), string(&out)],
            vec![string(&missing), string(&out)],
            vec![],
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
                .arg(tool)
                .args(args)
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
    let source = work.path().join("report.pdf");
    pdf(&source);
    for (option, value) in [("--dpi", "0"), ("--max-pages", "-1"), ("--max-pages", "0")] {
        let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
            .args(["render", string(&source), string(&out), option, value])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let image = work.path().join("photo.png");
    image::RgbImage::from_pixel(10, 10, image::Rgb([255, 255, 255]))
        .save(&image)
        .unwrap();
    for quality in ["0", "101"] {
        let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
            .args([
                "scan",
                string(&image),
                string(&out.with_extension("pdf")),
                "--quality",
                quality,
            ])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for (tool, input, output) in [
        ("scan", &image, &image),
        ("office-pdf", &source, &out.with_extension("pdf")),
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
            .args([tool, string(input), string(output)])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for (tool, extension) in [("office-pdf", "docx"), ("tex", "tex")] {
        let input = work.path().join(format!("valid.{extension}"));
        fs::write(&input, "converter fixture").unwrap();
        for path in [None, Some("")] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_corpus"));
            command.args([
                tool,
                string(&input),
                string(&out.with_extension(if path.is_some() { "pdf" } else { "txt" })),
            ]);
            if let Some(path) = path {
                command.env("PATH", path);
            }
            let result = command.output().unwrap();
            assert_eq!(
                result.status.code(),
                Some(2),
                "{tool}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
    let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args(["scan", string(&image), string(&out.with_extension("txt"))])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    let original = fs::read(&source).unwrap();
    let alias = work.path().join("alias.pdf");
    std::os::unix::fs::symlink(&source, &alias).unwrap();
    fs::create_dir(work.path().join("nested")).unwrap();
    for output in [
        alias,
        work.path().join("nested/../report.pdf"),
        work.path().join("./report.pdf"),
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
            .args(["scan", string(&source), string(&output)])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(fs::read(&source).unwrap(), original);
    }
    let alias = work.path().join("image.txt");
    std::os::unix::fs::symlink(&image, &alias).unwrap();
    let render = work.path().join("image-render");
    run("render", &[string(&alias), string(&render)]);
    assert_eq!(
        fs::read(render.join("photo.png")).unwrap(),
        fs::read(&image).unwrap()
    );
    let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args(["recalc", string(&bad)])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    let result = Command::new(env!("CARGO_BIN_EXE_corpus"))
        .args([
            "stamp",
            string(&bad),
            "--author",
            "Author",
            "--date",
            "invalid",
        ])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
}

#[test]
fn office_profiles_are_isolated_and_metadata_removed() {
    let work = tempfile::tempdir().unwrap();
    let private = work.path().join("metadata.pdf");
    pdf(&private);
    let mut source_pdf = Document::load(&private).unwrap();
    let metadata = source_pdf.add_object(Stream::new(
        dictionary! {"Type"=>"Metadata","Subtype"=>"XML"},
        b"<metadata>/home/private-user</metadata>".to_vec(),
    ));
    source_pdf.catalog_mut().unwrap().set("Metadata", metadata);
    source_pdf.save(&private).unwrap();
    let stubs = work.path().join("stubs");
    fs::create_dir(&stubs).unwrap();
    let trace = work.path().join("profiles.jsonl");
    let converter = stubs.join("soffice");
    fixture(
        work.path(),
        r#"import sys
from pathlib import Path
p=Path(sys.argv[1]);script='''#!/usr/bin/env python3
import json,pathlib,shutil,sys
args=sys.argv[1:]
profile=next(a for a in args if a.startswith('-env:UserInstallation='))
with open(%r,'a') as f:f.write(json.dumps(profile)+'\\n')
out=pathlib.Path(args[args.index('--outdir')+1])/(pathlib.Path(args[-1]).stem+'.pdf')
shutil.copyfile(%r,out)
'''%(str(p/'profiles.jsonl'),str(p/'metadata.pdf'))
(p/'stubs/soffice').write_text(script);(p/'stubs/soffice').chmod(0o755)
"#,
    );
    assert!(converter.is_file());
    let source = work.path().join("document.docx");
    fs::write(&source, b"converter is stubbed").unwrap();
    let temp_parent = work.path().join("temporary profiles 日本語");
    fs::create_dir(&temp_parent).unwrap();
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for n in 0..3 {
            let output = work.path().join(format!("clean-{n}.pdf"));
            let path = std::env::join_paths(
                std::iter::once(stubs.clone())
                    .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
            )
            .unwrap();
            let source = &source;
            let temp_parent = &temp_parent;
            handles.push(scope.spawn(move || {
                let out = Command::new(env!("CARGO_BIN_EXE_corpus"))
                    .args(["office-pdf", string(source), string(&output)])
                    .env("PATH", path)
                    .env("TMPDIR", temp_parent)
                    .output()
                    .unwrap();
                assert!(
                    out.status.success(),
                    "{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                assert_eq!(assert_clean_pdf(&output).get_pages().len(), 1);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
    let profiles = fs::read_to_string(trace)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<String>(line).unwrap())
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(profiles.len(), 3);
    assert!(profiles.iter().all(|p| p.contains("file://")
        && p.contains("temporary%20profiles%20%E6%97%A5%E6%9C%AC%E8%AA%9E")
        && !p.contains(' ')));
}
#[test]
fn parallel_recalc_stamp_render_and_office_conversion() {
    let work = tempfile::tempdir().unwrap();
    fixture(
        work.path(),
        r#"import sys
from pathlib import Path
from docx import Document
from openpyxl import Workbook
from pptx import Presentation
p=Path(sys.argv[1])
for number in range(3):
 d=p/str(number);d.mkdir();b=Workbook();b.active['A1']=number+10;b.active['A2']='=A1*3';b.save(d/'report.xlsx')
d=p/'office files';d.mkdir();doc=Document();doc.add_paragraph('First monthly report');doc.add_page_break();doc.add_paragraph('Second monthly report');doc.core_properties.author='/home/private-user';doc.save(d/'report.docx')
b=Workbook();b.active['A1']='Monthly sales 12345';b.save(d/'report.xlsx')
s=Presentation()
for text in ('First monthly report','Second monthly report'):sl=s.slides.add_slide(s.slide_layouts[0]);sl.shapes.title.text=text
s.save(d/'report.pptx')
"#,
    );
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for n in 0..3 {
            let directory = work.path().join(n.to_string());
            handles.push(scope.spawn(move || {
                let path = directory.join("report.xlsx");
                run("recalc", &[string(&path)]);
                let parts = zip_parts(&path);
                let doc = roxmltree::Document::parse(
                    std::str::from_utf8(&parts["xl/worksheets/sheet1.xml"]).unwrap(),
                )
                .unwrap();
                let cell = doc
                    .descendants()
                    .find(|node| {
                        node.is_element()
                            && node.tag_name().name() == "c"
                            && node.attribute("r") == Some("A2")
                    })
                    .unwrap();
                assert_eq!(
                    cell.children()
                        .find(|node| node.is_element() && node.tag_name().name() == "v")
                        .unwrap()
                        .text()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                    (n + 10) * 3
                );
                run(
                    "stamp",
                    &[
                        string(&path),
                        "--author",
                        "Eleanor",
                        "--date",
                        "2026-01-01T12:00:00Z",
                    ],
                );
                assert_eq!(property(&path, "creator"), "Eleanor");
                let render = directory.join("render");
                run("render", &[string(&path), string(&render)]);
                assert!(render.join("report-1.png").is_file());
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for (extension, pages, needle) in [
            ("docx", 2, "monthly"),
            ("xlsx", 1, "Monthly"),
            ("pptx", 2, "monthly"),
        ] {
            let source = work
                .path()
                .join("office files")
                .join(format!("report.{extension}"));
            let output = work.path().join(format!("{extension}.pdf"));
            let render = work.path().join(extension);
            handles.push(scope.spawn(move || {
                run("office-pdf", &[string(&source), string(&output)]);
                let d = assert_clean_pdf(&output);
                assert_eq!(d.get_pages().len(), pages);
                assert!(d.extract_text(&[1]).unwrap().contains(needle));
                run("render", &[string(&source), string(&render)]);
                assert_eq!(fs::read_dir(render).unwrap().count(), pages);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
}
#[test]
fn tex_japanese_and_fonts_available() {
    let fonts = Command::new("fc-list").arg(":lang=ja").output().unwrap();
    assert!(
        fonts.status.success() && !fonts.stdout.is_empty(),
        "Japanese fonts unavailable"
    );
    let work = tempfile::tempdir().unwrap();
    let source = work.path().join("japanese.tex");
    fs::write(&source,r"\documentclass{article}\usepackage{luatexja}\begin{document}日本語の月次報告書です。\end{document}").unwrap();
    let output = work.path().join("japanese.pdf");
    run("tex", &[string(&source), string(&output)]);
    assert_eq!(assert_clean_pdf(&output).get_pages().len(), 1);
    let text = Command::new("pdftotext")
        .args([string(&output), "-"])
        .output()
        .unwrap();
    assert!(text.status.success());
    assert!(
        String::from_utf8_lossy(&text.stdout)
            .replace(' ', "")
            .contains("月次報告書")
    );
}
