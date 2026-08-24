# Product Alpha R7 repository evidence: p01-full-007744

This synthetic Aster Loop engineering note covers `svc-observability`. Product Alpha R7 (`ALPHA-R7`) released on 2026-07-13 under ADR-042, which retains the asynchronous release gate.

## Evidence

The observed path is 184 ms p95 against the 200 ms ceiling. The approval boundary joins Release Coordinator to Decision Store; Event Bus delivery follows the durable decision and Observability closes the path. The 2026-07-20 review kept ADR-042.

## Reference

Source `p01-src-007744` records a repository-local operational explanation. The 2026-07-17 controlled rollback drill recovered in 702 seconds; it is not an incident record.
