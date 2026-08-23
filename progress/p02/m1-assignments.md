# p02 M1 fixed assignments

- Persona: `p02` / Site Reliability Engineer
- Final root: `workspace/people/p02-site-reliability-engineer/home`
- Milestone: M1, exactly 200 final files across 20 plan-defined leaf scopes
- Parent session: `p02-m1-codex-root-1787412818512`
- Common anchors for every `seed=yes` artifact: `Northstar Transit Cloud`; `Checkout SVC-17`; `2026-07-13`; `error budget 2.4%`; `INC-260713`
- Release tokens are intentionally absent from this document.
- Filenames and format-family assignments below are frozen before any scope worker is started.

## Required global format distribution

| Family | Count |
|---|---:|
| md | 40 |
| txt_log | 44 |
| code | 30 |
| structured_text | 40 |
| csv_tsv | 10 |
| html_eml | 6 |
| ipynb | 0 |
| pdf_text | 8 |
| pdf_scan | 0 |
| docx | 4 |
| xlsx | 2 |
| pptx | 2 |
| image | 4 |
| media | 0 |
| domain_binary | 10 |
| **Total** | **200** |

## p02-primary-01

- Leaf folder: `documents/operations/runbooks`
- Count: 20
- Seed artifact: `checkout-svc17-error-budget-breach.md`
- Required skill: none
- Subagent: `p02_primary_01_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-error-budget-breach.md | md | Markdown | yes |
| 2 | checkout-svc17-database-failover.md | md | Markdown | no |
| 3 | checkout-svc17-payment-queue-saturation.md | md | Markdown | no |
| 4 | checkout-svc17-regional-traffic-shift.md | md | Markdown | no |
| 5 | checkout-svc17-cache-degradation.md | md | Markdown | no |
| 6 | identity-token-validation-degradation.md | md | Markdown | no |
| 7 | identity-session-store-failover.md | md | Markdown | no |
| 8 | edge-ingress-5xx-spike.md | md | Markdown | no |
| 9 | postgres-replica-lag.md | md | Markdown | no |
| 10 | message-bus-consumer-lag.md | md | Markdown | no |
| 11 | service-mesh-certificate-rotation.md | md | Markdown | no |
| 12 | synthetic-check-failure-triage.md | md | Markdown | no |
| 13 | observability-pipeline-backpressure.md | md | Markdown | no |
| 14 | log-export-delivery-gap.md | md | Markdown | no |
| 15 | kubernetes-node-pressure.md | md | Markdown | no |
| 16 | kubernetes-pod-crashloop.md | md | Markdown | no |
| 17 | terraform-plan-drift-response.md | md | Markdown | no |
| 18 | error-budget-policy-exception.md | md | Markdown | no |
| 19 | incident-commander-handoff.md | md | Markdown | no |
| 20 | regional-disaster-recovery.md | md | Markdown | no |

## p02-primary-02

- Leaf folder: `documents/operations/postmortems`
- Count: 18
- Seed artifact: `INC-260713-checkout-svc17-postmortem.docx`
- Required skill: Documents
- Subagent: `p02_primary_02_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | INC-260713-checkout-svc17-postmortem.docx | docx | DOCX | yes |
| 2 | INC-260628-identity-cache-postmortem.docx | docx | DOCX | no |
| 3 | INC-260519-ingress-dns-postmortem.docx | docx | DOCX | no |
| 4 | INC-260407-metrics-pipeline-postmortem.docx | docx | DOCX | no |
| 5 | INC-260713-follow-up-tracker.md | md | Markdown | no |
| 6 | postmortem-review-guidelines.md | md | Markdown | no |
| 7 | recurrence-analysis-2026-Q2.md | md | Markdown | no |
| 8 | corrective-actions-status.md | md | Markdown | no |
| 9 | INC-260713-incident-chat.txt | txt_log | UTF-8 text transcript | no |
| 10 | INC-260713-timeline.log | txt_log | operational log | no |
| 11 | INC-260628-incident-chat.txt | txt_log | UTF-8 text transcript | no |
| 12 | INC-260519-timeline.log | txt_log | operational log | no |
| 13 | INC-260407-metrics-recovery.log | txt_log | operational log | no |
| 14 | postmortem-action-audit.txt | txt_log | UTF-8 audit text | no |
| 15 | INC-260713-events.json | structured_text | JSON | no |
| 16 | INC-260713-impact.yaml | structured_text | YAML | no |
| 17 | postmortem-action-register.xml | structured_text | XML | no |
| 18 | incident-duration-analysis.sql | structured_text | SQL | no |

