# ADR-125: rollback rehearsal cadence

- Record: p01-src-001517
- Product: Product Alpha R7
- Status: approved
- Decision date: 2026-07-20
- Owner role: Migration Lead

## Context

Aster Loop株式会社 maintains an asynchronous release gate. This decision records how svc-contract-registry will apply a rollback rehearsal cadence while preserving the ADR-042 approval boundary. The Alpha evidence window measured p95 184 ms against a 200 ms target; Beta uses a 225 ms design budget.

## Decision

Adopt the rollback rehearsal cadence for svc-contract-registry. The implementation emits a decision record with the stable identifier ADR-125, a lifecycle status of approved, and an auditable rationale. No individual mailbox, credential, or customer identifier is stored in this record.

## Consequences

Reviewers compare release-gate evidence, compatibility results, and rollback-drill observations before changing the lifecycle state. The choice keeps Product Alpha R7 and Product Beta R4 terminology aligned across engineering records.
