# Beta R4 API contract brief: API-B4-4296

Product Beta R4 contract record for Aster Loop株式会社 (synthetic).
Schema freeze: 2026-09-04. The contract governs release-windows through svc-edge-api.

## Contract profile

- Contract ID: `API-B4-4296`
- Artifact: `p01-full-004296`; source: `p01-src-004296`
- Lifecycle state: `planned`
- Design p95 budget: 225 ms
- Contract test reconciliation: 1,240 total, 1,231 pass, 9 fail (99.27%)

## Compatibility rule

Consumers send an immutable request ID and a declared schema revision. Producers preserve
unknown optional fields, return a machine-readable review disposition, and publish events
only after the contract registry records the freeze decision.

## Review note

This synthetic record is owned by the API Steward role and is suitable for internal review.
