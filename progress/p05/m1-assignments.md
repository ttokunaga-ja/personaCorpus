# p05 M1 frozen assignments

Status: **FROZEN before the first scope worker starts**. This file is token-free. Do not rename, add, remove, or move any listed final artifact.

## Corpus contract

- Persona: `p05` / BI・データアナリスト / `bi-data-analyst`
- Fictional organization: Mosaic Retail Analytics
- Environment: Windows analysis laptop, `ja-JP`, Japanese/English working files
- Reference date: `2026-07-13`
- Data classification: `S2`, wholly synthetic OneDrive and warehouse exports; no real PII, sales, systems, credentials, or endpoints
- Final root: `workspace/people/p05-bi-data-analyst/home`
- Temporary and QA root: `scratch/p05/<scope-id>/` only
- Ownership: exactly one Subagent per leaf scope; no worker may write outside its final leaf and its own scratch leaf
- Final count: 20 scopes / 200 files
- Family ledger: `md=16`, `txt_log=10`, `code=12`, `structured_text=28`, `csv_tsv=40`, `html_eml=10`, `ipynb=10`, `pdf_text=10`, `pdf_scan=2`, `docx=6`, `xlsx=30`, `pptx=8`, `image=6`, `media=0`, `domain_binary=12`
- Extension contract: domain binaries are valid `.pcap`; the production SQL seed is `structured_text`, never `code`

## Shared factual spine

Use these values consistently where a file needs quantitative examples. Derived percentages may be rounded to two decimals.

- MTR-09 definition: completed order-line merchandise value, tax excluded, net of recognized refunds, reported in JPY
- Gross recognized merchandise value: `¥86.0M`; recognized refunds: `¥1.8M`; reported GMV: `¥84.2M`
- FY26 Q3 plan: `¥80.0M`; variance: `+¥4.2M` / `+5.25%`
- Prior comparable period: `¥78.6M`; variance: `+¥5.6M` / `+7.12%`
- Channel split: online `¥46.3M`; stores `¥37.9M`
- Category split: home `¥24.8M`; electronics `¥22.1M`; apparel `¥19.3M`; beauty `¥10.7M`; other `¥7.3M`
- Region split: east `¥26.4M`; west `¥22.8M`; central `¥18.6M`; north `¥16.4M`
- Daily review series for 2026-07-07 through 2026-07-13: `10.8, 11.2, 11.5, 12.1, 12.4, 13.0, 13.2` million JPY, totaling `84.2`
- Synthetic warehouse refresh: through `2026-07-13 23:59 JST`, quality review at `2026-07-14 06:00 JST`

## Seed contract

The following 12 exact files are seeds. Each must visibly/searchably contain all five exact anchors: `Mosaic Retail`, `FY26 Q3`, `2026-07-13`, `GMV ¥84.2M`, `metric MTR-09`.

1. `analytics/sql/production/fy26-q3-gmv-mtr09-production-query.sql`
2. `analytics/sql/ad-hoc/fy26-q3-gmv-variance-notes.md`
3. `analytics/data-dictionary/mtr-09-metric-data-dictionary.docx`
4. `analytics/lineage/mtr-09-gmv-lineage.json`
5. `dashboards/sales/fy26-q3-sales-dashboard-export.csv`
6. `dashboards/product/fy26-q3-product-mix-dashboard.png`
7. `reports/weekly/2026-07-13-weekly-gmv-review.pdf`
8. `reports/monthly/fy26-q3-monthly-performance-report.xlsx`
9. `forecasts/scenarios/fy26-q3-gmv-scenario-model.xlsx`
10. `requests/stakeholder/mtr-09-definition-request.eml`
11. `exports/warehouse/2026-07-13-gmv-warehouse-export.tsv`
12. `meetings/metric-reviews/2026-07-13-mtr-09-metric-review.pptx`

For seed XLSX files, a visible title/note carries the exact anchor `GMV ¥84.2M`, while a calculated cell carries numeric `84200000` with JPY number formatting.

## Common quality gates

