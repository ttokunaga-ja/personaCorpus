# API-B4-8979 — Product Beta R4 API contract

**Schema freeze:** 2026-09-04 · **Status:** released · **Owner:** API Steward

## Scope

This synthetic Aster Loop株式会社 contract defines the `evidence-snapshot` resource served by
`svc-decision-store`. Its record is p01-full-008979 and its canonical source is p01-src-008979.

## Interoperability expectation

A caller supplies an immutable request identifier and declared schema revision. The registry
preserves optional fields it does not recognize, records the review disposition, and allows
event publication only after the frozen schema is registered.

## Quality reconciliation

Product Beta R4 uses 1,240 contract tests: 1,231 pass and 9 fail, yielding 99.27% when
rounded to two decimals. The 225 ms p95 value is a design budget, not an observed incident.
