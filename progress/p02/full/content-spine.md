# p02 Full content spine

- Persona: `p02` / Site Reliability Engineer
- Contract: `p02-content-spine-v1`
- State: frozen for `p02-full-master-v1`
- Content language: en-US English; timestamps use UTC unless a file explicitly says otherwise
- Synthetic-use notice: every organization, service, incident, identifier, address, and measurement below is fictional
- Security boundary: no credentials, tokens, secrets, real PII/PHI, customer records, or private source material belong in any derived artifact

This spine is the shared factual contract for all twenty p02 scopes. It preserves the
accepted M1 anchors **Northstar Transit Cloud**, **Checkout SVC-17**,
**2026-07-13**, **error budget 2.4%**, and **INC-260713**. Individual files may
show only the facts relevant to their purpose, but they must not contradict this
contract.

## Stable entity registry

| Spine ID | Frozen fact |
| --- | --- |
| `org.ntc` | Northstar Transit Cloud (NTC), a fictional managed platform for transit-ticket checkout, rider identity, and operations telemetry. |
| `team.platform-sre` | Platform Reliability owns incident command, SLO governance, production readiness, and cross-service follow-up. |
| `team.checkout` | Checkout Operations owns Checkout SVC-17 and its payment workflow. |
| `team.identity` | Identity Operations owns Identity SVC-23 and authentication/session continuity. |
| `team.observability` | Observability Operations owns Telemetry Pipeline SVC-31, alert delivery, dashboards, and export integrity. |
| `team.change` | Change Enablement records approvals, deployment evidence, rollback decisions, and maintenance windows. |
| `env.prod` | Production is split across the fictional `aurora-east` and `borealis-west` regions; `harbor-test` is the synthetic staging environment. |
| `svc.checkout.svc17` | Checkout SVC-17 is a tier-1 checkout orchestrator. Its synthetic service catalog key is `ntc/checkout/svc-17`. |
| `svc.identity.svc23` | Identity SVC-23 issues and validates rider sessions. Its synthetic service catalog key is `ntc/identity/svc-23`. |
| `svc.edge.svc08` | Edge SVC-08 terminates ingress and applies traffic-shift policy for the two production regions. |
| `svc.telemetry.svc31` | Telemetry Pipeline SVC-31 transports metrics, traces, alert events, and bounded log exports. |
| `dep.orders.pg04` | Orders PostgreSQL cluster PG-04 is the Checkout SVC-17 system of record. |
| `dep.payments.q07` | Payment work queue Q-07 buffers authorization jobs and is the principal saturation signal for INC-260713. |
| `dep.checkout.c12` | Checkout cache C-12 stores short-lived routing and cart metadata; it contains no corpus customer data. |
| `dep.gateway.g03` | Payment Gateway G-03 is a fictional sandbox dependency represented only by synthetic status and latency facts. |

Use role aliases rather than personal names. Permitted synthetic contacts are
`incident-commander@northstar.invalid`, `checkout-oncall@northstar.invalid`,
`identity-oncall@northstar.invalid`, and `observability-oncall@northstar.invalid`.
The `.invalid` domain is deliberate and must never be replaced with a deliverable
domain.

## Time and event registry

| Spine ID | Frozen fact |
| --- | --- |
| `date.2026-07-13` | Primary incident/on-call date. The reference shift is 08:00–20:00 UTC. |
| `date.2026-07-14` | Initial evidence review and action-owner confirmation. |
| `date.2026-07-15` | Guardrail hotfix and first follow-up change window. |
| `date.2026-07-20` | Seven-day corrective-action and SLO review. |
| `period.2026-q3` | Reliability and capacity planning period from 2026-07-01 through 2026-09-30. |
| `inc.inc-260713` | S2 Checkout SVC-17 queue-saturation incident, declared 09:47 UTC and resolved 10:42 UTC on 2026-07-13. Customer-impact window: 09:47–10:31 UTC. |
| `inc.inc-260628` | S3 Identity SVC-23 cache-churn incident used only for recurrence comparison. |
| `inc.inc-260519` | S3 Edge SVC-08 DNS-convergence incident used only for recurrence comparison. |
| `inc.inc-260407` | S3 Telemetry Pipeline SVC-31 backpressure incident used only for recurrence comparison. |
| `change.chg-260713-042` | Approved Checkout SVC-17 deployment that began 09:18 UTC. Rollback decision recorded at 10:02 UTC. |
| `deploy.dep-260713-017` | Deployment execution ID associated with CHG-260713-042. |
| `problem.prb-260713-009` | Problem record for the missing queue-pressure guardrail and undersized consumer concurrency. |
| `action.act-260713-001` | Restore the Q-07 depth guardrail; completed 2026-07-15. |
| `action.act-260713-002` | Add a two-window burn-rate alert; completed 2026-07-20. |
| `action.act-260713-003` | Validate regional queue-drain capacity; due 2026-07-31. |
| `action.act-260713-004` | Add change-template evidence for rollback readiness; due 2026-08-07. |