- All artifacts are substantive, internally coherent, professionally named, and distinct by content; no template clones or filler.
- Text, CSV/TSV, HTML/EML, JSON/YAML/XML/SQL, and code parse with an appropriate parser. CSV/TSV have useful headers and multiple rows.
- Code uses documentation/reserved examples only and contains no credentials, destructive actions, or live endpoints.
- IPYNB files have valid markdown/code/saved outputs; deterministic rerun output must match; render each notebook to HTML and inspect it.
- DOCX files use the Documents skill, an appropriate business preset/header, generally span at least two pages, render every page, and pass visual inspection.
- Searchable PDFs use the PDF skill, have a real text layer, generally span at least two pages, render every page, and pass visual inspection.
- Scan PDFs use ImageGen sources plus the PDF skill, contain at least two image-only pages, have no extractable text layer, render every page, and pass visual inspection.
- XLSX files use the Spreadsheets skill and `@oai/artifact-tool`; meaningful workbooks separate raw/assumptions/derived content where applicable, include formulas, contain zero formula errors, render every sheet, and pass visual inspection.
- PPTX files use the Presentations skill and `@oai/artifact-tool`, follow the Codex Grid design route, contain at least five slides, render every slide, pass visual inspection, and pass `slides_test.py` with overflow count zero.
- Each of the six final PNG files is created with a distinct ImageGen call, is a true PNG, and is inspected at original detail. Scan-PDF source images also use ImageGen and remain only in the scope scratch directory.
- Each PCAP has a valid libpcap header and synthetic packets using documentation/reserved IP ranges only; validate with a parser such as `tcpdump -nr` or Scapy.
- Final leaves contain only the exact listed artifacts. Build files, scripts, renders, caches, and QA evidence stay under the scope scratch leaf.

## Scope assignments

### p05-primary-01 — `analytics/sql/production` — 20 files

Families: `structured_text=12`, `code=4`, `md=2`, `txt_log=2`. Supporting facts: production-safe MTR-09 logic, net-of-refund reconciliation, parameter guards, and synthetic warehouse execution evidence. No live connection strings.

- `fy26-q3-gmv-mtr09-production-query.sql` (structured_text, seed)
- `dim-store-current-state.sql` (structured_text)
- `fct-order-line-net-sales.sql` (structured_text)
- `mtr09-daily-gmv-aggregate.sql` (structured_text)
- `mtr09-channel-reconciliation.sql` (structured_text)
- `mtr09-return-adjustment.sql` (structured_text)
- `mtr09-tax-exclusion.sql` (structured_text)
- `mtr09-late-arrival-backfill.sql` (structured_text)
- `mtr09-data-quality-checks.sql` (structured_text)
- `mtr09-weekly-rollup.sql` (structured_text)
- `mtr09-monthly-rollup.sql` (structured_text)
- `mtr09-access-view.sql` (structured_text)
- `run-production-query.py` (code)
- `validate-gmv-contract.py` (code)
- `parameter-guard.ts` (code)
- `warehouse-schema-check.rs` (code)
- `production-query-runbook.md` (md)
- `mtr09-change-log.md` (md)
- `2026-07-13-production-query-run.log` (txt_log)
- `mtr09-query-review-notes.txt` (txt_log)

### p05-primary-02 — `analytics/sql/ad-hoc` — 18 files

Families: `structured_text=6`, `code=4`, `ipynb=4`, `md=4`. Supporting facts: variance decomposition by channel, region, category, promotion, and late-arriving records; deterministic local notebook results.

- `channel-mix-variance.sql` (structured_text)
- `regional-gmv-variance.sql` (structured_text)
- `promotion-lift-check.sql` (structured_text)
- `return-timing-sensitivity.sql` (structured_text)
- `category-drift-analysis.sql` (structured_text)
- `store-cohort-comparison.sql` (structured_text)
- `sandbox-query-runner.py` (code)
- `sampling-helpers.py` (code)
- `query-budget-guard.ts` (code)
- `column-profile.rs` (code)
- `gmv-variance-decomposition.ipynb` (ipynb)
- `channel-mix-exploration.ipynb` (ipynb)
- `promotion-lift-exploration.ipynb` (ipynb)
- `late-arrival-sensitivity.ipynb` (ipynb)
- `fy26-q3-gmv-variance-notes.md` (md, seed)
- `ad-hoc-query-catalog.md` (md)
- `2026-07-13-analysis-log.md` (md)
- `mtr09-assumption-notes.md` (md)

