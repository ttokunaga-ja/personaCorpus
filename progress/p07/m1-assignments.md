# p07 M1 frozen assignments

Status: **FROZEN before the first scope worker starts**. This file is token-free. Do not rename, add, remove, or move any listed final artifact.

## Corpus contract

- Persona: `p07` / Humanities researcher / `humanities-researcher`
- Fictional project: **Linden Archive Project**
- Environment: macOS humanities-research laptop; `en-GB`; working languages `en`, `fr`, `de`, `ja`
- Reference date: `2026-07-13`
- Classification: wholly synthetic S0/S1 scholarship; no real unpublished archive, manuscript, private correspondence, researcher PII, credential, login, or copyrighted source text
- Final root: `workspace/people/p07-humanities-researcher/home`
- Temporary/QA root: `scratch/p07/<scope-id>/`
- Ownership: one Subagent owns exactly one Rust scope ID and its one plan-defined final leaf; no grandchildren
- Final count: `20 scopes / 200 files`
- Family ledger: `md=24`, `txt_log=20`, `code=0`, `structured_text=8`, `csv_tsv=6`, `html_eml=10`, `ipynb=0`, `pdf_text=50`, `pdf_scan=40`, `docx=20`, `xlsx=2`, `pptx=4`, `image=12`, `media=2`, `domain_binary=2`

## Shared synthetic content spine

- **Institution and collection:** the Haldenmere Civic Repository, its Linden Correspondence Series (`LCS`), all shelfmarks, access rules, staff, and descriptive records are fictional. The project snapshot is dated `2026-07-13`.
- **Principal fictional figures:** essayist Elinor Vale (1857-1919, English); printer-translator Lucien Moreau (1861-1922, French); curator Marta Weiss (1864-1930, German); critic Aiko Mori (1872-1941, Japanese). These people never existed.
- **Key source:** `LA-SRC-B14-F184`, Box 14, folio 184, a four-page letter dated 1907-03-16 from Vale to Moreau. Its working bibliography key is `citation LA-042`. The central wholly original line is: “The linden's shade keeps the town's many tongues in one patient circle.”
- **Related Box 14 sequence:** folios 181 (1907-02-21, Vale draft), 183 (1907-03-10, Moreau enclosure), 186 (1907-03-22, Weiss note), 188 (1907-04-02, Vale revision), 191 (1907-04-19, Moreau proof), 193 (1907-05-01, Mori gloss), and 196 (1907-05-18, Vale response). Source IDs use `LA-SRC-B14-F<folio>`.
- **Additional fictional series:** Box 6 folios 112/115 (1891 lecture notes); Box 9 folios 072/075 (1898 print proofs); Box 18 folios 203/207 (1910 translation circulars); Box 21 folios 044/049 (1912 memorial programme). Their source IDs follow the same pattern.
- **Interpretive throughline:** the dissertation argues that the recurring linden image is an editorial meeting device rather than a stable national symbol; evidence is compared across material marks, language shifts, circulation notes, and later reception.
- **Citation spine:** `LA-017` = Box 6/F112; `LA-028` = Box 9/F072; `LA-042` = Box 14/F184; `LA-057` = Box 18/F203; `LA-063` = Box 21/F044. Page/folio references and bibliographic records must agree with these keys.
- **OCR contract:** scans and transcripts share source ID, Box, folio, source date, page order, and line count. Unreadable text is `[illegible: n words]`; uncertain readings are `[word?]`; editorial supply is `[supplied: text]`; deletions are `<del>text</del>`. Never silently repair an uncertain reading.
- **Translation contract:** parallel translations align paragraph IDs (`P01`, `P02`, ...), preserve original text beside the translation, and record choices for “linden/tilleul/Linde/菩提樹”. No translation is presented as an authentic historical quotation.
- **Material details:** Box 14 uses cream laid paper, a fictional oval `HCR/LCS` accession stamp, iron-gall-style brown synthetic handwriting, blue editorial pencil, and a small leaf watermark. Other boxes use visibly distinct but coherent synthetic paper and annotation conventions.
- Every quotation, image, scan, OCR line, bibliography entry, address, email, packet payload, audio recording, and research conclusion is original synthetic content created for this corpus.

