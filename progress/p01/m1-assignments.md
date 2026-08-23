# p01 milestone 1 fixed assignments

This token-free checkpoint fixes the first 200 final artifacts for `p01`
(`software-engineer`). Counts are derived from the accepted full-plan scope
allocation as `scope.raw_files / 60`; no scope, path, or quota is redefined
here.

## Shared content and quality requirements

- Use only fully synthetic facts for **Aster Loop 株式会社** and its Product
  Alpha/Beta engineering work. Do not include real names, email addresses,
  customers, credentials, tokens, keys, or copied private/copyrighted source.
- Keep the release world internally consistent: **Product Alpha R7** ships on
  **2026-07-13**, its observed latency anchor is **p95 184 ms**, and
  **ADR-042** selects the asynchronous release gate. Seed artifacts explicitly
  show all five anchors; supporting files use the same facts where relevant.
- Final files go only in the exact folder named for their scope. Builders,
  sources, renders, extracted text, and QA images go under
  `scratch/p01/<scope-id>/` and are not final artifacts.
- Every final artifact must contain substantive, realistic engineering content;
  no empty shells, lorem ipsum, unexplained placeholders, or duplicated filler.
  Extensions must match real bytes. JSON/JSONL/YAML/CSV/notebooks must parse,
  source code must be coherent, EML must be RFC-style mail, SQLite/ZIP files
  must be real binary containers, and Office/PDF/image files must open.
- `pdf_text`, `pdf_scan`, `docx`, `xlsx`, `pptx`, and `image` are skill-routed.
  Follow the applicable skill's creation, render, and full visual-inspection
  gate. A `pdf_scan` must visibly look scanned and have no accidental text
  layer; a `pdf_text` must have extractable text. ImageGen is required for the
  final PNGs and for scan-source artwork.

## Fixed totals

| Family | Count |
|---|---:|
| md | 44 |
| txt_log | 16 |
| code | 56 |
| structured_text | 24 |
| csv_tsv | 6 |
| html_eml | 10 |
| ipynb | 2 |
| pdf_text | 14 |
| pdf_scan | 2 |
| docx | 6 |
| xlsx | 4 |
| pptx | 4 |
| image | 6 |
| media | 0 |
| domain_binary | 6 |
| **Total** | **200** |

## p01-primary-01 — 20 files

Final folder: `workspace/people/p01-software-engineer/home/documents/work/product-alpha/architecture`

- `md` `ADR-042-async-release-gate.md` — Seed ADR with status, context, decision, alternatives, consequences, rollout, p95 184 ms evidence, 2026-07-13 approval/sign-off roles, and all anchors.
- `md` `alpha-r7-system-context.md` — C4-style prose and Mermaid context for clients, edge API, release coordinator, event bus, and observability boundaries.
- `md` `event-contract-boundaries.md` — Ownership, versioning, idempotency, retry, and failure-boundary rules for release events.
- `md` `p95-latency-budget.md` — 184 ms end-to-end budget decomposition, measurement window, alert thresholds, and trade-offs.
- `md` `rollback-strategy.md` — Trigger matrix, feature-flag sequencing, data compatibility, verification, and rollback stop conditions.
- `txt_log` `architecture-review-notes-2026-07-10.txt` — Natural bilingual review notes with disagreements, decisions, and follow-ups; no real people.
- `code` `release_gate.rs` — Compilable-looking async state machine with typed states, idempotency key, timeouts, and tests.
- `code` `latency_budget.rs` — Rust budget constants and evaluator yielding the 184 ms p95 decision.
- `code` `topology_types.ts` — TypeScript service/event topology types and validation helpers.
- `code` `health_probe.go` — Go readiness probe aggregating coordinator, queue, and datastore health.
- `code` `schema_guard.py` — Python compatibility checker with deterministic sample inputs and clear diagnostics.
- `code` `migration_check.sql` — Transaction-safe preflight SQL with invariant and rollback queries.
- `code` `adr_lint.mjs` — Node ESM linter checking ADR front matter, anchors, headings, and decision status.
- `structured_text` `architecture-components.yaml` — Parseable component catalog with owners, SLOs, dependencies, and data classifications.
- `structured_text` `alpha-r7-risk-register.json` — Parseable risks with likelihood, impact, mitigation, owner role, and due date.
- `csv_tsv` `latency-budget-breakdown.csv` — Typed stage rows summing to 184 ms, with target, observed, and measurement source.
- `pdf_text` `alpha-r7-architecture-overview.pdf` — Polished 3-5 page text-layer architecture brief with one clear system-flow figure and visible anchors.
- `pdf_text` `adr-042-approval-record.pdf` — Text-layer approval packet recording rationale, evidence, objections, resolution, and synthetic role sign-offs.
- `docx` `adr-042-decision-memo.docx` — Polished decision memo using a documented preset, explicit table geometry, and 2-4 clean rendered pages.
- `pptx` `alpha-r7-architecture-walkthrough.pptx` — 5-7 slide internal walkthrough with narrative arc, architecture flow, latency budget, decision, and rollout; no overflow.

