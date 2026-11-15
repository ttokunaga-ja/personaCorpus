# Event contract boundaries
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

## Ownership and versioning
Release Engineering owns release.gate.requested.v1 and release.gate.decided.v1. Additive fields are compatible; semantic changes need a major suffix.
## Idempotency
Key is release_id plus caller_scope, retained 24 hours. Duplicates return original ticket and never republish.
## Retry and failure
Bounded exponential retry retains event id. Poison payloads go to release.gate.dead-letter.v1. Timeout creates timed_out, never approval.
## Data boundary
Synthetic release metadata only. No customer data, credentials, tokens, or PII.