### INC-260713 canonical timeline

| UTC | Event |
| --- | --- |
| 09:18 | DEP-260713-017 begins in `aurora-east`. |
| 09:42 | Q-07 queue depth crosses the 12,000 warning threshold. |
| 09:47 | S2 declared after checkout availability and latency alerts agree. |
| 09:54 | New authorizations are rate-limited; Identity SVC-23 remains available. |
| 10:02 | Incident command approves rollback of CHG-260713-042. |
| 10:11 | Consumer concurrency is raised under the documented emergency limit. |
| 10:24 | Q-07 falls below 8,000 and the short-window burn rate begins recovering. |
| 10:31 | Customer-impact window closes. |
| 10:42 | Incident resolved; enhanced monitoring remains active for 24 hours. |

The causal chain is fixed: a deployment removed an effective queue-pressure
guardrail; demand then exceeded consumer concurrency; retry amplification increased
Q-07 depth; Checkout SVC-17 latency and request failures rose. Identity SVC-23 was
degraded only by increased validation load and was not the initiating cause.

## SLO, error-budget, and measurement registry

| Spine ID | Frozen measure |
| --- | --- |
| `slo.checkout.availability` | Checkout SVC-17 request-based availability target: 99.95% over a rolling 30-day window. |
| `slo.checkout.latency` | Checkout SVC-17 latency target: 99.0% of eligible requests below 400 ms over 30 days. |
| `slo.identity.availability` | Identity SVC-23 request-based availability target: 99.99% over 30 days. |
| `slo.telemetry.delivery` | Telemetry Pipeline SVC-31 delivery target: 99.9% of accepted events exported within 10 minutes. |
| `budget.checkout.2026-07` | At the 2026-07-13 noon review, Checkout SVC-17 had **2.4% of its request-error allowance remaining**. This percentage is not wall-clock downtime. |
| `measure.checkout.baseline-rps` | Pre-incident eligible checkout rate: 1,850 requests/s. |
| `measure.checkout.peak-rps` | Incident peak eligible checkout rate: 2,420 requests/s. |
| `measure.checkout.availability-low` | Lowest five-minute Checkout availability during INC-260713: 92.14%. |
| `measure.checkout.p95-high` | Highest five-minute Checkout p95 latency: 1.84 s. |
| `measure.queue.depth-high` | Q-07 maximum observed depth: 38,420 jobs. Warning threshold: 12,000; critical threshold: 24,000. |
| `measure.queue.drain-rate` | Sustained recovery drain rate after mitigation: 1,460 jobs/min. |
| `measure.burn-rate-short` | Peak one-hour Checkout availability burn rate: 14.6×. |
| `measure.burn-rate-long` | Peak six-hour Checkout availability burn rate: 5.2×. |
| `measure.identity.p95-high` | Identity SVC-23 p95 validation latency peak: 286 ms, below its 350 ms paging threshold. |
| `measure.capacity.headroom` | Post-mitigation Checkout consumer headroom at forecast peak: 31%. |
| `measure.log-export-gap` | Telemetry export gap during the incident: 7 minutes, with reconciliation complete by 11:06 UTC. |

When a file rounds a value for presentation, it must retain the same direction and
unit. CSV/TSV or structured facts retain the exact values above; workbooks,
searchable PDFs, DOCX reviews, and presentations cite those base artifacts through
ledger dependencies.

## Operational terminology

| Spine ID | Frozen usage |
| --- | --- |
| `term.s2` | Material service degradation requiring coordinated incident command; synthetic severity only. |
| `term.s3` | Limited or short-lived degradation handled by one service team; synthetic severity only. |
| `term.burn-rate` | Consumed error allowance divided by the rate that would exactly exhaust the allowance in the SLO window. |
| `term.impact-window` | Period of confirmed SLI/customer-impact breach, distinct from declaration-to-resolution time. |
| `term.mitigation` | Action that reduces impact without necessarily removing the root cause. |
| `term.rollback-ready` | Change has a tested reversal path, evidence location, owner role, and stop condition. |
| `term.evidence-chain` | Stable IDs connecting raw facts/logs to derived Office, PDF, or image artifacts. |
| `term.no-kio-claim` | Corpus production and QA do not establish Kio indexing, history, chunks, search quality, or performance. |

## Cross-format relationship registry

| Spine ID | Required relationship |
| --- | --- |
| `rel.inc260713.timeline` | Structured/CSV/log timeline facts precede the INC-260713 workbook, postmortem DOCX, searchable PDF review, and operations presentation. |
| `rel.slo.2026-q3` | Exact SLO and burn-rate facts precede dashboard workbooks, policy DOCX, searchable PDF reports, and planning slides. |
| `rel.capacity.2026-q3` | Demand, concurrency, queue, and headroom facts precede capacity workbooks/reports/slides. |
| `rel.change.260713` | CHG-260713-042 and DEP-260713-017 structured evidence precedes approval summaries, rollback reviews, and briefings. |
| `rel.oncall.2026-07-13` | Alert, log, handoff, and action IDs remain consistent across checkout, identity, mail, desktop, and team-shared scopes. |
| `rel.topology.260713` | Structured service/region facts precede topology and burn-rate images. A scan PDF, if canonical inventory ever supplied one, would additionally depend on its ImageGen source. |
| `rel.recurrence.2026-q2` | INC-260628, INC-260519, and INC-260407 comparisons use the same severity and action terminology. |
| `rel.archive.closed` | Closed records retain incident/change IDs and final disposition but never introduce real packet, customer, or credential data. |

