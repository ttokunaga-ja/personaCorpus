# p03 M1 fixed assignments

Status: frozen before first Subagent launch

Persona: p03 security-grc-analyst (Cobalt Harbor Systems)

Final root (repository-relative): workspace/people/p03-security-grc-analyst/home

Parent session: p03-m1-parent-1787418032041-r1 (release token intentionally omitted)

## Immutable production contract

- Exactly 20 plan-defined leaf scopes and 200 final files.
- Each named Subagent owns exactly one leaf scope and may write only its final leaf plus `scratch/p03/<scope-id>`.
- Parent claims the Rust scope ID immediately before launch and retains every release token in memory only.
- No worker may spawn a child agent. Assigned filenames do not change after the first launch.
- All content is fictional, defensive/audit-oriented, ja-JP plus English, with documentation-only networks for PCAP.
- No live scanning, exploit/payload/malware, credentials, secrets, real PII, routable IPs, placeholders, Lorem ipsum, or exact duplicates.
- Rich artifacts use the named skill and keep builders/renders/QA evidence only under the scope scratch directory.
- Seed rows must naturally contain all five exact anchors: `Cobalt Harbor`, `Aegis Control AC-27`, `2026-07-13`, `evidence set 184`, `RISK-031`.

## Fixed file ledger

| Scope ID | Repository-relative leaf path | Exact filename | Format family | Concrete format | Seed | Required skill | Assigned Subagent | Required rendered surface |
|---|---|---|---|---|---|---|---|---|
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | RISK-031-aegis-ac27-threat-model.md | md | Markdown | seed-01 | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | identity-boundary-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | evidence-ingestion-trust-boundaries.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | vendor-portal-dataflow-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | audit-export-integrity-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | siem-evidence-pipeline-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | managed-endpoint-grc-workflow.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | control-testing-authorization-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | s3-evidence-retention-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | sharepoint-audit-library-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | risk-register-change-control.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | exception-approval-abuse-cases.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | audit-request-spoofing-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | evidence-chain-of-custody.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | third-party-assessment-boundaries.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | privacy-review-dataflow.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | detection-rule-governance-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | incident-evidence-access-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | control-owner-handoff-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-01 | workspace/people/p03-security-grc-analyst/home/security/threat-models | quarterly-attestation-threat-model.md | md | Markdown | — | — | p03_primary_01_worker | n/a |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | 2026-07-13-aegis-ac27-defensive-assessment.pdf | pdf_text | PDF (text) | seed-02 | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | identity-access-review-assessment.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | evidence-export-integrity-assessment.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | vendor-portal-configuration-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | siem-parser-assurance-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | endpoint-hardening-validation.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | cloud-storage-permission-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | audit-workflow-abuse-case-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | control-evidence-retention-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | third-party-sso-configuration-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | security-header-compliance-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | logging-coverage-gap-assessment.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | backup-restoration-control-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | privileged-role-separation-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | privacy-intake-security-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | change-approval-control-assessment.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | incident-evidence-handling-review.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-02 | workspace/people/p03-security-grc-analyst/home/security/pentest-reports | q3-defensive-testing-summary.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_02_worker | 2 pages |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | RISK-031-vulnerability-register-20260713.csv | csv_tsv | CSV | seed-03 | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | open-findings-register.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | remediation-sla-tracker.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | control-gap-inventory.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | endpoint-exposure-summary.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | cloud-configuration-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | application-review-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | vendor-security-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | evidence-quality-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | identity-review-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | siem-coverage-findings.csv | csv_tsv | CSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | finding-owner-matrix.tsv | csv_tsv | TSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | severity-calibration.tsv | csv_tsv | TSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | exception-expiry-tracker.tsv | csv_tsv | TSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | verification-sample-plan.tsv | csv_tsv | TSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-03 | workspace/people/p03-security-grc-analyst/home/security/vulnerabilities | closed-finding-audit.tsv | csv_tsv | TSV | — | — | p03_primary_03_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260713-RISK-031-incident-report.docx | docx | DOCX | seed-04 | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260629-evidence-export-incident.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260611-vendor-portal-access-review.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260528-siem-ingestion-gap.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260506-policy-exception-misroute.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260418-audit-library-permission-drift.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260327-endpoint-evidence-delay.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260219-control-owner-handoff-gap.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260117-retention-label-mismatch.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | 2026-Q2-security-incident-summary.docx | docx | DOCX | — | documents | p03_primary_04_worker | 2 pages |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260713-timeline.log | txt_log | LOG | — | — | p03_primary_04_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | INC-260713-decision-record.txt | txt_log | TXT | — | — | p03_primary_04_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | incident-evidence-intake.log | txt_log | LOG | — | — | p03_primary_04_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | incident-role-handoff.txt | txt_log | TXT | — | — | p03_primary_04_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | corrective-action-status.log | txt_log | LOG | — | — | p03_primary_04_worker | n/a |
| p03-primary-04 | workspace/people/p03-security-grc-analyst/home/security/incident-reports | incident-review-index.txt | txt_log | TXT | — | — | p03_primary_04_worker | n/a |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | aegis-ac27-evidence-handling-policy.pdf | pdf_text | PDF (text) | seed-05 | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | information-security-governance-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | risk-acceptance-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | vulnerability-management-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | security-incident-reporting-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | vendor-risk-management-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | audit-evidence-retention-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | access-review-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | logging-monitoring-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | privacy-risk-assessment-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | control-testing-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | security-exception-policy.pdf | pdf_text | PDF (text) | — | pdf | p03_primary_05_worker | 2 pages |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | policy-control-crosswalk.yaml | structured_text | YAML | — | — | p03_primary_05_worker | n/a |
| p03-primary-05 | workspace/people/p03-security-grc-analyst/home/compliance/policies | policy-review-calendar.json | structured_text | JSON | — | — | p03_primary_05_worker | n/a |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-27-evidence-set-184-scan.pdf | pdf_scan | PDF (scan) | seed-06 | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-04-access-review-sample-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-09-logging-attestation-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-12-vendor-review-approval-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-16-incident-tabletop-record-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-21-retention-verification-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-24-privacy-review-checklist-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-31-change-approval-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-35-backup-restore-witness-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | AC-42-quarterly-control-signoff-scan.pdf | pdf_scan | PDF (scan) | — | imagegen + pdf | p03_primary_06_worker | 1 page |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | scan-batch-index.txt | txt_log | TXT | — | — | p03_primary_06_worker | n/a |
| p03-primary-06 | workspace/people/p03-security-grc-analyst/home/compliance/control-evidence | scan-quality-review.log | txt_log | LOG | — | — | p03_primary_06_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260713-AC27.eml | html_eml | EML | seed-07 | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260714-access-review.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260715-logging-evidence.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260716-vendor-risk.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260717-retention-sample.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260720-incident-followup.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260721-privacy-control.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260722-change-approval.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260723-backup-restore.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-request-AR-20260724-exception-register.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-response-AR-20260713-clarification.eml | html_eml | EML | — | — | p03_primary_07_worker | n/a |
| p03-primary-07 | workspace/people/p03-security-grc-analyst/home/compliance/audit-requests | audit-weekly-status-20260724.html | html_eml | HTML | — | — | p03_primary_07_worker | n/a |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | Cobalt-Harbor-AC27-vendor-questionnaire.xlsx | xlsx | XLSX | seed-08 | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | HarborLink-security-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | BlueQuay-privacy-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | IronBeacon-cloud-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | NorthPier-data-processing-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | Tideglass-business-continuity-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | Seawall-identity-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | Mariner-SOC-questionnaire.xlsx | xlsx | XLSX | — | spreadsheets | p03_primary_08_worker | 3 sheets |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | questionnaire-status-register.csv | csv_tsv | CSV | — | — | p03_primary_08_worker | n/a |
| p03-primary-08 | workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires | vendor-assessment-scores.csv | csv_tsv | CSV | — | — | p03_primary_08_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | evidence-set-184-siem-export.json | structured_text | JSON | seed-09 | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | aegis-ac27-control-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | RISK-031-correlation-events.jsonl | structured_text | JSONL | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | identity-review-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | audit-library-access-events.jsonl | structured_text | JSONL | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | vendor-portal-review-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | endpoint-compliance-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | policy-exception-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | evidence-retention-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-09 | workspace/people/p03-security-grc-analyst/home/soc/siem-exports | incident-case-events.json | structured_text | JSON | — | — | p03_primary_09_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | aegis-ac27-evidence-anomaly-rule.yaml | structured_text | YAML | seed-10 | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | risk-register-change-alert.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | audit-library-permission-drift.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | evidence-export-integrity-alert.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | vendor-questionnaire-escalation.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | control-attestation-overdue.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | incident-evidence-gap.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-10 | workspace/people/p03-security-grc-analyst/home/soc/detection-rules | privacy-review-bypass-alert.yaml | structured_text | YAML | — | — | p03_primary_10_worker | n/a |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | RISK-031-privacy-dataflow.png | image | PNG | seed-11 | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | aegis-ac27-evidence-map.png | image | PNG | — | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | privacy-intake-trust-boundaries.png | image | PNG | — | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | vendor-data-handling-overview.png | image | PNG | — | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | audit-evidence-lifecycle.png | image | PNG | — | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | risk-treatment-decision-map.png | image | PNG | — | imagegen | p03_primary_11_worker | 1 image |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | image-export-notes.txt | txt_log | TXT | — | — | p03_primary_11_worker | n/a |
| p03-primary-11 | workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments | privacy-visual-review.log | txt_log | LOG | — | — | p03_primary_11_worker | n/a |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | 2026-07-13-security-review.pptx | pptx | PPTX | seed-12 | presentations | p03_primary_12_worker | 5 slides |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | 2026-Q3-control-assurance-review.pptx | pptx | PPTX | — | presentations | p03_primary_12_worker | 5 slides |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | RISK-031-risk-committee-readout.pptx | pptx | PPTX | — | presentations | p03_primary_12_worker | 5 slides |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | vendor-risk-quarterly-review.pptx | pptx | PPTX | — | presentations | p03_primary_12_worker | 5 slides |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | security-review-decisions.txt | txt_log | TXT | — | — | p03_primary_12_worker | n/a |
| p03-primary-12 | workspace/people/p03-security-grc-analyst/home/meetings/security-reviews | security-review-action-log.txt | txt_log | TXT | — | — | p03_primary_12_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | evidence-manifest-check.py | code | Python | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | control-crosswalk.ts | code | TypeScript | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | risk-score-calculator.rs | code | Rust | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | sanitize-siem-export.py | code | Python | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | audit-request-validator.ts | code | TypeScript | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | pcap-evidence-index.rs | code | Rust | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | working-control-map.yaml | structured_text | YAML | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | incident-correlation.sql | structured_text | SQL | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | terminal-session-2026-07-13.log | txt_log | LOG | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-01 | workspace/people/p03-security-grc-analyst/home/desktop/working | analyst-scratchpad.txt | txt_log | TXT | — | — | p03_secondary_01_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | evidence-hash-audit.py | code | Python | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | policy-link-check.ts | code | TypeScript | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | finding-deduplicator.rs | code | Rust | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | questionnaire-score.py | code | Python | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | control-taxonomy.xml | structured_text | XML | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | finding-schema.json | structured_text | JSON | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | control-owner-directory.csv | csv_tsv | CSV | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-02 | workspace/people/p03-security-grc-analyst/home/documents/reference | evidence-classification.tsv | csv_tsv | TSV | — | — | p03_secondary_02_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | siem-ingest-sample-20260713.pcap | domain_binary | PCAP | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | audit-export-sample-20260713.pcap | domain_binary | PCAP | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | vendor-portal-sample-20260713.pcap | domain_binary | PCAP | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | identity-review-sample-20260713.pcap | domain_binary | PCAP | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | download-intake.log | txt_log | LOG | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | pcap-checksums.txt | txt_log | TXT | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | evidence-transfer-note.txt | txt_log | TXT | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-03 | workspace/people/p03-security-grc-analyst/home/downloads/inbox | inbox-triage.log | txt_log | LOG | — | — | p03_secondary_03_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | export-redactor.py | code | Python | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | manifest-normalizer.rs | code | Rust | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | export-bundle-index.json | structured_text | JSON | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | redaction-profile.yaml | structured_text | YAML | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | export-readme.txt | txt_log | TXT | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-04 | workspace/people/p03-security-grc-analyst/home/downloads/exports | export-generation.log | txt_log | LOG | — | — | p03_secondary_04_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | personal-risk-digest.py | code | Python | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | control-review-calendar.ts | code | TypeScript | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | evidence-integrity.rs | code | Rust | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | audit-sample-selector.py | code | Python | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | personal-review-notes.yaml | structured_text | YAML | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-05 | workspace/people/p03-security-grc-analyst/home/cloud/my-files | query-library.sql | structured_text | SQL | — | — | p03_secondary_05_worker | n/a |
| p03-secondary-06 | workspace/people/p03-security-grc-analyst/home/cloud/team-shared | ac27-control-owner-handoff.txt | txt_log | TXT | — | — | p03_secondary_06_worker | n/a |
| p03-secondary-06 | workspace/people/p03-security-grc-analyst/home/cloud/team-shared | risk-committee-working-session.log | txt_log | LOG | — | — | p03_secondary_06_worker | n/a |
| p03-secondary-06 | workspace/people/p03-security-grc-analyst/home/cloud/team-shared | audit-readiness-notes.txt | txt_log | TXT | — | — | p03_secondary_06_worker | n/a |
| p03-secondary-06 | workspace/people/p03-security-grc-analyst/home/cloud/team-shared | q3-control-assurance-decisions.log | txt_log | LOG | — | — | p03_secondary_06_worker | n/a |
| p03-secondary-07 | workspace/people/p03-security-grc-analyst/home/mail/recent | RISK-031-status-update.eml | html_eml | EML | — | — | p03_secondary_07_worker | n/a |
| p03-secondary-07 | workspace/people/p03-security-grc-analyst/home/mail/recent | AC27-evidence-followup.eml | html_eml | EML | — | — | p03_secondary_07_worker | n/a |
| p03-secondary-07 | workspace/people/p03-security-grc-analyst/home/mail/recent | vendor-review-reminder.html | html_eml | HTML | — | — | p03_secondary_07_worker | n/a |
| p03-secondary-07 | workspace/people/p03-security-grc-analyst/home/mail/recent | q3-audit-readiness-review.html | html_eml | HTML | — | — | p03_secondary_07_worker | n/a |
| p03-secondary-08 | workspace/people/p03-security-grc-analyst/home/archive/closed | RISK-031-closed-evidence.pcap | domain_binary | PCAP | — | — | p03_secondary_08_worker | n/a |
| p03-secondary-08 | workspace/people/p03-security-grc-analyst/home/archive/closed | INC-260528-siem-gap-closed.pcap | domain_binary | PCAP | — | — | p03_secondary_08_worker | n/a |
| p03-secondary-08 | workspace/people/p03-security-grc-analyst/home/archive/closed | closed-risk-register.json | structured_text | JSON | — | — | p03_secondary_08_worker | n/a |
| p03-secondary-08 | workspace/people/p03-security-grc-analyst/home/archive/closed | archive-retention-index.xml | structured_text | XML | — | — | p03_secondary_08_worker | n/a |