## Seed contract

The following exact 12 files are seeds. Every seed must visibly/searchably contain all five exact anchors: `Linden Archive Project`, `Box 14`, `2026-07-13`, `folio 184`, `citation LA-042`.

1. `research/primary-sources/box-14-folio-184-source-edition.pdf`
2. `research/archive-scans/box-14-folio-184-archive-scan.pdf`
3. `research/ocr-transcripts/box-14-folio-184-ocr-transcript.txt`
4. `research/bibliography-exports/linden-archive-bibliography-export.csv`
5. `notes/source-annotations/box-14-folio-184-source-annotations.md`
6. `notes/literature/citation-la-042-literature-note.md`
7. `dissertation/chapter-01/chapter-01-archive-context.docx`
8. `dissertation/chapter-02/chapter-02-folio-184-analysis.docx`
9. `dissertation/appendices/appendix-source-register.xlsx`
10. `translations/working/folio-184-parallel-translation.md`
11. `conferences/presentations/2026-07-13-linden-archive-conference.pptx`
12. `correspondence/archive-requests/2026-07-13-box-14-access-request.eml`

## Common quality gates

- Final leaves contain only the exact listed artifacts. Builders, sources, renders, previews, QA logs, caches, `__pycache__`, and `.pyc` remain in the scope scratch leaf.
- Searchable PDF uses PDF skill, has a non-empty text layer, citations/footnotes/page numbers where appropriate, and every page is rendered and individually inspected.
- Scan PDF uses one distinct built-in ImageGen source call per file plus PDF skill, is image-only with zero extractable text, and every page is rendered and individually inspected.
- DOCX uses Documents, resolves one design preset and one appropriate first-page pattern, and passes all-page render and individual visual inspection.
- XLSX uses Spreadsheets with bundled `@oai/artifact-tool`, typed values, auditable formulas, zero formula errors, and render/inspection of every sheet.
- PPTX uses Presentations with bundled `@oai/artifact-tool` and Codex Grid route, has at least five slides, renders every slide, passes `slides_test.py`, and has overflow zero.
- Each final PNG is created by a distinct built-in ImageGen call, is a true PNG, and is inspected at original detail.
- WAV is a real playable RIFF/WAVE synthetic recording with duration/sample-rate/channel validation. PCAP is valid libpcap with meaningful synthetic packets, valid checksums, and documentation-only IP addresses.
- Text/data formats are UTF-8 and pass their matching parser. Code and IPYNB are forbidden. Exact SHA-256 duplicates are forbidden.

## Scope assignments

### p07-primary-01 — 20 files

Final folder: `workspace/people/p07-humanities-researcher/home/research/primary-sources`

- `box-14-folio-184-source-edition.pdf` — family=pdf_text; ext=.pdf; seed
- `box-14-folio-181-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-183-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-186-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-188-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-191-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-193-source-edition.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-196-source-edition.pdf` — family=pdf_text; ext=.pdf
- `linden-series-editorial-introduction.pdf` — family=pdf_text; ext=.pdf
- `box-14-provenance-and-custody-note.pdf` — family=pdf_text; ext=.pdf
- `box-14-folio-184-diplomatic-transcription.docx` — family=docx; ext=.docx
- `linden-series-source-description.docx` — family=docx; ext=.docx
- `source-edition-conventions.md` — family=md; ext=.md
- `box-14-research-questions.md` — family=md; ext=.md
- `folio-184-collation-notes.txt` — family=txt_log; ext=.txt
- `primary-source-accession.log` — family=txt_log; ext=.log
- `box-14-source-register.json` — family=structured_text; ext=.json
- `primary-source-date-index.csv` — family=csv_tsv; ext=.csv
- `folio-184-seal-detail.png` — family=image; ext=.png
- `folio-184-pronunciation-note.wav` — family=media; ext=.wav

