# ADR-B4-144: event envelope version boundary

| Field | Value |
| --- | --- |
| Record | p01-src-007506 |
| Product | Product Beta R4 |
| Lifecycle state | reviewed |
| Review date | 2026-08-17 |
| Accountable role | Decision Owner |

## Context

Aster Loop株式会社 uses decision records to keep the asynchronous release gate reviewable. This record concerns svc-edge-api; it preserves the ADR-042 approval boundary from Release Coordinator to Decision Store.

## Decision

Adopt a event envelope version boundary. The lifecycle evidence names ADR-B4-144, records the state as reviewed, and uses only role-based ownership. Product Alpha evidence remains 184 ms p95 against a 200 ms ceiling; Product Beta uses a 225 ms design budget.

## Review signal

The next reviewer compares gate evidence, compatibility observations, and the controlled rollback drill before changing this decision's state. The record is synthetic and contains neither individual identity data nor operational access material.
