# ADR-079: release evidence retention window

| Field | Value |
| --- | --- |
| Record | p01-src-009829 |
| Product | Product Alpha R7 |
| Lifecycle state | closed |
| Review date | 2026-07-20 |
| Accountable role | Platform Engineer |

## Context

Aster Loop株式会社 uses decision records to keep the asynchronous release gate reviewable. This record concerns svc-release-coordinator; it preserves the ADR-042 approval boundary from Release Coordinator to Decision Store.

## Decision

Adopt a release evidence retention window. The lifecycle evidence names ADR-079, records the state as closed, and uses only role-based ownership. Product Alpha evidence remains 184 ms p95 against a 200 ms ceiling; Product Beta uses a 225 ms design budget.

## Review signal

The next reviewer compares gate evidence, compatibility observations, and the controlled rollback drill before changing this decision's state. The record is synthetic and contains neither individual identity data nor operational access material.