### p07-primary-02 — 18 files

Final folder: `workspace/people/p07-humanities-researcher/home/research/archive-scans`

- `box-14-folio-184-archive-scan.pdf` — family=pdf_scan; ext=.pdf; seed
- `box-14-folio-181-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-183-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-186-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-188-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-191-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-193-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-folio-196-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-09-folio-072-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-09-folio-075-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-21-folio-044-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-21-folio-049-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-06-folio-112-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-06-folio-115-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-18-folio-203-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-18-folio-207-archive-scan.pdf` — family=pdf_scan; ext=.pdf
- `box-14-container-label.png` — family=image; ext=.png
- `folio-184-watermark-reference.png` — family=image; ext=.png

### p07-primary-03 — 16 files

Final folder: `workspace/people/p07-humanities-researcher/home/research/ocr-transcripts`

- `box-14-folio-184-ocr-transcript.txt` — family=txt_log; ext=.txt; seed
- `box-14-folio-181-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-183-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-186-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-188-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-191-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-193-ocr-transcript.txt` — family=txt_log; ext=.txt
- `box-14-folio-196-ocr-transcript.txt` — family=txt_log; ext=.txt
- `ocr-editorial-conventions.md` — family=md; ext=.md
- `box-14-ocr-correction-journal.md` — family=md; ext=.md
- `box-14-ocr-zone-map.xml` — family=structured_text; ext=.xml
- `ocr-source-alignment.json` — family=structured_text; ext=.json
- `box-14-ocr-quality-report.pdf` — family=pdf_text; ext=.pdf
- `folio-184-handwriting-comparison.pdf` — family=pdf_text; ext=.pdf
- `box-14-ocr-editorial-review.docx` — family=docx; ext=.docx
- `box-14-ocr-parallel-view.html` — family=html_eml; ext=.html

### p07-primary-04 — 16 files

Final folder: `workspace/people/p07-humanities-researcher/home/research/bibliography-exports`

- `linden-archive-bibliography-export.csv` — family=csv_tsv; ext=.csv; seed
- `citation-authority-table.csv` — family=csv_tsv; ext=.csv
- `source-citation-crosswalk.tsv` — family=csv_tsv; ext=.tsv
- `multilingual-title-index.csv` — family=csv_tsv; ext=.csv
- `linden-archive-bibliography.json` — family=structured_text; ext=.json
- `citation-style-map.yaml` — family=structured_text; ext=.yaml
- `tei-bibliography.xml` — family=structured_text; ext=.xml
- `annotated-bibliography-browser.html` — family=html_eml; ext=.html
- `citation-la-042-entry.html` — family=html_eml; ext=.html
- `bibliography-method-note.pdf` — family=pdf_text; ext=.pdf
- `archival-citation-style-guide.pdf` — family=pdf_text; ext=.pdf
- `multilingual-bibliography-audit.pdf` — family=pdf_text; ext=.pdf
- `citation-la-042-source-chain.pdf` — family=pdf_text; ext=.pdf
- `linden-bibliography-control-register.xlsx` — family=xlsx; ext=.xlsx
- `bibliography-curation-notes.md` — family=md; ext=.md
- `bibliography-export-validation.log` — family=txt_log; ext=.log

### p07-primary-05 — 14 files

Final folder: `workspace/people/p07-humanities-researcher/home/notes/source-annotations`

