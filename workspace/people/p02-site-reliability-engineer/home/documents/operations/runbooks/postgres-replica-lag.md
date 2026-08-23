# Postgres replica lag

## Purpose and trigger
Use when Checkout or Identity replica lag exceeds 60 seconds, replay is stalled, or read traffic is serving materially stale data.

## Impact signals
Read-after-write inconsistencies, failover ineligibility, growing WAL retention, and elevated primary I/O.

## Safe prerequisites
Identify the exact cluster and replica. Stop nonessential analytic reads after database lead approval; do not promote a lagging replica.

## Diagnosis
1. Run `northstarctl db replicas --cluster checkout-ledger`.
2. Compare receive lag, replay lag, storage latency, and long-running transactions.
3. Check replication-slot retention and recent index maintenance.

## Mitigation
1. Move reporting workload off the primary via `northstarctl db workload throttle --class analytics --percent 50`.
2. Cancel only database-lead-approved synthetic maintenance jobs.
3. If a replica is irrecoverably behind, request controller rebuild rather than copying data manually.

## Verification
Replay lag stays below 15 seconds for 20 minutes and WAL retention trends downward without increasing write latency.

## Rollback and escalation
Restore reporting throttle gradually after headroom returns. Escalate to Database Critical if primary disk use exceeds 80% or all replicas exceed 60 seconds.

## Evidence capture
Save lag series, transaction identifiers, throttle decision, and controller rebuild request.

## Owners and review date
Owner: Database Platform. Review: 2026-09-30.

## Cross-references
[Checkout database failover](checkout-svc17-database-failover.md), [Identity session-store failover](identity-session-store-failover.md), [node pressure](kubernetes-node-pressure.md).