## p01-primary-02 — 18 files

Final folder: `workspace/people/p01-software-engineer/home/documents/work/product-alpha/api-contracts`

- `md` `api-contract-conventions.md` — Resource naming, pagination, errors, idempotency, dates, versioning, and deprecation policy.
- `code` `generated_client.ts` — Typed Alpha R7 client with request IDs, pagination, retries, and example usage.
- `code` `request_validator.rs` — Rust request/response validation with structured errors and tests.
- `code` `pagination.ts` — Cursor encoding/decoding and stable-order pagination helpers.
- `code` `error_mapping.go` — Go mapping from domain failures to documented HTTP errors.
- `code` `contract_test.py` — Deterministic contract tests for happy path, conflict, validation, and timeout behavior.
- `code` `webhook_signature.ts` — Synthetic signing/verification algorithm using obvious non-secret fixtures only.
- `code` `schema_diff.mjs` — ESM compatibility diff for required fields, enum narrowing, and endpoint removal.
- `structured_text` `openapi-alpha-r7.yaml` — Seed OpenAPI 3.1 contract for release creation/status, with examples, p95 description, errors, and all anchors.
- `structured_text` `openapi-alpha-r7-overlay.yaml` — Parseable overlay adding internal descriptions and release-window metadata.
- `structured_text` `asyncapi-release-events.yaml` — Parseable release.requested/approved/completed event contract with correlation and retry semantics.
- `structured_text` `error-catalog.json` — Stable synthetic error codes, HTTP mappings, retryability, and user-safe messages.
- `structured_text` `examples-create-release.json` — Valid create request/response fixture for Product Alpha R7.
- `structured_text` `examples-release-status.json` — Valid status fixture showing ADR-042 gate progression.
- `structured_text` `spectral-rules.yaml` — Lint rules for operation IDs, descriptions, request IDs, and example coverage.
- `csv_tsv` `status-code-matrix.csv` — Endpoint-by-status coverage with error codes and retry guidance.
- `html_eml` `api-contract-preview.html` — Standalone accessible HTML reference with navigation, endpoint tables, examples, and responsive styling.
- `pdf_text` `alpha-r7-api-review.pdf` — Text-layer design review with contract summary, compatibility findings, risks, and disposition.

## p01-primary-03 — 16 files

Final folder: `workspace/people/p01-software-engineer/home/documents/work/product-alpha/release-notes`

