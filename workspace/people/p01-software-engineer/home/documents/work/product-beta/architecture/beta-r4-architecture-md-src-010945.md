# Product Beta R4 architecture evidence 010945

Aster Loop株式会社 | BETA-R4 | ADR-B4-001 | 2026-08-17 architecture review | 225 ms design p95 budget | p01-src-010945

## Focus

This synthetic review records **projection lag accounting** at the command dispatcher. The architecture persists a decision before downstream event distribution and keeps projection work asynchronous.

## Review conclusion

The 2026-08-17 review retained the boundary and assigned role-level stewardship to the Platform Engineer. The local allocation is 193 ms within the 225 ms design target.

## Follow-up

- State: reviewed
- Checkpoint: 2026-09-04 schema freeze
- Evidence label: beta-r4-b003-010945
- Scope: synthetic staging material only