## p02-primary-03

- Leaf folder: `infrastructure/kubernetes`
- Count: 16
- Seed artifact: `checkout-svc17-production-deployment.yaml`
- Required skill: none
- Subagent: `p02_primary_03_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-production-deployment.yaml | structured_text | Kubernetes YAML | yes |
| 2 | checkout-svc17-service.yaml | structured_text | Kubernetes YAML | no |
| 3 | checkout-svc17-hpa.yaml | structured_text | Kubernetes YAML | no |
| 4 | checkout-svc17-pdb.yaml | structured_text | Kubernetes YAML | no |
| 5 | checkout-svc17-networkpolicy.yaml | structured_text | Kubernetes YAML | no |
| 6 | checkout-svc17-servicemonitor.yaml | structured_text | Kubernetes YAML | no |
| 7 | identity-production-deployment.yaml | structured_text | Kubernetes YAML | no |
| 8 | identity-service.yaml | structured_text | Kubernetes YAML | no |
| 9 | identity-hpa.yaml | structured_text | Kubernetes YAML | no |
| 10 | identity-networkpolicy.yaml | structured_text | Kubernetes YAML | no |
| 11 | regional-ingress.yaml | structured_text | Kubernetes YAML | no |
| 12 | pod-security-standards.yaml | structured_text | Kubernetes YAML | no |
| 13 | node-affinity-rules.yaml | structured_text | Kubernetes YAML | no |
| 14 | priority-classes.yaml | structured_text | Kubernetes YAML | no |
| 15 | maintenance-cronjob.yaml | structured_text | Kubernetes YAML | no |
| 16 | observability-exporter-daemonset.yaml | structured_text | Kubernetes YAML | no |

## p02-primary-04

- Leaf folder: `infrastructure/terraform`
- Count: 16
- Seed artifact: `checkout-svc17-observability.tf`
- Required skill: none
- Subagent: `p02_primary_04_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-observability.tf | code | Terraform HCL | yes |
| 2 | versions.tf | code | Terraform HCL | no |
| 3 | providers.tf | code | Terraform HCL | no |
| 4 | variables.tf | code | Terraform HCL | no |
| 5 | locals.tf | code | Terraform HCL | no |
| 6 | checkout-svc17-service.tf | code | Terraform HCL | no |
| 7 | identity-service.tf | code | Terraform HCL | no |
| 8 | regional-load-balancers.tf | code | Terraform HCL | no |
| 9 | dns-failover.tf | code | Terraform HCL | no |
| 10 | monitoring-workspaces.tf | code | Terraform HCL | no |
| 11 | alert-routing.tf | code | Terraform HCL | no |
| 12 | log-exports.tf | code | Terraform HCL | no |
| 13 | capacity-reservations.tf | code | Terraform HCL | no |
| 14 | maintenance-windows.tf | code | Terraform HCL | no |
| 15 | incident-tags.tf | code | Terraform HCL | no |
| 16 | outputs.tf | code | Terraform HCL | no |

## p02-primary-05

