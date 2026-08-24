# API-A7-0504: release-gates register contract

**Status:** reviewed  
**Release:** Product Alpha R7 (`ALPHA-R7`)  
**Review date:** 2026-07-20

## Intent

This synthetic Aster Loop API contract records the `register` interaction for
`release-gates`. The request enters through `svc-edge-api` and is catalogued by
`svc-contract-registry`; it contains no customer or production data.

## Compatibility

Changes follow the additive-before-removal policy. The API target is p95 200 ms;
the 184 ms reference is the accepted Alpha release-gate observation, not a
measurement of this endpoint.

## Review outcome

The API Steward marked API-A7-0504 reviewed with an explicit `reviewed` lifecycle
state. ADR-042 remains the asynchronous release-gate decision.
