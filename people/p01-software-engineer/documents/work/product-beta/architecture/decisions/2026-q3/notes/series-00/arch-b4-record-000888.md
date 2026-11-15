# Product Beta R4 idempotency window review record 000888

Aster Loop株式会社 / BETA-R4 / ADR-B4-093 / architecture review 2026-08-17 / design p95 budget 225 ms

## Decision context

The Product Engineering review examined idempotency window at the edge intake boundary. The design remains event-driven: commands are admitted before durable decisions, and projections consume accepted events asynchronously. This record is a synthetic planning artifact.

## Review outcome

- Decision: retain the proposed boundary with explicit ownership by the Platform Engineer role.
- Budget: 148 ms allocated to this path segment; end-to-end design target remains 225 ms.
- Evidence: trace label `beta-r4-000888` is reserved for synthetic staging validation.
- Follow-up: confirm the contract at the 2026-09-04 schema-freeze checkpoint.

## Constraints

No personal identifiers, access material, or production endpoints are represented. Failure states are classified as controlled retry, deferred projection, or rejected command; none is described as an outage.