### p05-primary-03 — `analytics/data-dictionary` — 16 files

Families: `structured_text=6`, `csv_tsv=4`, `md=3`, `xlsx=2`, `docx=1`. Supporting facts: MTR-09 definition, grains, dimensions, ownership, allowed values, thresholds, and bilingual terminology. Skills: Documents and Spreadsheets for their respective files.

- `mtr09-metric-schema.json` (structured_text)
- `mtr09-dimensions.yaml` (structured_text)
- `mtr09-registry.xml` (structured_text)
- `mtr09-calculation-contract.json` (structured_text)
- `data-quality-status-codes.yaml` (structured_text)
- `order-line-grain.xml` (structured_text)
- `metric-field-catalog.csv` (csv_tsv)
- `dimension-allowed-values.csv` (csv_tsv)
- `metric-owner-matrix.tsv` (csv_tsv)
- `quality-thresholds.csv` (csv_tsv)
- `dictionary-maintenance-guide.md` (md)
- `business-terms-glossary.md` (md)
- `mtr09-frequently-asked-questions.md` (md)
- `mtr09-data-dictionary-workbook.xlsx` (xlsx)
- `field-coverage-audit.xlsx` (xlsx)
- `mtr-09-metric-data-dictionary.docx` (docx, seed)

### p05-primary-04 — `analytics/lineage` — 16 files

Families: `structured_text=4`, `code=2`, `domain_binary=4`, `md=2`, `csv_tsv=3`, `html_eml=1`. Supporting facts: synthetic OneDrive-to-warehouse lineage, transformations, owners, refresh checkpoints, and reserved-address connector diagnostics.

- `mtr-09-gmv-lineage.json` (structured_text, seed)
- `warehouse-lineage-nodes.yaml` (structured_text)
- `order-line-transform-map.xml` (structured_text)
- `lineage-quality-contract.json` (structured_text)
- `lineage-contract-validator.py` (code)
- `lineage-graph-export.ts` (code)
- `onedrive-ingest-handshake.pcap` (domain_binary)
- `warehouse-refresh-handshake.pcap` (domain_binary)
- `semantic-model-refresh.pcap` (domain_binary)
- `lineage-api-sample.pcap` (domain_binary)
- `mtr09-lineage-overview.md` (md)
- `lineage-incident-playbook.md` (md)
- `lineage-node-inventory.csv` (csv_tsv)
- `transformation-step-catalog.csv` (csv_tsv)
- `refresh-checkpoints.tsv` (csv_tsv)
- `mtr09-lineage-report.html` (html_eml)

### p05-primary-05 — `dashboards/sales` — 14 files

Families: `csv_tsv=6`, `xlsx=4`, `image=1`, `pdf_text=1`, `pptx=1`, `html_eml=1`. Supporting facts: sales GMV by day, region, channel, category, plan, and prior period. Skills: Spreadsheets, ImageGen, PDF, and Presentations.

- `fy26-q3-sales-dashboard-export.csv` (csv_tsv, seed)
- `sales-gmv-daily-series.csv` (csv_tsv)
- `sales-gmv-by-region.csv` (csv_tsv)
- `sales-gmv-by-channel.tsv` (csv_tsv)
- `sales-gmv-by-category.csv` (csv_tsv)
- `sales-dashboard-filter-state.csv` (csv_tsv)
- `sales-dashboard-model.xlsx` (xlsx)
- `sales-kpi-reconciliation.xlsx` (xlsx)
- `sales-region-drilldown.xlsx` (xlsx)
- `sales-channel-drilldown.xlsx` (xlsx)
- `fy26-q3-sales-region-heatmap.png` (image)
- `2026-07-13-sales-dashboard-snapshot.pdf` (pdf_text)
- `fy26-q3-sales-dashboard-review.pptx` (pptx)
- `sales-dashboard-accessible-export.html` (html_eml)

### p05-primary-06 — `dashboards/product` — 12 files

