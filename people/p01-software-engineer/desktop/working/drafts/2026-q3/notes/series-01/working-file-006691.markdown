# Desktop working note: ALW-8691

**State:** revised — ephemeral working copy, not an accepted record.

This Aster Loop Product Engineering note examines `svc-release-coordinator` for Product Alpha R7.
It references ADR-125 and preserves the asynchronous release-gate boundary between
Release Coordinator and Decision Store. The observed working-path value is 194 ms;
the accepted Alpha reference remains p95 184 ms.

## Handling

- Review date: 2026-09-23 (Asia/Tokyo)
- Scope: synthetic desktop work only; no customer or production data
- Outcome: retain this draft until a reviewed team-shared record is created
