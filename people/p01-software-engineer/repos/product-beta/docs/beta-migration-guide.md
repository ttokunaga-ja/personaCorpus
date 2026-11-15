# Beta preview migration guide
Aster Loop 株式会社 | Product Alpha R7 | release 2026-07-13 | p95 184 ms | ADR-042

## Scope and maturity
Beta is a reversible preview projection, not a replacement system. Software Engineering owns the adapter; Platform owns projection health; Release Engineering owns cohort gates.
## Phase 0 - prerequisites
Confirm Alpha R7 v1 capture, adapter build, a clean dry run, queue depth below 200, and baseline p95 at or below 184 ms. Save verifier output and dry-run log as evidence.
## Phase 1 - compatibility decision
The adapter accepts v1 API/event fields and creates an additive Beta projection. Missing optional beta_label becomes a synthetic default during dry run; no source record is mutated.
## Phase 2 - dry run
Transform configuration with dryRun=true, replay the fixed fixture, and run verify_beta_state. Gate promotion on matching record counts, checkpoint parity, queue lag under five minutes, and p95 at or below 184 ms.
## Phase 3 - cohort rollout
Enable 5%, observe 30 minutes; 25%, observe one hour; then 100%, observe one release window. Record cohort, lag, latency, adapter errors, and verifier result at entry and exit.
## Compatibility
Keep v1 events and forward-readable storage. Dry run before cohort rollout.
## Rollback
Freeze cohort, disable Beta projection, retain Alpha R7 reads, drain the synthetic queue, compare counts, and re-run the Alpha canary. Do not rewrite decisions or delete evidence.
## Post-checks
Retire the flag only after two clean windows, zero projection mismatches, and a role review.
