# p06 M1 frozen assignments

Status: **FROZEN before the first scope worker starts**. This file is token-free. Do not rename, add, remove, or move any listed final artifact.

## Corpus contract

- Persona: `p06` / Life-science researcher / `life-science-researcher`
- Fictional organization: Orchid Cellworks
- Environment: Windows laboratory workstation, `en-US`, English
- Reference date: `2026-07-13`
- Classification: wholly synthetic S2/S3 laboratory data; no real patient, participant, sample, PHI, PII, credential, endpoint, or research data
- Final root: `workspace/people/p06-life-science-researcher/home`
- Temporary/QA root: `scratch/p06/<scope-id>/`
- Ownership: one Subagent owns exactly one Rust scope ID and its one plan-defined final leaf
- Final count: `20 scopes / 200 files`
- Family ledger: `md=12`, `txt_log=12`, `code=6`, `structured_text=10`, `csv_tsv=30`, `html_eml=4`, `ipynb=6`, `pdf_text=36`, `pdf_scan=16`, `docx=16`, `xlsx=16`, `pptx=10`, `image=18`, `media=0`, `domain_binary=8`

## Shared synthetic factual spine

- Study Alpha: synthetic `cohort A-24`, 24 coded samples `OCW-A24-001` through `OCW-A24-024`; protocol `OCW-BIO-017`; run `ALPHA-A24-R03`.
- Study Alpha reference result: median assay concentration `6.8 ng/mL`; acceptable duplicate CV `<=10.0%`; recovery window `90-110%`.
- Study Beta: synthetic `cohort B-16`, 16 coded samples `OCW-B16-001` through `OCW-B16-016`; run `BETA-B16-R02`; normalized response baseline `1.00`, treated `1.18`.
- Fictional instruments: `SpectraNova SN-4` plate reader, `PipetSure P200`, `ThermoNest IN-2`; calibration lot `CAL-260713-A`.
- Calibration controls: slope `0.998`, `R² 0.9992`, blank `0.02 AU`; review timestamp `2026-07-13 17:30 UTC`.
- All identifiers, measurements, plots, packet traces, literature summaries, and grants are original synthetic artifacts for Orchid Cellworks.

## Seed contract

These exact 12 files are seeds. Every seed must visibly/searchably contain all five exact anchors: `Orchid Cellworks`, `Study Alpha`, `2026-07-13`, `cohort A-24`, `assay 6.8 ng/mL`.

1. `lab/lab-notebooks/2026-07-13-study-alpha-cohort-a24-lab-notebook.md`
2. `lab/protocols/study-alpha-assay-quantification-protocol.docx`
3. `programs/study-alpha/2026/cohort-a/raw-exports/study-alpha-cohort-a24-assay-raw-export.csv`
4. `programs/study-alpha/2026/cohort-a/analysis/study-alpha-cohort-a24-assay-analysis.ipynb`
5. `programs/study-beta/2026/cohort-b/raw-exports/study-beta-cohort-b16-instrument-export.tsv`
6. `programs/study-beta/2026/cohort-b/analysis/study-beta-cohort-b16-assay-analysis.xlsx`
7. `instruments/calibration/2026-07-13-plate-reader-calibration-scan.pdf`
8. `samples/manifests/study-alpha-cohort-a24-sample-manifest.json`
9. `literature/papers/study-alpha-a24-assay-method-review.pdf`
10. `grants/applications/study-alpha-a24-translational-research-grant.docx`
11. `figures/manuscript/study-alpha-a24-assay-response-figure.png`
12. `meetings/lab/2026-07-13-study-alpha-lab-review.pptx`

## Common quality gates