- `md` `product-alpha-r7-release-notes.md` — Seed release notes dated 2026-07-13 with highlights, breaking/non-breaking changes, p95 184 ms, ADR-042, upgrade and rollback links.
- `md` `upgrade-guide.md` — Ordered preflight, deployment, validation, and recovery steps for R6 to R7.
- `md` `known-issues.md` — Realistic bounded issues with symptoms, impact, workaround, owner role, and status.
- `md` `rollback-notes.md` — Compatibility window, rollback triggers, data caveats, commands, and verification evidence.
- `txt_log` `release-qa-summary.txt` — Concise signed-off QA narrative with synthetic role labels and test counts.
- `txt_log` `canary-observations.log` — Timestamped synthetic canary log with p50/p95/p99 and one resolved warning.
- `txt_log` `customer-impact-summary.txt` — S1/S2-safe impact summary without real customer or contact information.
- `code` `generate_changelog.ts` — Deterministic TypeScript grouping commits into release-note sections.
- `code` `verify_release_tags.rs` — Rust verifier for semantic version, commit ancestry, and R7 release date.
- `code` `render_notes.mjs` — ESM Markdown-to-HTML release note renderer with escaping and headings.
- `code` `backport_report.py` — Python report generator over synthetic pull-request fixtures.
- `structured_text` `release-metadata.yaml` — Parseable version, date, artifact, compatibility, owner-role, and decision references.
- `structured_text` `artifact-digests.json` — Synthetic artifact names and valid-looking non-secret SHA-256 fixture values.
- `csv_tsv` `change-summary.csv` — Change ID, area, compatibility, risk, reviewer role, and documentation link.
- `html_eml` `release-notes-preview.html` — Polished accessible standalone HTML rendering of the R7 notes.
- `xlsx` `alpha-r7-release-verification.xlsx` — Formula-driven workbook with Checks, Metrics, and Summary sheets; pass rate and p95 margin formulas, no formula errors, all sheets visually verified.

## p01-primary-04 — 16 files

Final folder: `workspace/people/p01-software-engineer/home/documents/work/product-beta/architecture`

- `md` `beta-architecture-overview.md` — Service boundaries, data flow, async execution, and Alpha coexistence strategy.
- `md` `rust-service-boundaries.md` — Crate/module ownership, public APIs, error boundaries, and test seams.
- `md` `adr-043-beta-storage.md` — Candidate storage ADR explicitly related to but distinct from ADR-042.
- `txt_log` `benchmark-run-2026-07-09.log` — Synthetic repeatable benchmark output with environment, samples, p95, and notes.
- `code` `product-beta-design.rs` — Seed Rust design code containing domain types, async traits, state transitions, ADR-042 reference, and tests.
- `code` `command_router.rs` — Typed command dispatch with cancellation and structured errors.
- `code` `event_store.rs` — Append/read interface with optimistic concurrency and in-memory test implementation.
- `code` `projection_worker.rs` — Idempotent checkpointed projection loop with retry policy.
- `code` `latency_sampler.rs` — Histogram sample collector and percentile calculation centered on the 184 ms budget.
- `code` `feature_flags.rs` — Typed staged-rollout flags with safe defaults and audit context.
- `code` `config_loader.rs` — Layered config parser with validation and synthetic fixtures only.
- `code` `integration_boundary.rs` — Alpha/Beta adapter traits and compatibility errors.
- `code` `architecture_tests.rs` — Scenario tests for duplicate events, partial failure, rollback, and recovery.
- `structured_text` `beta-components.yaml` — Parseable crates/services, ownership roles, dependencies, and maturity.
- `structured_text` `beta-feature-flags.json` — Parseable flag definitions, cohorts, defaults, and expiry dates.
- `domain_binary` `beta-benchmark-fixture.sqlite` — Real SQLite database containing benchmark runs, percentile samples, schema version, and anchor metadata.

## p01-primary-05 — 14 files

Final folder: `workspace/people/p01-software-engineer/home/documents/work/product-beta/api-contracts`

- `md` `beta-contract-notes.md` — Beta preview scope, compatibility stance, naming, unresolved questions, and rollout constraints.
- `code` `product-beta-contract.ts` — Seed TypeScript contract with discriminated unions, branded IDs, release state, validation, and all anchors.
- `code` `beta-client.ts` — Typed preview client with cancellation, retry classification, and examples.
- `code` `beta-validator.ts` — Runtime validators and human-readable issue paths.
- `code` `beta-errors.ts` — Stable error taxonomy and HTTP/event mappings.
- `code` `beta-pagination.ts` — Cursor contract and deterministic ordering helpers.
- `code` `beta-webhooks.ts` — Webhook envelope, delivery attempt, and acknowledgement types.
- `code` `beta-contract-tests.ts` — Valid/invalid fixtures exercising backward compatibility.
- `structured_text` `openapi-beta-preview.yaml` — Parseable OpenAPI 3.1 preview with examples and Alpha comparison notes.
- `structured_text` `beta-examples.json` — Request/response/event examples sharing consistent synthetic IDs.
- `structured_text` `beta-error-catalog.json` — Error records with retryability, status, and remediation.
- `structured_text` `beta-spectral-rules.yaml` — Preview-contract lint rules and deprecation checks.
- `html_eml` `beta-api-preview.html` — Accessible standalone preview reference with endpoint and model sections.
- `pdf_text` `beta-api-design-review.pdf` — Text-layer review summarizing differences, compatibility risks, and next decisions.