Families: `csv_tsv=4`, `xlsx=3`, `image=2`, `pdf_text=1`, `pptx=1`, `html_eml=1`. Supporting facts: product mix, category contribution, returns, and margin-context notes without inventing live financial data. Skills: Spreadsheets, ImageGen, PDF, and Presentations.

- `product-mix-export.csv` (csv_tsv)
- `product-category-gmv.csv` (csv_tsv)
- `product-return-rate.tsv` (csv_tsv)
- `product-dashboard-filter-state.csv` (csv_tsv)
- `product-mix-model.xlsx` (xlsx)
- `category-contribution-analysis.xlsx` (xlsx)
- `product-return-reconciliation.xlsx` (xlsx)
- `fy26-q3-product-mix-dashboard.png` (image, seed)
- `fy26-q3-category-contribution-chart.png` (image)
- `2026-07-13-product-dashboard-snapshot.pdf` (pdf_text)
- `fy26-q3-product-dashboard-review.pptx` (pptx)
- `product-dashboard-accessible-export.html` (html_eml)

### p05-primary-07 — `reports/weekly` — 12 files

Families: `pdf_text=4`, `xlsx=2`, `docx=1`, `pptx=1`, `csv_tsv=2`, `md=2`. Supporting facts: weekly operating review, daily bridge to ¥84.2M, exceptions, decisions, and owners. Skills: PDF, Spreadsheets, Documents, and Presentations.

- `2026-07-13-weekly-gmv-review.pdf` (pdf_text, seed)
- `2026-07-13-weekly-channel-review.pdf` (pdf_text)
- `2026-07-13-weekly-region-review.pdf` (pdf_text)
- `2026-07-13-weekly-data-quality-review.pdf` (pdf_text)
- `weekly-gmv-bridge.xlsx` (xlsx)
- `weekly-exception-tracker.xlsx` (xlsx)
- `weekly-performance-commentary.docx` (docx)
- `2026-07-13-weekly-review-deck.pptx` (pptx)
- `weekly-gmv-detail.csv` (csv_tsv)
- `weekly-exception-detail.tsv` (csv_tsv)
- `weekly-review-action-log.md` (md)
- `weekly-metric-notes.md` (md)

### p05-primary-08 — `reports/monthly` — 10 files

Families: `xlsx=5`, `pdf_text=2`, `docx=1`, `pptx=1`, `image=1`. Supporting facts: FY26 Q3 month-to-date performance, plan/prior bridges, category outlook, and controlled commentary. Skills: Spreadsheets, PDF, Documents, Presentations, and ImageGen.

- `fy26-q3-monthly-performance-report.xlsx` (xlsx, seed)
- `fy26-q3-monthly-gmv-bridge.xlsx` (xlsx)
- `fy26-q3-monthly-channel-detail.xlsx` (xlsx)
- `fy26-q3-monthly-region-detail.xlsx` (xlsx)
- `fy26-q3-monthly-quality-scorecard.xlsx` (xlsx)
- `fy26-q3-monthly-performance-summary.pdf` (pdf_text)
- `fy26-q3-monthly-control-review.pdf` (pdf_text)
- `fy26-q3-monthly-commentary.docx` (docx)
- `fy26-q3-monthly-review.pptx` (pptx)
- `fy26-q3-monthly-gmv-waterfall.png` (image)

### p05-primary-09 — `forecasts/scenarios` — 10 files

Families: `xlsx=6`, `ipynb=2`, `csv_tsv=2`. Supporting facts: base/upside/downside scenarios, assumption isolation, sensitivity, and deterministic forecast calculations. Skill: Spreadsheets.

- `fy26-q3-gmv-scenario-model.xlsx` (xlsx, seed)
- `fy26-q3-base-case.xlsx` (xlsx)
- `fy26-q3-upside-case.xlsx` (xlsx)
- `fy26-q3-downside-case.xlsx` (xlsx)
- `fy26-q3-channel-sensitivity.xlsx` (xlsx)
- `fy26-q3-refund-sensitivity.xlsx` (xlsx)
- `gmv-scenario-simulation.ipynb` (ipynb)
- `gmv-driver-sensitivity.ipynb` (ipynb)
- `scenario-assumptions.csv` (csv_tsv)
- `scenario-results.tsv` (csv_tsv)