- `box-14-folio-184-source-annotations.md` — family=md; ext=.md; seed
- `box-14-folio-181-source-annotations.md` — family=md; ext=.md
- `box-14-folio-183-source-annotations.md` — family=md; ext=.md
- `box-14-folio-186-source-annotations.md` — family=md; ext=.md
- `linden-motif-thread-annotations.md` — family=md; ext=.md
- `source-annotation-codebook.md` — family=md; ext=.md
- `folio-184-marginal-analysis.pdf` — family=pdf_text; ext=.pdf
- `epistolary-motif-map.pdf` — family=pdf_text; ext=.pdf
- `box-14-chronology-discrepancy-note.pdf` — family=pdf_text; ext=.pdf
- `source-annotation-synthesis.pdf` — family=pdf_text; ext=.pdf
- `source-annotation-review-dossier.docx` — family=docx; ext=.docx
- `box-14-interpretive-memo.docx` — family=docx; ext=.docx
- `box-14-folio-184-annotation-overlay.png` — family=image; ext=.png
- `annotation-session.log` — family=txt_log; ext=.log

### p07-primary-06 — 12 files

Final folder: `workspace/people/p07-humanities-researcher/home/notes/literature`

- `citation-la-042-literature-note.md` — family=md; ext=.md; seed
- `citation-la-017-literature-note.md` — family=md; ext=.md
- `citation-la-028-literature-note.md` — family=md; ext=.md
- `comparative-literature-synthesis.md` — family=md; ext=.md
- `linden-reception-history.pdf` — family=pdf_text; ext=.pdf
- `multilingual-memory-studies-review.pdf` — family=pdf_text; ext=.pdf
- `archival-silence-methods-review.pdf` — family=pdf_text; ext=.pdf
- `material-philology-review.pdf` — family=pdf_text; ext=.pdf
- `citation-network-synthesis.pdf` — family=pdf_text; ext=.pdf
- `literature-review-chapter-draft.docx` — family=docx; ext=.docx
- `historiography-working-paper.docx` — family=docx; ext=.docx
- `literature-note-index.html` — family=html_eml; ext=.html

### p07-primary-07 — 12 files

Final folder: `workspace/people/p07-humanities-researcher/home/dissertation/chapter-01`

- `chapter-01-archive-context.docx` — family=docx; ext=.docx; seed
- `chapter-01-research-question.docx` — family=docx; ext=.docx
- `chapter-01-methodology.docx` — family=docx; ext=.docx
- `chapter-01-section-draft.docx` — family=docx; ext=.docx
- `chapter-01-annotated-proof.pdf` — family=pdf_text; ext=.pdf
- `chapter-01-evidence-docket.pdf` — family=pdf_text; ext=.pdf
- `chapter-01-footnote-audit.pdf` — family=pdf_text; ext=.pdf
- `chapter-01-supervisor-review.pdf` — family=pdf_text; ext=.pdf
- `chapter-01-revision-plan.md` — family=md; ext=.md
- `chapter-01-citation-ledger.md` — family=md; ext=.md
- `chapter-01-change.log` — family=txt_log; ext=.log
- `chapter-01-source-route-diagram.png` — family=image; ext=.png

### p07-primary-08 — 10 files

Final folder: `workspace/people/p07-humanities-researcher/home/dissertation/chapter-02`

- `chapter-02-folio-184-analysis.docx` — family=docx; ext=.docx; seed
- `chapter-02-close-reading.docx` — family=docx; ext=.docx
- `chapter-02-translation-method.docx` — family=docx; ext=.docx
- `chapter-02-conclusion-draft.docx` — family=docx; ext=.docx
- `chapter-02-annotated-proof.pdf` — family=pdf_text; ext=.pdf
- `chapter-02-passage-comparison.pdf` — family=pdf_text; ext=.pdf
- `chapter-02-argument-map.pdf` — family=pdf_text; ext=.pdf
- `chapter-02-revision-notes.md` — family=md; ext=.md
- `chapter-02-supervisor-comments.txt` — family=txt_log; ext=.txt
- `folio-184-rhetorical-structure-map.png` — family=image; ext=.png

### p07-primary-09 — 10 files

Final folder: `workspace/people/p07-humanities-researcher/home/dissertation/appendices`