- Final leaves contain only the exact listed artifacts. Builders, renders, previews, QA logs, caches, sidecars, `__pycache__`, and `.pyc` remain under the scope scratch leaf.
- DOCX uses Documents with a resolved preset/header pattern, substantive content, all-page render, and individual visual inspection.
- Searchable PDF uses the PDF skill, has a non-empty text layer, and every page is rendered and inspected.
- Scan PDF uses built-in ImageGen source artwork plus the PDF skill, is image-only with no extractable text layer, and every page is rendered and inspected.
- XLSX uses Spreadsheets and `@oai/artifact-tool`, separates raw/analysis/results where applicable, keeps typed units and formula-derived results, has zero formula errors, and renders every sheet for inspection.
- PPTX uses Presentations and `@oai/artifact-tool`, follows Codex Grid unless a scope-specific visual direction is stated, contains at least five slides, renders every slide, and passes `slides_test.py` with overflow zero.
- Every final PNG and every scan source uses a distinct built-in ImageGen call. Final PNGs are true PNG and inspected at original detail.
- IPYNB validates as nbformat, contains markdown/code/saved synthetic output, has zero error outputs, is executed twice offline when feasible, and is rendered to HTML for inspection.
- CSV/TSV are UTF-8, rectangular, have useful headers and multiple data rows. JSON/JSONL/YAML/XML/SQL/HTML/EML and code use an appropriate parser/compiler.
- PCAP files have valid libpcap containers, meaningful synthetic packets, valid checksums, documentation-only IPs, and no real payloads.
- Exact SHA-256 duplicates are prohibited.

## Scope assignments

### p06-primary-01 — 20 files

Final folder: `workspace/people/p06-life-science-researcher/home/lab/lab-notebooks`

- `2026-07-13-study-alpha-cohort-a24-lab-notebook.md` — family=md; ext=.md; seed
- `study-alpha-a24-observation-notes.md` — family=md; ext=.md
- `study-beta-b16-run-comparison.md` — family=md; ext=.md
- `alpha-a24-bench-handoff.txt` — family=txt_log; ext=.txt
- `2026-07-13-incubator-monitor.log` — family=txt_log; ext=.log
- `lab-notebook-change-register.jsonl` — family=txt_log; ext=.jsonl
- `a24-lab-notebook-summary.ipynb` — family=ipynb; ext=.ipynb
- `alpha-a24-daily-bench-summary.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-assay-run-03-readout.pdf` — family=pdf_text; ext=.pdf
- `beta-b16-observation-summary.pdf` — family=pdf_text; ext=.pdf
- `cross-study-qc-note.pdf` — family=pdf_text; ext=.pdf
- `2026-07-13-lab-notebook-index.pdf` — family=pdf_text; ext=.pdf
- `bench-notebook-page-scan.pdf` — family=pdf_scan; ext=.pdf
- `alpha-a24-experiment-narrative.docx` — family=docx; ext=.docx
- `beta-b16-experiment-narrative.docx` — family=docx; ext=.docx
- `alpha-a24-notebook-measurements.xlsx` — family=xlsx; ext=.xlsx
- `lab-notebook-qc-tracker.xlsx` — family=xlsx; ext=.xlsx
- `alpha-a24-bench-layout.png` — family=image; ext=.png
- `a24-observation-timepoints.csv` — family=csv_tsv; ext=.csv
- `lab-notebook-entry-schema.yaml` — family=structured_text; ext=.yaml

### p06-primary-02 — 18 files

Final folder: `workspace/people/p06-life-science-researcher/home/lab/protocols`

- `study-alpha-assay-quantification-protocol.docx` — family=docx; ext=.docx; seed
- `assay-sample-preparation-sop.docx` — family=docx; ext=.docx
- `instrument-run-release-checklist.docx` — family=docx; ext=.docx
- `study-alpha-protocol-overview.pdf` — family=pdf_text; ext=.pdf
- `assay-acceptance-criteria.pdf` — family=pdf_text; ext=.pdf
- `protocol-deviation-handling.pdf` — family=pdf_text; ext=.pdf
- `synthetic-biosafety-work-instruction.pdf` — family=pdf_text; ext=.pdf
- `signed-protocol-training-sheet-scan.pdf` — family=pdf_scan; ext=.pdf
- `pipetting-checklist-scan.pdf` — family=pdf_scan; ext=.pdf
- `protocol-version-history.md` — family=md; ext=.md
- `protocol-quick-reference.md` — family=md; ext=.md
- `protocol-execution-log.txt` — family=txt_log; ext=.txt
- `protocol-ocw-bio-017.xml` — family=structured_text; ext=.xml
- `protocol-parameter-contract.json` — family=structured_text; ext=.json
- `assay-reagent-preparation-calculator.xlsx` — family=xlsx; ext=.xlsx
- `assay-workflow-reference.png` — family=image; ext=.png
- `validate-protocol-parameters.py` — family=code; ext=.py
- `protocol-training-brief.pptx` — family=pptx; ext=.pptx