## Frozen totals

- Scope counts: p03-primary-01=20, p03-primary-02=18, p03-primary-03=16, p03-primary-04=16, p03-primary-05=14, p03-primary-06=12, p03-primary-07=12, p03-primary-08=10, p03-primary-09=10, p03-primary-10=8, p03-primary-11=8, p03-primary-12=6, p03-secondary-01=10, p03-secondary-02=8, p03-secondary-03=8, p03-secondary-04=6, p03-secondary-05=6, p03-secondary-06=4, p03-secondary-07=4, p03-secondary-08=4.
- Format families: md=20, txt_log=24, code=16, structured_text=30, csv_tsv=20, html_eml=16, ipynb=0, pdf_text=30, pdf_scan=10, docx=10, xlsx=8, pptx=4, image=6, media=0, domain_binary=6; total=200.
- Rich outputs: pdf_text=30 (60 pages), pdf_scan=10 (10 pages), docx=10 (20 pages), xlsx=8 (24 sheets), pptx=4 (20 slides), image=6.
- Seeds: seed-01=workspace/people/p03-security-grc-analyst/home/security/threat-models/RISK-031-aegis-ac27-threat-model.md; seed-02=workspace/people/p03-security-grc-analyst/home/security/pentest-reports/2026-07-13-aegis-ac27-defensive-assessment.pdf; seed-03=workspace/people/p03-security-grc-analyst/home/security/vulnerabilities/RISK-031-vulnerability-register-20260713.csv; seed-04=workspace/people/p03-security-grc-analyst/home/security/incident-reports/INC-260713-RISK-031-incident-report.docx; seed-05=workspace/people/p03-security-grc-analyst/home/compliance/policies/aegis-ac27-evidence-handling-policy.pdf; seed-06=workspace/people/p03-security-grc-analyst/home/compliance/control-evidence/AC-27-evidence-set-184-scan.pdf; seed-07=workspace/people/p03-security-grc-analyst/home/compliance/audit-requests/audit-request-AR-20260713-AC27.eml; seed-08=workspace/people/p03-security-grc-analyst/home/vendor-risk/questionnaires/Cobalt-Harbor-AC27-vendor-questionnaire.xlsx; seed-09=workspace/people/p03-security-grc-analyst/home/soc/siem-exports/evidence-set-184-siem-export.json; seed-10=workspace/people/p03-security-grc-analyst/home/soc/detection-rules/aegis-ac27-evidence-anomaly-rule.yaml; seed-11=workspace/people/p03-security-grc-analyst/home/privacy/risk-assessments/RISK-031-privacy-dataflow.png; seed-12=workspace/people/p03-security-grc-analyst/home/meetings/security-reviews/2026-07-13-security-review.pptx.
- Machine validation: `scratch/p03/validate_m1.py --assignment-only` before launch; full validation without that flag after production.