## p01-primary-06 — 12 files

Final folder: `workspace/people/p01-software-engineer/home/repos/product-alpha/docs`

- `md` `README.md` — Seed repository README with purpose, architecture, setup, test, release flow, ADR-042, 2026-07-13 R7, and p95 184 ms.
- `md` `local-development.md` — macOS ja-JP prerequisites, synthetic config, commands, tests, and troubleshooting.
- `md` `release-checklist.md` — Preflight through post-release checklist with owner roles and evidence links.
- `md` `observability-guide.md` — Metrics, logs, traces, dashboards, alert interpretation, and latency SLO.
- `md` `troubleshooting.md` — Symptom-to-diagnosis runbook for queue lag, schema mismatch, and p95 regression.
- `code` `quickstart.ts` — Safe client example using synthetic endpoints and values, with no credential-shaped strings.
- `code` `sample_client.rs` — Rust client example with error handling and request correlation.
- `code` `smoke_test.sh` — Strict shell smoke test using local/synthetic fixtures and readable failure output.
- `html_eml` `docs-preview.html` — Standalone accessible documentation landing page linking the fixed docs.
- `ipynb` `latency-analysis.ipynb` — Valid notebook with Markdown narrative, deterministic synthetic samples, percentile calculation, and outputs showing 184 ms.
- `pdf_text` `alpha-r7-developer-guide.pdf` — Polished text-layer developer guide with setup, flow, examples, testing, and release checklist.
- `pdf_scan` `whiteboard-auth-flow-scan.pdf` — ImageGen-created synthetic photographed/scan-style auth-and-release-flow sheet, wrapped as a real image-only PDF and visually inspected.

## p01-primary-07 — 12 files

Final folder: `workspace/people/p01-software-engineer/home/repos/product-beta/docs`

- `md` `beta-migration-guide.md` — Seed migration guide from Alpha R7 to Beta preview with phases, compatibility, dry run, rollback, and all anchors.
- `md` `README.md` — Beta docs index, maturity notice, local setup, and links to the fixed files.
- `md` `compatibility-notes.md` — API, event, storage, configuration, and operational compatibility matrix.
- `md` `feature-flag-rollout.md` — Cohort sequence, observability gates, abort criteria, and flag retirement.
- `md` `troubleshooting-beta.md` — Symptoms and fixes for projection lag, adapter mismatch, and flag drift.
- `txt_log` `migration-dry-run.log` — Timestamped synthetic dry-run output with one recovered incompatibility.
- `code` `migrate_config.ts` — Typed configuration transformer with dry-run and validation.
- `code` `verify_beta_state.rs` — Rust invariant verifier with structured results and tests.
- `code` `rollback_beta.sh` — Strict, safe illustrative rollback steps over local synthetic fixtures.
- `html_eml` `migration-preview.html` — Accessible standalone rendered migration guide.
- `docx` `beta-migration-field-guide.docx` — Compact polished operator guide with steps, checkpoints, rollback callout, and clean full-page renders.
- `image` `beta-migration-flow.png` — ImageGen-produced readable migration flow with Alpha R7, adapter, dual-read, Beta projection, verification, and rollback path.

## p01-primary-08 — 10 files

Final folder: `workspace/people/p01-software-engineer/home/work-items/decision-records`