### p06-primary-03 — 16 files

Final folder: `workspace/people/p06-life-science-researcher/home/programs/study-alpha/2026/cohort-a/raw-exports`

- `study-alpha-cohort-a24-assay-raw-export.csv` — family=csv_tsv; ext=.csv; seed
- `alpha-a24-plate-layout.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-assay-controls.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-replicate-readings.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-sample-volume-log.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-temperature-trace.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-qc-flags.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-import-schema.sql` — family=structured_text; ext=.sql
- `alpha-a24-export-metadata.json` — family=structured_text; ext=.json
- `alpha-a24-instrument-export.log` — family=txt_log; ext=.log
- `alpha-a24-export-readme.txt` — family=txt_log; ext=.txt
- `alpha-a24-raw-export-review.xlsx` — family=xlsx; ext=.xlsx
- `alpha-a24-export-data-dictionary.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-raw-export-qc-report.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-instrument-transfer.pcap` — family=domain_binary; ext=.pcap
- `alpha-a24-export-integrity-check.pcap` — family=domain_binary; ext=.pcap

### p06-primary-04 — 16 files

Final folder: `workspace/people/p06-life-science-researcher/home/programs/study-alpha/2026/cohort-a/analysis`

- `study-alpha-cohort-a24-assay-analysis.ipynb` — family=ipynb; ext=.ipynb; seed
- `alpha-a24-dose-response-analysis.ipynb` — family=ipynb; ext=.ipynb
- `alpha-a24-qc-diagnostics.ipynb` — family=ipynb; ext=.ipynb
- `alpha-a24-analysis-pipeline.py` — family=code; ext=.py
- `alpha-a24-statistics.rs` — family=code; ext=.rs
- `alpha-a24-clean-analysis-table.csv` — family=csv_tsv; ext=.csv
- `alpha-a24-model-diagnostics.tsv` — family=csv_tsv; ext=.tsv
- `alpha-a24-analysis-workbook.xlsx` — family=xlsx; ext=.xlsx
- `alpha-a24-qc-analysis.xlsx` — family=xlsx; ext=.xlsx
- `alpha-a24-analysis-report.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-dose-response-methods.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-sensitivity-analysis.pdf` — family=pdf_text; ext=.pdf
- `alpha-a24-analysis-plan.docx` — family=docx; ext=.docx
- `alpha-a24-dose-response-plot.png` — family=image; ext=.png
- `alpha-a24-analysis-readme.md` — family=md; ext=.md
- `alpha-a24-analysis-readout.pptx` — family=pptx; ext=.pptx

### p06-primary-05 — 14 files

Final folder: `workspace/people/p06-life-science-researcher/home/programs/study-beta/2026/cohort-b/raw-exports`

- `study-beta-cohort-b16-instrument-export.tsv` — family=csv_tsv; ext=.tsv; seed
- `beta-b16-plate-layout.tsv` — family=csv_tsv; ext=.tsv
- `beta-b16-assay-raw-export.tsv` — family=csv_tsv; ext=.tsv
- `beta-b16-control-readings.csv` — family=csv_tsv; ext=.csv
- `beta-b16-replicate-readings.csv` — family=csv_tsv; ext=.csv
- `beta-b16-incubation-log.csv` — family=csv_tsv; ext=.csv
- `beta-b16-qc-flags.csv` — family=csv_tsv; ext=.csv
- `beta-b16-export-index.csv` — family=csv_tsv; ext=.csv
- `beta-b16-instrument-export.log` — family=txt_log; ext=.log
- `beta-b16-export-readme.txt` — family=txt_log; ext=.txt
- `beta-b16-raw-export-review.xlsx` — family=xlsx; ext=.xlsx
- `beta-b16-export-data-dictionary.pdf` — family=pdf_text; ext=.pdf
- `beta-b16-raw-export-qc-report.pdf` — family=pdf_text; ext=.pdf
- `beta-b16-instrument-transfer.pcap` — family=domain_binary; ext=.pcap

