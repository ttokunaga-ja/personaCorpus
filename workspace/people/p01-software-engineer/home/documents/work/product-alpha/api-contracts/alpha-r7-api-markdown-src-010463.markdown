# API-A7-10463 — evidence-snapshots reconcile

**Product:** Product Alpha R7 (`ALPHA-R7`)  
**Lifecycle state:** reviewed  
**Contract review:** 2026-07-20

## Purpose

This synthetic Aster Loop API note defines the `reconcile` interaction for
`evidence-snapshots`. `svc-edge-api` accepts the request and `svc-contract-registry`
records the reviewed schema reference.

## Compatibility and timing

The compatibility policy is additive-before-removal. The endpoint target is
p95 200 ms. The 184 ms Alpha release-gate value is an accepted shared reference,
not a measurement of this endpoint. ADR-042 continues to govern the asynchronous
release gate.

## Verification

Contract API-A7-10463 is a synthetic engineering record; it contains no customer,
employee, credential, or production payload data.
