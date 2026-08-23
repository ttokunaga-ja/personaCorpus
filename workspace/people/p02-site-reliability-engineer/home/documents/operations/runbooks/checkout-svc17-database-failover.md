# Checkout SVC-17 database failover

## Purpose and trigger
Use for confirmed Checkout primary database unavailability, write latency above 1.5 seconds, or an approved failover recommendation from the database controller.

## Impact signals
Checkout write errors rise while read health remains normal; connection-pool waits and transaction aborts increase. Do not infer a database fault from a payment-provider error alone.

## Safe prerequisites
Open an incident, name a database incident lead, verify a healthy replica in the same data class, and stop schema migrations. Confirm the target is `checkout-ledger` in `nstar-a` or `nstar-b`, never an unlabelled cluster.

## Diagnosis
1. Inspect replication and fencing state: `northstarctl db status --cluster checkout-ledger --region nstar-a`.
2. Confirm the candidate has replica lag below 15 seconds and no recovery errors.
3. Record the last committed transaction marker and active migration state.

## Mitigation
1. Set Checkout writes to controlled retry mode: `northstarctl service mode set --service checkout-svc17 --mode guarded-writes`.
2. Request controller-managed failover: `northstarctl db failover request --cluster checkout-ledger --candidate replica-a2 --reason incident`.
3. Wait for fencing confirmation; then refresh Checkout connections with `kubectl -n northstar-checkout rollout restart deploy/checkout-svc17`.

## Verification
Confirm one writable leader, monotonically advancing commit markers, checkout success above 99.7%, and no duplicate order identifiers in the reconciliation sample.

## Rollback and escalation
Never manually promote the former primary. If fencing is incomplete after 8 minutes, stop retries and page Database Critical. Roll back only the application connection refresh after controller approval.

## Evidence capture
Capture controller request ID, leader history, lag snapshots, guarded-write duration, and reconciliation result.

## Owners and review date
Owner: Database Platform with Checkout Reliability. Review: 2026-10-01.

## Cross-references
[Postgres replica lag](postgres-replica-lag.md), [error budget breach](checkout-svc17-error-budget-breach.md), [incident handoff](incident-commander-handoff.md).