### p06-primary-06 — 12 files

Final folder: `workspace/people/p06-life-science-researcher/home/programs/study-beta/2026/cohort-b/analysis`

- `study-beta-cohort-b16-assay-analysis.xlsx` — family=xlsx; ext=.xlsx; seed
- `beta-b16-analysis-workbook.xlsx` — family=xlsx; ext=.xlsx
- `beta-b16-response-analysis.ipynb` — family=ipynb; ext=.ipynb
- `beta-b16-qc-diagnostics.ipynb` — family=ipynb; ext=.ipynb
- `validate-beta-analysis.ts` — family=code; ext=.ts
- `beta-b16-clean-analysis-table.csv` — family=csv_tsv; ext=.csv
- `beta-b16-model-diagnostics.csv` — family=csv_tsv; ext=.csv
- `beta-b16-analysis-report.pdf` — family=pdf_text; ext=.pdf
- `alpha-beta-comparator-analysis.pdf` — family=pdf_text; ext=.pdf
- `beta-b16-analysis-plan.docx` — family=docx; ext=.docx
- `beta-b16-response-plot.png` — family=image; ext=.png
- `beta-b16-analysis-readout.pptx` — family=pptx; ext=.pptx

### p06-primary-07 — 12 files

Final folder: `workspace/people/p06-life-science-researcher/home/instruments/calibration`

- `2026-07-13-plate-reader-calibration-scan.pdf` — family=pdf_scan; ext=.pdf; seed
- `spectranova-wavelength-check-scan.pdf` — family=pdf_scan; ext=.pdf
- `spectranova-linearity-check-scan.pdf` — family=pdf_scan; ext=.pdf
- `pipette-p200-calibration-scan.pdf` — family=pdf_scan; ext=.pdf
- `incubator-temperature-map-scan.pdf` — family=pdf_scan; ext=.pdf
- `balance-daily-check-scan.pdf` — family=pdf_scan; ext=.pdf
- `instrument-calibration-summary.pdf` — family=pdf_text; ext=.pdf
- `instrument-maintenance-history.pdf` — family=pdf_text; ext=.pdf
- `spectranova-calibration-standards.csv` — family=csv_tsv; ext=.csv
- `instrument-calibration-results.tsv` — family=csv_tsv; ext=.tsv
- `instrument-calibration-control-chart.xlsx` — family=xlsx; ext=.xlsx
- `spectranova-calibration-export.pcap` — family=domain_binary; ext=.pcap

### p06-primary-08 — 10 files

Final folder: `workspace/people/p06-life-science-researcher/home/samples/manifests`

- `study-alpha-cohort-a24-sample-manifest.json` — family=structured_text; ext=.json; seed
- `sample-chain-of-custody.yaml` — family=structured_text; ext=.yaml
- `sample-container-map.xml` — family=structured_text; ext=.xml
- `sample-manifest-schema.json` — family=structured_text; ext=.json
- `study-alpha-a24-sample-manifest.csv` — family=csv_tsv; ext=.csv
- `study-beta-b16-sample-manifest.tsv` — family=csv_tsv; ext=.tsv
- `sample-manifest-audit-report.pdf` — family=pdf_text; ext=.pdf
- `sample-inventory-and-alias-map.xlsx` — family=xlsx; ext=.xlsx
- `sample-storage-location-map.png` — family=image; ext=.png
- `sample-manifest-governance.md` — family=md; ext=.md

### p06-primary-09 — 10 files

Final folder: `workspace/people/p06-life-science-researcher/home/literature/papers`

- `study-alpha-a24-assay-method-review.pdf` — family=pdf_text; ext=.pdf; seed
- `synthetic-assay-validation-review.pdf` — family=pdf_text; ext=.pdf
- `plate-reader-linearity-methods.pdf` — family=pdf_text; ext=.pdf
- `cohort-design-comparison.pdf` — family=pdf_text; ext=.pdf
- `reproducibility-practices-review.pdf` — family=pdf_text; ext=.pdf
- `annotated-methods-paper-scan.pdf` — family=pdf_scan; ext=.pdf
- `literature-screening-notes.md` — family=md; ext=.md
- `study-alpha-evidence-map.md` — family=md; ext=.md
- `literature-review-synthesis.docx` — family=docx; ext=.docx
- `literature-evidence-map.png` — family=image; ext=.png

