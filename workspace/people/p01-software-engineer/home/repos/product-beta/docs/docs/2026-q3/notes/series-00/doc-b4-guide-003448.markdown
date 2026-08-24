# Product Beta R4: runbook rehearsal

**Record:** BETA-DOC-003448  
**Status:** reviewed  
**Owner:** Product Engineering, Aster Loop株式会社 (synthetic)

This repository note describes the runbook rehearsal for svc-observability. It supports the Product Beta R4 architecture review on 2026-08-17 and the API schema freeze on 2026-09-04. The design p95 budget is 225 ms; this is a target rather than an observed production result.

## Operating contract

- Preserve ordered events after the approval boundary and record the release decision before delivery.
- Keep compatibility evidence for 1,240 contract tests: 1,231 passing and 9 failing (99.27% rounded).
- Use synthetic role aliases and .invalid mail domains in examples.

## Verification

Review the typed contract, execute the documented check in env-stg, and attach BETA-DOC-003448 to the Beta R4 release evidence.