- Leaf folder: `services/checkout/prod/oncall/operations`
- Count: 14
- Seed artifact: `INC-260713-checkout-svc17-incident.log`
- Required skill: none
- Subagent: `p02_primary_05_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | INC-260713-checkout-svc17-incident.log | txt_log | operational incident log | yes |
| 2 | checkout-svc17-primary-oncall-handoff.txt | txt_log | UTF-8 handoff text | no |
| 3 | checkout-svc17-secondary-oncall-handoff.txt | txt_log | UTF-8 handoff text | no |
| 4 | checkout-svc17-mitigation-commands.log | txt_log | command log | no |
| 5 | checkout-svc17-database-failover.log | txt_log | operational log | no |
| 6 | checkout-svc17-queue-drain.log | txt_log | operational log | no |
| 7 | checkout-svc17-traffic-shift.log | txt_log | operational log | no |
| 8 | checkout-svc17-cache-recovery.log | txt_log | operational log | no |
| 9 | checkout-svc17-payment-provider-check.log | txt_log | operational log | no |
| 10 | checkout-svc17-synthetic-probe.log | txt_log | operational log | no |
| 11 | checkout-svc17-recovery-validation.log | txt_log | operational log | no |
| 12 | checkout-svc17-customer-impact.txt | txt_log | UTF-8 incident note | no |
| 13 | checkout-svc17-follow-up-actions.txt | txt_log | UTF-8 action note | no |
| 14 | checkout-svc17-shift-summary.txt | txt_log | UTF-8 shift summary | no |

## p02-primary-06

- Leaf folder: `services/identity/prod/oncall/operations`
- Count: 12
- Seed artifact: `identity-fallback-during-INC-260713.md`
- Required skill: none
- Subagent: `p02_primary_06_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | identity-fallback-during-INC-260713.md | md | Markdown | yes |
| 2 | identity-token-validation-runbook.md | md | Markdown | no |
| 3 | identity-session-store-failover.md | md | Markdown | no |
| 4 | identity-key-rotation-procedure.md | md | Markdown | no |
| 5 | identity-rate-limit-degradation.md | md | Markdown | no |
| 6 | identity-directory-sync-lag.md | md | Markdown | no |
| 7 | identity-mfa-provider-failure.md | md | Markdown | no |
| 8 | identity-regional-traffic-shift.md | md | Markdown | no |
| 9 | identity-cache-flush-procedure.md | md | Markdown | no |
| 10 | identity-audit-log-gap.md | md | Markdown | no |
| 11 | identity-break-glass-validation.md | md | Markdown | no |
| 12 | identity-oncall-handoff.md | md | Markdown | no |

## p02-primary-07

- Leaf folder: `observability/alerts`
- Count: 12
- Seed artifact: `checkout-svc17-error-budget-alert.json`
- Required skill: none
- Subagent: `p02_primary_07_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-error-budget-alert.json | structured_text | JSON | yes |
| 2 | checkout-svc17-latency-alert.json | structured_text | JSON | no |
| 3 | checkout-svc17-availability-alert.json | structured_text | JSON | no |
| 4 | checkout-svc17-queue-depth-alert.json | structured_text | JSON | no |
| 5 | checkout-svc17-payment-error-alert.json | structured_text | JSON | no |
| 6 | identity-authentication-failure-alert.json | structured_text | JSON | no |
| 7 | identity-token-latency-alert.json | structured_text | JSON | no |
| 8 | identity-session-store-alert.json | structured_text | JSON | no |
| 9 | regional-ingress-error-alert.json | structured_text | JSON | no |
| 10 | postgres-replica-lag-alert.json | structured_text | JSON | no |
| 11 | kubernetes-node-pressure-alert.json | structured_text | JSON | no |
| 12 | log-export-gap-alert.json | structured_text | JSON | no |

## p02-primary-08

- Leaf folder: `observability/dashboards`
- Count: 10
- Seed artifact: `checkout-svc17-error-budget-dashboard.csv`
- Required skill: Spreadsheets
- Subagent: `p02_primary_08_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-error-budget-dashboard.csv | csv_tsv | CSV | yes |
| 2 | checkout-svc17-latency-panels.csv | csv_tsv | CSV | no |
| 3 | checkout-svc17-availability-panels.csv | csv_tsv | CSV | no |
| 4 | identity-slo-dashboard.csv | csv_tsv | CSV | no |
| 5 | regional-traffic-dashboard.csv | csv_tsv | CSV | no |
| 6 | platform-saturation-dashboard.csv | csv_tsv | CSV | no |
| 7 | incident-response-dashboard.csv | csv_tsv | CSV | no |
| 8 | observability-pipeline-dashboard.csv | csv_tsv | CSV | no |
| 9 | checkout-svc17-slo-review.xlsx | xlsx | XLSX workbook | no |
| 10 | northstar-oncall-health.xlsx | xlsx | XLSX workbook | no |

## p02-primary-09