### p06-primary-10 — 8 files

Final folder: `workspace/people/p06-life-science-researcher/home/grants/applications`

- `study-alpha-a24-translational-research-grant.docx` — family=docx; ext=.docx; seed
- `study-alpha-specific-aims.docx` — family=docx; ext=.docx
- `study-alpha-research-strategy.docx` — family=docx; ext=.docx
- `grant-application-internal-review.pdf` — family=pdf_text; ext=.pdf
- `grant-milestone-and-budget-model.xlsx` — family=xlsx; ext=.xlsx
- `grant-submission-checklist.md` — family=md; ext=.md
- `grant-workplan-figure.png` — family=image; ext=.png
- `grant-review-actions.jsonl` — family=txt_log; ext=.jsonl

### p06-primary-11 — 8 files

Final folder: `workspace/people/p06-life-science-researcher/home/figures/manuscript`

- `study-alpha-a24-assay-response-figure.png` — family=image; ext=.png; seed
- `alpha-a24-replicate-distribution.png` — family=image; ext=.png
- `alpha-a24-qc-control-chart.png` — family=image; ext=.png
- `alpha-beta-assay-comparison.png` — family=image; ext=.png
- `assay-workflow-manuscript-panel.png` — family=image; ext=.png
- `study-alpha-cohort-schematic.png` — family=image; ext=.png
- `manuscript-figure-captions.pdf` — family=pdf_text; ext=.pdf
- `manuscript-figure-review.pptx` — family=pptx; ext=.pptx

### p06-primary-12 — 6 files

Final folder: `workspace/people/p06-life-science-researcher/home/meetings/lab`

- `2026-07-13-study-alpha-lab-review.pptx` — family=pptx; ext=.pptx; seed
- `study-alpha-methods-workshop.pptx` — family=pptx; ext=.pptx
- `alpha-beta-program-review.pptx` — family=pptx; ext=.pptx
- `2026-07-13-lab-meeting-minutes.docx` — family=docx; ext=.docx
- `2026-07-13-lab-meeting-readout.pdf` — family=pdf_text; ext=.pdf
- `lab-meeting-action-register.md` — family=md; ext=.md

### p06-secondary-01 — 10 files

Final folder: `workspace/people/p06-life-science-researcher/home/desktop/working`

- `working-alpha-a24-calculation-sheet.xlsx` — family=xlsx; ext=.xlsx
- `working-beta-b16-qc-sheet.xlsx` — family=xlsx; ext=.xlsx
- `working-alpha-a24-filtered.csv` — family=csv_tsv; ext=.csv
- `working-beta-b16-filtered.tsv` — family=csv_tsv; ext=.tsv
- `working-analysis-notes.md` — family=md; ext=.md
- `desktop-assay-checklist.txt` — family=txt_log; ext=.txt
- `working-session.txt` — family=txt_log; ext=.txt
- `assay-unit-check.rs` — family=code; ext=.rs
- `working-plate-map.png` — family=image; ext=.png
- `handwritten-working-notes-scan.pdf` — family=pdf_scan; ext=.pdf

### p06-secondary-02 — 8 files

Final folder: `workspace/people/p06-life-science-researcher/home/documents/reference`

- `assay-reference-handbook.pdf` — family=pdf_text; ext=.pdf
- `synthetic-cohort-coding-guide.pdf` — family=pdf_text; ext=.pdf
- `archived-calibration-certificate-scan.pdf` — family=pdf_scan; ext=.pdf
- `reagent-receipt-scan.pdf` — family=pdf_scan; ext=.pdf
- `laboratory-reference-guide.docx` — family=docx; ext=.docx
- `synthetic-data-handling-guide.docx` — family=docx; ext=.docx
- `assay-unit-conversion-reference.png` — family=image; ext=.png
- `reference-query-patterns.sql` — family=structured_text; ext=.sql