- `md` `ADR-042-async-release-gate.md` — Canonical working copy of the seed decision with traceable context, decision, and consequences.
- `md` `ADR-039-p95-budget.md` — Decision establishing the 184 ms R7 latency budget and measurement method.
- `md` `ADR-040-event-versioning.md` — Event compatibility and deprecation decision.
- `md` `ADR-041-beta-feature-flags.md` — Feature-flag ownership, rollout, expiry, and fallback decision.
- `txt_log` `decision-log-index.txt` — Human-readable chronological index with status and cross-references.
- `structured_text` `decision-status.json` — Parseable status ledger with IDs, dates, supersession, and owner roles.
- `pdf_text` `adr-042-evidence-pack.pdf` — Text-layer evidence packet with latency data, failure scenarios, and alternatives.
- `pdf_text` `adr-042-risk-review.pdf` — Text-layer risk review with likelihood/impact, mitigations, and accepted residual risk.
- `docx` `ADR-042-decision-record.docx` — Seed polished decision DOCX with all anchors, clear hierarchy, decision callout, alternatives, and sign-off roles.
- `pptx` `architecture-decision-review.pptx` — 5-6 slide decision review covering problem, evidence, alternatives, decision, and rollout.

## p01-primary-09 — 10 files

Final folder: `workspace/people/p01-software-engineer/home/work-items/code-reviews`

- `md` `review-842-summary.md` — Review summary for async release-gate changes, findings, resolution, and approval conditions.
- `md` `review-849-followup.md` — Follow-up review on latency instrumentation and schema guard fixes.
- `txt_log` `reviewer-notes.txt` — Natural reviewer scratch notes with synthetic role labels only.
- `code` `auth_guard.rs` — Reviewed Rust snippet with corrected error paths and tests.
- `code` `release_gate_review.ts` — Reviewed TypeScript state transition logic with inline rationale comments.
- `code` `latency_query.sql` — Reviewed percentile query with correct window/filter semantics.
- `structured_text` `code-review-export.jsonl` — Seed parseable JSONL export of reviews, comments, patches metadata, timestamps, dispositions, and all anchors.
- `csv_tsv` `reviewer-turnaround.csv` — Synthetic review timing by change, stage, and reviewer role.
- `html_eml` `review-842-thread.html` — Accessible standalone review conversation with code blocks and resolved threads.
- `pdf_text` `review-842-approval.pdf` — Text-layer approval summary with evidence, residual risks, and checks.

## p01-primary-10 — 8 files

Final folder: `workspace/people/p01-software-engineer/home/meetings/engineering`

- `md` `2026-07-08-architecture-agenda.md` — Time-boxed agenda, pre-reads, decisions required, and roles.
- `md` `2026-07-08-action-items.md` — Action list with synthetic owner roles, due dates, status, and evidence.
- `txt_log` `meeting-chat-export.txt` — Natural asynchronous chat transcript without real identities or addresses.
- `structured_text` `meeting-actions.yaml` — Parseable action/decision ledger linked to ADR-042.
- `pdf_text` `architecture-pre-read.pdf` — Text-layer pre-read summarizing problem, options, latency evidence, and questions.
- `docx` `architecture-decisions-memo.docx` — Polished post-meeting memo with decisions, rationale, actions, and open items.
- `pptx` `2026-07-08-design-review.pptx` — Seed 6-8 slide design meeting deck with all anchors, coherent narrative, readable architecture/latency visuals, and speaker-note sources where applicable.
- `image` `event-flow-whiteboard.png` — ImageGen-produced synthetic whiteboard-style event flow, readable and consistent with ADR-042.

## p01-primary-11 — 8 files

Final folder: `workspace/people/p01-software-engineer/home/vendor-docs/platforms`

- `md` `cloud-runtime-evaluation.md` — Synthetic evaluation of three invented platform tiers against latency, operations, and portability criteria.
- `md` `vendor-sdk-integration-notes.md` — Adapter patterns, version pinning, retry semantics, and known limitations.
- `structured_text` `platform-version-pins.yaml` — Parseable invented package/version ledger with compatibility notes and review dates.
- `html_eml` `vendor-api-reference-snapshot.html` — Standalone synthetic API snapshot clearly labeled internal evaluation material.
- `pdf_text` `platform-integration-guide.pdf` — Seed polished text-layer PDF with setup, lifecycle, error handling, and Alpha R7 integration examples; all anchors visible.
- `xlsx` `platform-compatibility-matrix.xlsx` — Formula-driven multi-sheet comparison with weighted criteria, typed inputs, scores, and visually verified summary.
- `image` `platform-deployment-topology.png` — ImageGen-produced deployment topology with no real vendor logos or names and readable synthetic labels.
- `domain_binary` `vendor-sdk-snapshot.zip` — Real ZIP containing a synthetic README, manifest, tiny code sample, and license notice invented for Aster Loop evaluation.

