# Product Beta R4 replay safety review record 000759

Aster Loop株式会社 / BETA-R4 / ADR-B4-163 / architecture review 2026-08-17 / design p95 budget 225 ms

## Decision context

The Product Engineering review examined replay safety at the event ledger boundary. The design remains event-driven: commands are admitted before durable decisions, and projections consume accepted events asynchronously. This record is a synthetic planning artifact.

## Review outcome

- Decision: retain the proposed boundary with explicit ownership by the Platform Engineer role.
- Budget: 184 ms allocated to this path segment; end-to-end design target remains 225 ms.
- Evidence: trace label `beta-r4-000759` is reserved for synthetic staging validation.
- Follow-up: confirm the contract at the 2026-09-04 schema-freeze checkpoint.

## Constraints

No personal identifiers, access material, or production endpoints are represented. Failure states are classified as controlled retry, deferred projection, or rejected command; none is described as an outage.