### p06-secondary-03 — 8 files

Final folder: `workspace/people/p06-life-science-researcher/home/downloads/inbox`

- `incoming-alpha-a24-rerun.csv` — family=csv_tsv; ext=.csv
- `incoming-beta-b16-export.tsv` — family=csv_tsv; ext=.tsv
- `incoming-instrument-release-note.pdf` — family=pdf_text; ext=.pdf
- `incoming-data-transfer-summary.pdf` — family=pdf_text; ext=.pdf
- `incoming-reagent-certificate-scan.pdf` — family=pdf_scan; ext=.pdf
- `incoming-plate-layout-preview.png` — family=image; ext=.png
- `download-inbox-index.html` — family=html_eml; ext=.html
- `download-transfer-diagnostic.pcap` — family=domain_binary; ext=.pcap

### p06-secondary-04 — 6 files

Final folder: `workspace/people/p06-life-science-researcher/home/downloads/exports`

- `study-alpha-curated-export.csv` — family=csv_tsv; ext=.csv
- `study-alpha-export-bundle.xlsx` — family=xlsx; ext=.xlsx
- `study-alpha-export-summary.pdf` — family=pdf_text; ext=.pdf
- `study-alpha-export-handoff.pptx` — family=pptx; ext=.pptx
- `study-beta-export-handoff.pptx` — family=pptx; ext=.pptx
- `dashboard-export-diagnostic.pcap` — family=domain_binary; ext=.pcap

### p06-secondary-05 — 6 files

Final folder: `workspace/people/p06-life-science-researcher/home/cloud/my-files`

- `personal-study-alpha-summary.pdf` — family=pdf_text; ext=.pdf
- `personal-bench-note-scan.pdf` — family=pdf_scan; ext=.pdf
- `personal-research-plan.docx` — family=docx; ext=.docx
- `personal-assay-checks.xlsx` — family=xlsx; ext=.xlsx
- `personal-study-map.png` — family=image; ext=.png
- `synthetic-export-validator.ts` — family=code; ext=.ts

### p06-secondary-06 — 4 files

Final folder: `workspace/people/p06-life-science-researcher/home/cloud/team-shared`

- `team-assay-review-register.tsv` — family=csv_tsv; ext=.tsv
- `team-shared-study-overview.pdf` — family=pdf_text; ext=.pdf
- `team-study-status-review.pptx` — family=pptx; ext=.pptx
- `team-handoff-notes.docx` — family=docx; ext=.docx

### p06-secondary-07 — 4 files

Final folder: `workspace/people/p06-life-science-researcher/home/mail/recent`

- `study-alpha-review-invitation.eml` — family=html_eml; ext=.eml
- `instrument-export-complete.eml` — family=html_eml; ext=.eml
- `lab-mail-index.html` — family=html_eml; ext=.html
- `recent-mail-thread.txt` — family=txt_log; ext=.txt

### p06-secondary-08 — 4 files

Final folder: `workspace/people/p06-life-science-researcher/home/archive/closed`

- `closed-study-alpha-bench-sheet-scan.pdf` — family=pdf_scan; ext=.pdf
- `closed-study-alpha-archive-map.png` — family=image; ext=.png
- `closed-alpha-export-transfer.pcap` — family=domain_binary; ext=.pcap
- `closed-beta-export-transfer.pcap` — family=domain_binary; ext=.pcap

## Parent acceptance checklist

- Exact 20 scope headings, exact per-scope counts, exact 200 assigned paths, and exact requested family ledger.
- Exactly 12 seed paths; each seed contains all 5 exact anchors.
- Every rich artifact completes its skill-specific render/inspection gate.
- Every parser/compiler/container check passes and final extensions are authentic.
- Final root has 200 files, 200 unique SHA-256 values, zero duplicate groups, and zero unassigned artifacts.
- p01-p05 fixed manifests and all 1,000 referenced files remain byte-for-byte unchanged.
- Canonical, owner, config, pinned runtime, Git branch/HEAD/status/worktree remain unchanged.
- Every scope lease releases normally before the parent lease; final `active_leases=0`.