- Leaf folder: `observability/log-exports`
- Count: 10
- Seed artifact: `checkout-svc17-INC-260713-export.log`
- Required skill: none
- Subagent: `p02_primary_09_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-INC-260713-export.log | txt_log | operational log | yes |
| 2 | checkout-svc17-access-sample.jsonl | txt_log | JSON Lines log | no |
| 3 | checkout-svc17-error-sample.jsonl | txt_log | JSON Lines log | no |
| 4 | identity-authentication-sample.jsonl | txt_log | JSON Lines log | no |
| 5 | regional-ingress-2026-07-13.log | txt_log | operational log | no |
| 6 | postgres-performance-2026-07-13.log | txt_log | operational log | no |
| 7 | kubernetes-events-2026-07-13.log | txt_log | operational log | no |
| 8 | alert-router-delivery.log | txt_log | operational log | no |
| 9 | log-exporter-retry.log | txt_log | operational log | no |
| 10 | incident-correlation.log | txt_log | operational log | no |

## p02-primary-10

- Leaf folder: `changes/deployments`
- Count: 8
- Seed artifact: `2026-07-13-checkout-svc17-change.html`
- Required skill: none
- Subagent: `p02_primary_10_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | 2026-07-13-checkout-svc17-change.html | html_eml | HTML | yes |
| 2 | 2026-07-15-identity-hotfix.html | html_eml | HTML | no |
| 3 | checkout-svc17-rollback-checklist.md | md | Markdown | no |
| 4 | deployment-approval-notes.md | md | Markdown | no |
| 5 | deploy-20260713.yaml | structured_text | YAML | no |
| 6 | change-freeze-exceptions.json | structured_text | JSON | no |
| 7 | deploy-20260713.log | txt_log | deployment log | no |
| 8 | rollback-drill.txt | txt_log | UTF-8 drill record | no |

## p02-primary-11

- Leaf folder: `capacity/reports`
- Count: 8
- Seed artifact: `2026-07-13-checkout-svc17-capacity-review.pdf`
- Required skill: PDF
- Subagent: `p02_primary_11_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | 2026-07-13-checkout-svc17-capacity-review.pdf | pdf_text | text-based PDF | yes |
| 2 | 2026-Q3-checkout-demand-forecast.pdf | pdf_text | text-based PDF | no |
| 3 | 2026-Q3-identity-capacity-plan.pdf | pdf_text | text-based PDF | no |
| 4 | regional-ingress-headroom.pdf | pdf_text | text-based PDF | no |
| 5 | postgres-storage-growth.pdf | pdf_text | text-based PDF | no |
| 6 | message-bus-throughput-plan.pdf | pdf_text | text-based PDF | no |
| 7 | observability-retention-capacity.pdf | pdf_text | text-based PDF | no |
| 8 | holiday-readiness-capacity.pdf | pdf_text | text-based PDF | no |

## p02-primary-12

- Leaf folder: `meetings/operations`
- Count: 6
- Seed artifact: `2026-07-13-operations-review.pptx`
- Required skill: Presentations
- Subagent: `p02_primary_12_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | 2026-07-13-operations-review.pptx | pptx | PowerPoint | yes |
| 2 | 2026-Q3-reliability-planning.pptx | pptx | PowerPoint | no |
| 3 | 2026-07-13-operations-minutes.md | md | Markdown | no |
| 4 | 2026-Q3-reliability-planning-notes.md | md | Markdown | no |
| 5 | operations-review-action-log.txt | txt_log | UTF-8 action log | no |
| 6 | reliability-planning-decisions.txt | txt_log | UTF-8 decision record | no |

## p02-secondary-01

- Leaf folder: `desktop/working`
- Count: 10
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_01_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | annotate-incident-timeline.py | code | Python | no |
| 2 | slo-window-calculator.py | code | Python | no |
| 3 | merge-alert-snapshots.ts | code | TypeScript | no |
| 4 | generate-handoff.ts | code | TypeScript | no |
| 5 | log-gap-scanner.rs | code | Rust | no |
| 6 | pcap-indexer.rs | code | Rust | no |
| 7 | terminal-session-2026-07-13.log | txt_log | terminal log | no |
| 8 | scratchpad-commands.txt | txt_log | UTF-8 command notes | no |
| 9 | working-alert-map.yaml | structured_text | YAML | no |
| 10 | incident-correlation.sql | structured_text | SQL | no |

