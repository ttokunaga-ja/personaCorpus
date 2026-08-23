# Beta troubleshooting
Aster Loop 株式会社 | Product Alpha R7 | release 2026-07-13 | p95 184 ms | ADR-042

## Projection lag
Signals: queue depth rises, checkpoint is stale, or lag exceeds five minutes. Diagnose producer/consumer offsets. Freeze cohort and drain synthetic queue; verify checkpoint and counts; Platform disables Beta if lag persists.
## Adapter mismatch
Signals: validation issue or unknown required v1 field. Compare input to matrix, block apply, preserve payload summary, then dry-run and unit-test the explicit mapping. Software Engineering retains Alpha reads on failure.
## Flag drift
Signals: observed routing differs from desired cohort. Compare flag snapshot with edge sample, freeze changes, set cohort to zero, and verify Alpha canary. Release Engineering runs rollback dry-run.