### p05-primary-10 — `requests/stakeholder` — 8 files

Families: `html_eml=3`, `docx=1`, `pdf_text=1`, `md=3`. Supporting facts: fictional request intake, definition clarification, approval trail, response SLA, and decision records. Skills: Documents and PDF.

- `mtr-09-definition-request.eml` (html_eml, seed)
- `fy26-q3-gmv-variance-followup.eml` (html_eml)
- `stakeholder-request-dashboard.html` (html_eml)
- `mtr09-definition-response.docx` (docx)
- `mtr09-definition-approval.pdf` (pdf_text)
- `stakeholder-request-register.md` (md)
- `mtr09-definition-decision.md` (md)
- `request-triage-playbook.md` (md)

### p05-primary-11 — `exports/warehouse` — 8 files

Families: `csv_tsv=6`, `domain_binary=2`. Supporting facts: synthetic warehouse extracts with schema/version metadata, aggregate-only sales data, and reserved-address export diagnostics.

- `2026-07-13-gmv-warehouse-export.tsv` (csv_tsv, seed)
- `2026-07-13-order-line-aggregate.csv` (csv_tsv)
- `2026-07-13-channel-aggregate.csv` (csv_tsv)
- `2026-07-13-region-aggregate.csv` (csv_tsv)
- `2026-07-13-category-aggregate.csv` (csv_tsv)
- `2026-07-13-refund-aggregate.tsv` (csv_tsv)
- `warehouse-export-transfer.pcap` (domain_binary)
- `warehouse-export-validation.pcap` (domain_binary)

### p05-primary-12 — `meetings/metric-reviews` — 6 files

Families: `pptx=3`, `docx=1`, `pdf_text=1`, `image=1`. Supporting facts: metric governance review, decision log, action owners, and executive-ready summary. Skills: Presentations, Documents, PDF, and ImageGen.

- `2026-07-13-mtr-09-metric-review.pptx` (pptx, seed)
- `2026-07-13-mtr-09-definition-workshop.pptx` (pptx)
- `fy26-q3-gmv-executive-brief.pptx` (pptx)
- `2026-07-13-metric-review-minutes.docx` (docx)
- `2026-07-13-metric-review-readout.pdf` (pdf_text)
- `mtr09-review-decision-map.png` (image)

### p05-secondary-01 — `desktop/working` — 10 files

Families: `xlsx=3`, `ipynb=2`, `csv_tsv=2`, `txt_log=2`, `code=1`. Supporting facts: believable analyst working copies, reproducible scratch analysis, and local-only logs. Skill: Spreadsheets.

- `gmv-review-working-copy.xlsx` (xlsx)
- `region-bridge-working.xlsx` (xlsx)
- `channel-check-working.xlsx` (xlsx)
- `gmv-outlier-check.ipynb` (ipynb)
- `refund-timing-check.ipynb` (ipynb)
- `working-filter-sample.csv` (csv_tsv)
- `working-variance-sample.tsv` (csv_tsv)
- `2026-07-13-refresh-debug.log` (txt_log)
- `analysis-todo-notes.txt` (txt_log)
- `aggregate-check.rs` (code)

### p05-secondary-02 — `documents/reference` — 8 files

Families: `docx=1`, `pdf_scan=2`, `xlsx=1`, `html_eml=1`, `txt_log=1`, `pptx=1`, `image=1`. Supporting facts: internal reference material, wholly synthetic scanned source sheets, metric governance aids, and a diagram. Skills: Documents, PDF, ImageGen, Spreadsheets, and Presentations.

- `metric-governance-handbook.docx` (docx)
- `synthetic-store-close-sheet.pdf` (pdf_scan)
- `synthetic-refund-review-sheet.pdf` (pdf_scan)
- `metric-reference-catalog.xlsx` (xlsx)
- `mtr09-reference-index.html` (html_eml)
- `warehouse-field-notes.txt` (txt_log)
- `metric-onboarding-reference.pptx` (pptx)
- `mtr09-calculation-reference.png` (image)

### p05-secondary-03 — `downloads/inbox` — 8 files

