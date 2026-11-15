# Product Alpha R7 Decision Durability — p01-full-002947

## Purpose

This repository reference documents `svc-release-coordinator` for synthetic Aster Loop Product Alpha R7. It gives focused engineering guidance for decision durability and contains no customer environment data.

## Operating facts

| Field | Value |
| --- | --- |
| Release | Product Alpha R7 (`ALPHA-R7`) |
| Release date | 2026-07-13 |
| Governing decision | ADR-042; asynchronous release gate retained |
| Observed path p95 | 184 ms against a 200 ms SLO ceiling |
| Evidence window | 2026-07-13 through 2026-07-20 |
| Repository reference | p01-src-002947 |

## Engineering guidance

The approval boundary is the Release Coordinator to Decision Store handoff. Event Bus delivery starts after the durable gate decision, and Observability confirmation closes the measured path. The controlled rollback drill occurred on 2026-07-17 and recovered in 11m42s (702 seconds). The 2026-07-20 review kept ADR-042.

## Verification notes

The evidence set has 84,200 observed requests: 84,149 successful and 51 unsuccessful. Gate sampling has 9,000 decisions: 8,883 approved and 117 non-approved. Synthetic environment labels are `env-dev`, `env-stg`, and `env-prod`.