- `appendix-source-register.xlsx` — family=xlsx; ext=.xlsx; seed
- `appendix-palaeography-plate.pdf` — family=pdf_text; ext=.pdf
- `appendix-source-register-summary.pdf` — family=pdf_text; ext=.pdf
- `appendix-chronology.pdf` — family=pdf_text; ext=.pdf
- `appendix-editorial-conventions.docx` — family=docx; ext=.docx
- `appendix-multilingual-glossary.docx` — family=docx; ext=.docx
- `appendix-source-concordance.tsv` — family=csv_tsv; ext=.tsv
- `appendix-source-register-schema.json` — family=structured_text; ext=.json
- `appendix-assembly-notes.md` — family=md; ext=.md
- `box-14-folio-map.png` — family=image; ext=.png

### p07-primary-10 — 8 files

Final folder: `workspace/people/p07-humanities-researcher/home/translations/working`

- `folio-184-parallel-translation.md` — family=md; ext=.md; seed
- `folio-183-parallel-translation.md` — family=md; ext=.md
- `translation-decisions-journal.md` — family=md; ext=.md
- `french-correspondence-translation-packet.docx` — family=docx; ext=.docx
- `german-japanese-gloss-dossier.docx` — family=docx; ext=.docx
- `translation-alignment-proof.pdf` — family=pdf_text; ext=.pdf
- `multilingual-terminology-concordance.pdf` — family=pdf_text; ext=.pdf
- `translation-review.txt` — family=txt_log; ext=.txt

### p07-primary-11 — 8 files

Final folder: `workspace/people/p07-humanities-researcher/home/conferences/presentations`

- `2026-07-13-linden-archive-conference.pptx` — family=pptx; ext=.pptx; seed
- `linden-methods-colloquium.pptx` — family=pptx; ext=.pptx
- `multilingual-archives-seminar.pptx` — family=pptx; ext=.pptx
- `conference-source-montage.png` — family=image; ext=.png
- `folio-184-translation-pathways.png` — family=image; ext=.png
- `citation-network-poster.png` — family=image; ext=.png
- `linden-archive-conference-handout.pdf` — family=pdf_text; ext=.pdf
- `presentation-bibliography.pdf` — family=pdf_text; ext=.pdf

### p07-primary-12 — 6 files

Final folder: `workspace/people/p07-humanities-researcher/home/correspondence/archive-requests`

- `2026-07-13-box-14-access-request.eml` — family=html_eml; ext=.eml; seed
- `box-14-reproduction-request.eml` — family=html_eml; ext=.eml
- `folio-184-translation-query.eml` — family=html_eml; ext=.eml
- `conference-permission-request.eml` — family=html_eml; ext=.eml
- `archive-request-packet.docx` — family=docx; ext=.docx
- `archive-correspondence-register.pdf` — family=pdf_text; ext=.pdf

### p07-secondary-01 — 10 files

Final folder: `workspace/people/p07-humanities-researcher/home/desktop/working`

- `working-session-agenda.md` — family=md; ext=.md
- `daily-research-notes.txt` — family=txt_log; ext=.txt
- `folio-184-task-queue.log` — family=txt_log; ext=.log
- `working-note-bundle.pdf` — family=pdf_text; ext=.pdf
- `source-comparison-brief.pdf` — family=pdf_text; ext=.pdf
- `desk-note-scan.pdf` — family=pdf_scan; ext=.pdf
- `printed-chapter-markup-scan.pdf` — family=pdf_scan; ext=.pdf
- `microfilm-reader-slip-scan.pdf` — family=pdf_scan; ext=.pdf
- `dissertation-corkboard-map.png` — family=image; ext=.png
- `weekly-research-review.pptx` — family=pptx; ext=.pptx

### p07-secondary-02 — 8 files

Final folder: `workspace/people/p07-humanities-researcher/home/documents/reference`