## p01-primary-12 — 6 files

Final folder: `workspace/people/p01-software-engineer/home/operations/migration-notes`

- `md` `alpha-to-beta-migration-plan.md` — Phases, prerequisites, dual-run, verification, owner roles, rollback, and timeline.
- `md` `migration-cutover-checklist.md` — Operational checklist with evidence fields, stop conditions, and post-cutover monitoring.
- `txt_log` `dry-run-observations.log` — Timestamped synthetic rehearsal output with mitigated warnings.
- `code` `migration_probe.rs` — Rust probe checking compatibility, checkpoints, queue lag, and latency.
- `structured_text` `migration-waves.yaml` — Parseable cohort/wave plan with gates, dates, thresholds, and rollback ownership.
- `image` `alpha-to-beta-migration-map.png` — Seed ImageGen-produced, highly readable migration diagram with all anchors, dual-run path, validation gate, and rollback arrow.

## p01-secondary-01 — 10 files

Final folder: `workspace/people/p01-software-engineer/home/desktop/working`

- `md` `r7-today.md` — Plausible personal work plan with priorities, dependencies, and end-of-day notes.
- `md` `debugging-scratchpad.md` — Structured hypotheses, experiments, results, and next steps around the 184 ms release path.
- `txt_log` `local-terminal-capture.log` — Synthetic terminal session with safe local commands, failures, and successful rerun.
- `code` `latency_probe.ts` — Local percentile probe over synthetic fixtures.
- `code` `repro_release_gate.rs` — Minimal Rust reproduction of a duplicate-event race and its expected fix.
- `code` `cleanup_workspace.sh` — Strict safe cleanup for explicitly named local fixture directories; no broad or destructive targets.
- `structured_text` `working-session.json` — Parseable editor tabs, tasks, and synthetic local paths with no secrets.
- `pdf_text` `alpha-r7-debug-session.pdf` — Text-layer debugging write-up with hypothesis timeline, measurements, conclusion, and follow-up.
- `image` `latency-trace-annotated.png` — ImageGen-produced annotated trace-style visual showing a 184 ms path and dominant stage.
- `domain_binary` `alpha-r7-build-cache.sqlite` — Real SQLite cache with artifact keys, timestamps, statuses, and synthetic metadata.

## p01-secondary-02 — 8 files

Final folder: `workspace/people/p01-software-engineer/home/documents/reference`

- `md` `rust-async-patterns.md` — Internal reference on cancellation, timeouts, retries, backpressure, and test patterns.
- `md` `openapi-review-checklist.md` — Practical compatibility, security-boundary, example, and documentation checks.
- `md` `macos-dev-environment.md` — ja-JP macOS setup notes using only synthetic/local configuration examples.
- `pdf_text` `internal-api-style-guide.pdf` — Polished text-layer internal API guide with examples and anti-patterns.
- `pdf_text` `release-governance-reference.pdf` — Text-layer governance reference linking evidence, approvals, rollback, and ADR lifecycle.
- `docx` `api-usage-reference.docx` — Polished compact reference guide with request/response patterns, errors, and checklists.
- `xlsx` `engineering-reference-index.xlsx` — Multi-sheet reference catalog with formulas for freshness/status, typed dates, filters, and visual verification.
- `domain_binary` `reference-bundle.zip` — Real ZIP of synthetic reference snippets plus manifest and provenance notes.

## p01-secondary-03 — 8 files

Final folder: `workspace/people/p01-software-engineer/home/downloads/inbox`

