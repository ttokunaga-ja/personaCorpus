# Product Alpha R7 release record REL-A7-20260717-40

- Artifact: `p01-full-000524`
- Canonical source: `p01-src-000524`
- Organization: Aster Loop株式会社 (synthetic)
- Release date: 2026-07-13
- Status: released

## Operational note

This release-note entry records the observability confirmation follow-up for REL-A7-20260717-40. The asynchronous release gate remains governed by ADR-042. The observation window remains 2026-07-13 through 2026-07-20.

## Evidence

Observed end-to-end p95 was 184 ms against the 200 ms SLO ceiling. The release evidence includes 84,200 requests, 84,149 successful requests, and 51 unsuccessful requests; these are synthetic aggregate counts. Item 15 is a scoped note, not a production incident.