- `reference-reading-list.md` — family=md; ext=.md
- `source-shelfmark-notes.txt` — family=txt_log; ext=.txt
- `humanities-data-citation-handbook.pdf` — family=pdf_text; ext=.pdf
- `french-palaeography-reference.pdf` — family=pdf_text; ext=.pdf
- `translation-standards-reference.pdf` — family=pdf_text; ext=.pdf
- `archive-handling-guide-scan.pdf` — family=pdf_scan; ext=.pdf
- `historical-calendar-reference-scan.pdf` — family=pdf_scan; ext=.pdf
- `reference-catalogue.yaml` — family=structured_text; ext=.yaml

### p07-secondary-03 — 8 files

Final folder: `workspace/people/p07-humanities-researcher/home/downloads/inbox`

- `download-triage-notes.txt` — family=txt_log; ext=.txt
- `download-provenance-report.pdf` — family=pdf_text; ext=.pdf
- `box-14-scan-intake-01.pdf` — family=pdf_scan; ext=.pdf
- `box-14-scan-intake-02.pdf` — family=pdf_scan; ext=.pdf
- `box-09-scan-intake.pdf` — family=pdf_scan; ext=.pdf
- `box-18-scan-intake.pdf` — family=pdf_scan; ext=.pdf
- `box-21-scan-intake.pdf` — family=pdf_scan; ext=.pdf
- `scan-batch-contact-sheet.png` — family=image; ext=.png

### p07-secondary-04 — 6 files

Final folder: `workspace/people/p07-humanities-researcher/home/downloads/exports`

- `export-transfer.log` — family=txt_log; ext=.log
- `digital-export-verification-report.pdf` — family=pdf_text; ext=.pdf
- `archive-export-receipt-scan.pdf` — family=pdf_scan; ext=.pdf
- `rights-declaration-scan.pdf` — family=pdf_scan; ext=.pdf
- `archive-export-transfer.pcap` — family=domain_binary; ext=.pcap
- `bibliography-sync-session.pcap` — family=domain_binary; ext=.pcap

### p07-secondary-05 — 6 files

Final folder: `workspace/people/p07-humanities-researcher/home/cloud/my-files`

- `personal-notebook-page-scan.pdf` — family=pdf_scan; ext=.pdf
- `folio-184-letter-printout-scan.pdf` — family=pdf_scan; ext=.pdf
- `translation-markup-scan.pdf` — family=pdf_scan; ext=.pdf
- `cloud-research-bundle-index.pdf` — family=pdf_text; ext=.pdf
- `personal-archive-dashboard.html` — family=html_eml; ext=.html
- `linden-pronunciation-comparison.wav` — family=media; ext=.wav

### p07-secondary-06 — 4 files

Final folder: `workspace/people/p07-humanities-researcher/home/cloud/team-shared`

- `team-meeting-whiteboard-scan.pdf` — family=pdf_scan; ext=.pdf
- `source-comparison-printout-scan.pdf` — family=pdf_scan; ext=.pdf
- `citation-audit-markup-scan.pdf` — family=pdf_scan; ext=.pdf
- `team-shared-research-readme.pdf` — family=pdf_text; ext=.pdf

### p07-secondary-07 — 4 files

Final folder: `workspace/people/p07-humanities-researcher/home/mail/recent`

- `archive-access-followup.eml` — family=html_eml; ext=.eml
- `recent-correspondence-digest.pdf` — family=pdf_text; ext=.pdf
- `attached-box-14-receipt-scan.pdf` — family=pdf_scan; ext=.pdf
- `meeting-note-attachment-scan.pdf` — family=pdf_scan; ext=.pdf

### p07-secondary-08 — 4 files

Final folder: `workspace/people/p07-humanities-researcher/home/archive/closed`

- `closed-box-06-lecture-note-scan.pdf` — family=pdf_scan; ext=.pdf
- `closed-box-09-print-proof-scan.pdf` — family=pdf_scan; ext=.pdf
- `closed-box-18-circular-scan.pdf` — family=pdf_scan; ext=.pdf
- `closed-box-21-programme-scan.pdf` — family=pdf_scan; ext=.pdf