The accepted p02 canonical inventory contains zero `pdf_scan` sources. The master
therefore creates no scan-PDF assignment and does not manufacture an exception to
the canonical family mix.

## Scope reuse map

| Rust scope ID | Primary subject IDs | Relationship ID |
| --- | --- | --- |
| `p02-primary-01` | `svc.checkout.svc17`, `svc.identity.svc23`, `term.mitigation` | `rel.inc260713.timeline` |
| `p02-primary-02` | `inc.inc-260713`, `problem.prb-260713-009`, `action.act-260713-001` | `rel.recurrence.2026-q2` |
| `p02-primary-03` | `svc.checkout.svc17`, `svc.identity.svc23`, `env.prod` | `rel.topology.260713` |
| `p02-primary-04` | `change.chg-260713-042`, `deploy.dep-260713-017`, `env.prod` | `rel.change.260713` |
| `p02-primary-05` | `svc.checkout.svc17`, `inc.inc-260713`, `dep.payments.q07` | `rel.oncall.2026-07-13` |
| `p02-primary-06` | `svc.identity.svc23`, `inc.inc-260713`, `measure.identity.p95-high` | `rel.oncall.2026-07-13` |
| `p02-primary-07` | `slo.checkout.availability`, `slo.checkout.latency`, `measure.burn-rate-short` | `rel.slo.2026-q3` |
| `p02-primary-08` | `budget.checkout.2026-07`, `measure.queue.depth-high`, `slo.telemetry.delivery` | `rel.slo.2026-q3` |
| `p02-primary-09` | `svc.telemetry.svc31`, `measure.log-export-gap`, `inc.inc-260713` | `rel.inc260713.timeline` |
| `p02-primary-10` | `change.chg-260713-042`, `deploy.dep-260713-017`, `term.rollback-ready` | `rel.change.260713` |
| `p02-primary-11` | `measure.capacity.headroom`, `measure.checkout.peak-rps`, `measure.queue.drain-rate` | `rel.capacity.2026-q3` |
| `p02-primary-12` | `period.2026-q3`, `inc.inc-260713`, `action.act-260713-004` | `rel.slo.2026-q3` |
| `p02-secondary-01` | `inc.inc-260713`, `term.evidence-chain`, `team.platform-sre` | `rel.oncall.2026-07-13` |
| `p02-secondary-02` | `slo.checkout.availability`, `term.burn-rate`, `term.rollback-ready` | `rel.slo.2026-q3` |
| `p02-secondary-03` | `inc.inc-260713`, `dep.payments.q07`, `term.evidence-chain` | `rel.archive.closed` |
| `p02-secondary-04` | `measure.burn-rate-short`, `measure.queue.depth-high`, `env.prod` | `rel.topology.260713` |
| `p02-secondary-05` | `team.platform-sre`, `budget.checkout.2026-07`, `period.2026-q3` | `rel.capacity.2026-q3` |
| `p02-secondary-06` | `team.checkout`, `team.identity`, `team.observability` | `rel.oncall.2026-07-13` |
| `p02-secondary-07` | `inc.inc-260713`, `change.chg-260713-042`, `date.2026-07-13` | `rel.oncall.2026-07-13` |
| `p02-secondary-08` | `inc.inc-260628`, `inc.inc-260519`, `inc.inc-260407` | `rel.archive.closed` |

## Allocation and dependency contract

1. A scope's foundation rows are canonical `structured_text`, `csv_tsv`, `code`,
   `txt_log`, and `domain_binary` sources. They are produced in bounded batches
   before derived visual/Office rows.
2. Every canonical `xlsx`, `docx`, `pdf_text`, and `pptx` row depends on a
   scope-local foundation artifact, preferring CSV/TSV and then structured facts.
3. Every `image` row is routed to ImageGen and depends on scope-local structured
   or tabular facts. A canonical `pdf_scan` row would depend on both a source image
   and foundation facts; p02 presently has none.
4. Ordinary Markdown, HTML, and EML rows follow the visual/Office work. In a scope
   with fewer than 100 such rows, they may share that scope's visual QA batch while
   preserving dependency order inside the batch.
5. Dependencies never cross Rust scope boundaries. Batch sequence numbers increase
   from foundation to visual to operational completion, and every out-of-batch
   dependency points backward.
6. This contract is allocation evidence only. `term.no-kio-claim` applies to every
   row and no row represents a completed physical artifact.