Families: `csv_tsv=3`, `xlsx=1`, `ipynb=1`, `html_eml=1`, `txt_log=1`, `domain_binary=1`. Supporting facts: plausible incoming synthetic exports with source labels, schema notes, and reserved-address transfer trace. Skill: Spreadsheets.

- `incoming-store-gmv.csv` (csv_tsv)
- `incoming-channel-gmv.tsv` (csv_tsv)
- `incoming-refund-adjustments.csv` (csv_tsv)
- `incoming-sales-workbook.xlsx` (xlsx)
- `incoming-profile-check.ipynb` (ipynb)
- `warehouse-export-notification.eml` (html_eml)
- `download-receipt.jsonl` (txt_log)
- `download-transfer-sample.pcap` (domain_binary)

### p05-secondary-04 — `downloads/exports` — 6 files

Families: `csv_tsv=4`, `xlsx=1`, `domain_binary=1`. Supporting facts: outbound synthetic analyst extracts with aggregation level, generation time, and reserved-address trace. Skill: Spreadsheets.

- `sales-dashboard-download.csv` (csv_tsv)
- `product-dashboard-download.csv` (csv_tsv)
- `weekly-review-download.tsv` (csv_tsv)
- `monthly-review-download.csv` (csv_tsv)
- `dashboard-download-bundle.xlsx` (xlsx)
- `dashboard-export-sample.pcap` (domain_binary)

### p05-secondary-05 — `cloud/my-files` — 6 files

Families: `csv_tsv=1`, `xlsx=1`, `ipynb=1`, `txt_log=1`, `code=1`, `domain_binary=1`. Supporting facts: personal synthetic OneDrive working artifacts and reserved-address connector diagnostics. Skill: Spreadsheets.

- `onedrive-gmv-snapshot.csv` (csv_tsv)
- `personal-analysis-checklist.xlsx` (xlsx)
- `personal-gmv-sanity-check.ipynb` (ipynb)
- `onedrive-sync-status.log` (txt_log)
- `onedrive-export-check.py` (code)
- `onedrive-connector-sample.pcap` (domain_binary)

### p05-secondary-06 — `cloud/team-shared` — 4 files

Families: `csv_tsv=2`, `xlsx=1`, `txt_log=1`. Supporting facts: team-shared metric calendar, review register, and controlled handoff notes. Skill: Spreadsheets.

- `team-metric-calendar.csv` (csv_tsv)
- `team-review-action-register.tsv` (csv_tsv)
- `team-shared-gmv-tracker.xlsx` (xlsx)
- `team-handoff-notes.txt` (txt_log)

### p05-secondary-07 — `mail/recent` — 4 files

Families: `html_eml=2`, `txt_log=2`. Supporting facts: synthetic correspondence about review scheduling, export completion, and analyst follow-up; use fictional recipients only.

- `metric-review-invitation.eml` (html_eml)
- `warehouse-refresh-complete.eml` (html_eml)
- `recent-mail-index.txt` (txt_log)
- `metric-followup-thread.log` (txt_log)

### p05-secondary-08 — `archive/closed` — 4 files

Families: `csv_tsv=1`, `domain_binary=3`. Supporting facts: closed synthetic extract and historical reserved-address diagnostics, clearly marked archived.

- `closed-fy26-q2-gmv-reconciliation.csv` (csv_tsv)
- `closed-fy26-q2-export-transfer.pcap` (domain_binary)
- `closed-fy26-q2-refresh-check.pcap` (domain_binary)
- `closed-fy26-q2-dashboard-session.pcap` (domain_binary)

## Parent acceptance checklist

- Per-scope exact path/count/family/name match and no extra final files
- All scope-local parsers and render QA pass before release
- 12/12 exact seeds contain 5/5 exact anchors; the two seed XLSX files also contain numeric `84200000` with JPY formatting
- Final global family ledger matches exactly; 200 unique SHA-256 values and zero duplicate-content groups
- Final manifest has exactly 200 verified rows relative to the p05 home root
- No final caches, temporary files, build scripts, render images, lock files, or hidden metadata
- p01–p04 manifests and all 800 referenced files remain unchanged
- canonical plan, workspace owner, runtime, Git HEAD/branch/status/worktree remain unchanged
- All 20 scope leases release normally, then the parent lease releases normally, then `active_leases=0`
