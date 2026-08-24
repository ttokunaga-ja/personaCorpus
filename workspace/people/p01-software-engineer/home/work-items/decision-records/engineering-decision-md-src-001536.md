# ADR-B4-144: bounded retry budget

- Record: p01-src-001536
- Product: Product Beta R4
- Status: reviewed
- Decision date: 2026-08-17
- Owner role: Decision Owner

## Context

Aster Loop株式会社 maintains an asynchronous release gate. This decision records how svc-edge-api will apply a bounded retry budget while preserving the ADR-042 approval boundary. The Alpha evidence window measured p95 184 ms against a 200 ms target; Beta uses a 225 ms design budget.

## Decision

Adopt the bounded retry budget for svc-edge-api. The implementation emits a decision record with the stable identifier ADR-B4-144, a lifecycle status of reviewed, and an auditable rationale. No individual mailbox, credential, or customer identifier is stored in this record.

## Consequences

Reviewers compare release-gate evidence, compatibility results, and rollback-drill observations before changing the lifecycle state. The choice keeps Product Alpha R7 and Product Beta R4 terminology aligned across engineering records.