- `txt_log` `download-triage-notes.txt` — Plausible inbox triage noting source class, relevance, and keep/archive decision without real URLs or identities.
- `code` `candidate-migration-script.ts` — Clearly labeled untrusted candidate script with safety review comments and no credential access.
- `ipynb` `candidate-latency-analysis.ipynb` — Valid notebook with deterministic synthetic samples and a note that results require review.
- `pdf_scan` `scanned-incident-note.pdf` — ImageGen-created synthetic photographed/scan-style incident note, wrapped as a real image-only PDF and visually inspected.
- `pdf_text` `vendor-migration-advisory.pdf` — Text-layer synthetic advisory with compatibility warnings and no real vendor identity.
- `xlsx` `download-inventory.xlsx` — Formula-driven triage inventory with type, source class, review status, risk, keep/archive result, and visual QA.
- `image` `downloaded-architecture-sketch.png` — ImageGen-produced rough architecture sketch, legible and clearly synthetic.
- `domain_binary` `r7-support-export.zip` — Real ZIP with synthetic logs, manifest, and redaction note; no secrets or user data.

## p01-secondary-04 — 6 files

Final folder: `workspace/people/p01-software-engineer/home/downloads/exports`

- `csv_tsv` `alpha-r7-build-metrics.csv` — Typed timestamp/build/duration/status/p95 rows with consistent totals.
- `csv_tsv` `review-export.csv` — Synthetic review IDs, stages, role labels, timestamps, and dispositions.
- `structured_text` `issue-export.json` — Parseable issue export with synthetic titles, labels, links-as-local-IDs, and dates.
- `html_eml` `pipeline-report.html` — Accessible standalone CI-style report over synthetic jobs and artifacts.
- `code` `normalize_export.ts` — Deterministic parser/normalizer for the fixed CSV and JSON export shapes.
- `domain_binary` `review-events.sqlite` — Real SQLite database mirroring synthetic review events with indexes and schema metadata.

## p01-secondary-05 — 6 files

Final folder: `workspace/people/p01-software-engineer/home/cloud/my-files`

- `md` `async-weekly-update.md` — Concise async update with outcomes, metrics, decisions, risks, and next work.
- `md` `personal-release-checklist.md` — Personal preflight and follow-up checklist aligned to the team plan.
- `code` `status_digest.ts` — TypeScript renderer turning synthetic task data into the weekly update.
- `code` `notes_indexer.rs` — Rust indexer over local Markdown metadata with deterministic tests.
- `docx` `async-handoff-playbook.docx` — Polished compact handoff guide with update template, escalation criteria, and examples.
- `pptx` `alpha-r7-async-status.pptx` — 4-6 slide asynchronous status update with outcomes, 184 ms metric, ADR-042, risks, and next steps.

## p01-secondary-06 — 4 files

Final folder: `workspace/people/p01-software-engineer/home/cloud/team-shared`

- `md` `team-release-board.md` — Shared release board snapshot with workstreams, role owners, status, dependencies, and dates.
- `txt_log` `shared-retro-notes.txt` — Blameless retrospective notes with what helped, what hurt, experiments, and role owners.
- `code` `release_window.ts` — Typed release-window rules for 2026-07-13 and validation tests.
- `code` `shared_contract_test.rs` — Team-shared Rust contract fixtures and assertions for ADR-042 transitions.

## p01-secondary-07 — 4 files

Final folder: `workspace/people/p01-software-engineer/home/mail/recent`

- `html_eml` `release-readiness-thread.eml` — Valid multipart synthetic mail thread among role aliases about R7 readiness, risks, and actions; no deliverable real addresses.
- `html_eml` `adr-042-approval-thread.eml` — Valid synthetic mail thread capturing objections, evidence, approval, and 2026-07-13 conditions.
- `txt_log` `reply-draft.txt` — Natural unsent reply summarizing remaining checks and asking role-based owners for evidence.
- `code` `release-notification.mjml` — Coherent MJML template for an internal R7 notification using synthetic placeholders only.

## p01-secondary-08 — 4 files

Final folder: `workspace/people/p01-software-engineer/home/archive/closed`

- `txt_log` `alpha-r6-closure-summary.txt` — Archived R6 closure summary that clearly predates R7 and records resolved follow-ups.
- `txt_log` `beta-spike-terminal.log` — Archived synthetic terminal log from an early Beta spike with final disposition.
- `code` `alpha-r6-cleanup.rs` — Archived cleanup utility scoped to synthetic R6 fixture records with tests.
- `code` `beta-spike-prototype.ts` — Archived experimental TypeScript prototype clearly marked superseded and non-production.
