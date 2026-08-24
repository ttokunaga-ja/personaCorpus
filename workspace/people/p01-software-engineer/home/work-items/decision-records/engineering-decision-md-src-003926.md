# ADR-146: evidence retention window

- Record: p01-src-003926
- Product: Product Alpha R7
- Status: reviewed
- Decision date: 2026-07-20
- Owner role: API Steward

## Context

Aster Loop株式会社 maintains an asynchronous release gate. This decision records how svc-decision-store will apply a evidence retention window while preserving the ADR-042 approval boundary. The Alpha evidence window measured p95 184 ms against a 200 ms target; Beta uses a 225 ms design budget.

## Decision

Adopt the evidence retention window for svc-decision-store. The implementation emits a decision record with the stable identifier ADR-146, a lifecycle status of reviewed, and an auditable rationale. No individual mailbox, credential, or customer identifier is stored in this record.

## Consequences

Reviewers compare release-gate evidence, compatibility results, and rollback-drill observations before changing the lifecycle state. The choice keeps Product Alpha R7 and Product Beta R4 terminology aligned across engineering records.