## p02-secondary-02

- Leaf folder: `documents/reference`
- Count: 8
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_02_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | burn-rate-query.py | code | Python | no |
| 2 | synthetic-check.ts | code | TypeScript | no |
| 3 | lease-safe-export.rs | code | Rust | no |
| 4 | terraform-state-scrubber.py | code | Python | no |
| 5 | slo-taxonomy.xml | structured_text | XML | no |
| 6 | incident-schema.json | structured_text | JSON | no |
| 7 | service-ownership.csv | csv_tsv | CSV | no |
| 8 | severity-matrix.tsv | csv_tsv | TSV | no |

## p02-secondary-03

- Leaf folder: `downloads/inbox`
- Count: 8
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_03_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-edge-sample-20260713.pcap | domain_binary | PCAP | no |
| 2 | checkout-db-sample-20260713.pcap | domain_binary | PCAP | no |
| 3 | checkout-queue-sample-20260713.pcap | domain_binary | PCAP | no |
| 4 | identity-auth-sample-20260713.pcap | domain_binary | PCAP | no |
| 5 | regional-ingress-sample-20260713.pcap | domain_binary | PCAP | no |
| 6 | observability-export-sample-20260713.pcap | domain_binary | PCAP | no |
| 7 | capture-intake.log | txt_log | operational log | no |
| 8 | capture-checksums.txt | txt_log | UTF-8 checksum record | no |

## p02-secondary-04

- Leaf folder: `downloads/exports`
- Count: 6
- Seed artifact: none
- Required skill: ImageGen
- Subagent: `p02_secondary_04_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-svc17-latency-heatmap.png | image | PNG | no |
| 2 | INC-260713-incident-topology.png | image | PNG | no |
| 3 | northstar-error-budget-burn.png | image | PNG | no |
| 4 | regional-failover-map.png | image | PNG | no |
| 5 | export-notes.txt | txt_log | UTF-8 export note | no |
| 6 | image-export.log | txt_log | operational log | no |

## p02-secondary-05

- Leaf folder: `cloud/my-files`
- Count: 6
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_05_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | slo-digest.py | code | Python | no |
| 2 | maintenance-window.ts | code | TypeScript | no |
| 3 | alert-router.rs | code | Rust | no |
| 4 | capacity-forecast.py | code | Python | no |
| 5 | personal-slo-notes.yaml | structured_text | YAML | no |
| 6 | query-library.sql | structured_text | SQL | no |

## p02-secondary-06

- Leaf folder: `cloud/team-shared`
- Count: 4
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_06_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | checkout-identity-handoff-20260713.txt | txt_log | UTF-8 team handoff | no |
| 2 | error-budget-policy-discussion.log | txt_log | decision log | no |
| 3 | platform-oncall-rotation-notes.txt | txt_log | UTF-8 team notes | no |
| 4 | q3-reliability-working-session.log | txt_log | working-session log | no |

## p02-secondary-07

- Leaf folder: `mail/recent`
- Count: 4
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_07_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | INC-260713-status-update.eml | html_eml | RFC 5322 EML | no |
| 2 | checkout-svc17-follow-up.eml | html_eml | RFC 5322 EML | no |
| 3 | identity-maintenance-notice.eml | html_eml | RFC 5322 EML | no |
| 4 | q3-reliability-review.eml | html_eml | RFC 5322 EML | no |

## p02-secondary-08

- Leaf folder: `archive/closed`
- Count: 4
- Seed artifact: none
- Required skill: none
- Subagent: `p02_secondary_08_worker`

| # | Exact filename | Family | Concrete format | Seed |
|---:|---|---|---|---|
| 1 | INC-260713-checkout-edge-closed.pcap | domain_binary | PCAP | no |
| 2 | INC-260628-identity-cache-closed.pcap | domain_binary | PCAP | no |
| 3 | INC-260519-regional-ingress-closed.pcap | domain_binary | PCAP | no |
| 4 | INC-260407-metrics-pipeline-closed.pcap | domain_binary | PCAP | no |
