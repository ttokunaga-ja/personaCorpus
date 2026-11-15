# Product Beta R4 event ordering rule

**Record:** BETA-REPO-009128  
**Service:** svc-decision-store  
**State:** approved

This repository document records the Product Beta R4 event ordering rule for Aster Loop株式会社 (synthetic). It aligns with the 2026-08-17 architecture review, the 2026-09-04 schema freeze, and the 2026-09-28 target release.

## Contract

- Design p95 budget: **225 ms**; it is a target, not an observed production value.
- Contract test baseline: **1,240 total**, **1,231 passing**, **9 failing** (**99.27%** rounded).
- Persist a gate decision before event delivery; use env-stg for rehearsal evidence.

## Review steps

Confirm schema compatibility, retain the role-owned evidence, and link BETA-REPO-009128 in the release packet.
