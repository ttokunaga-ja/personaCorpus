# Kubernetes node pressure

## Purpose and trigger
Use when a production node reports sustained memory, disk, or PID pressure and eviction risk affects Checkout or Identity workloads.

## Impact signals
Node conditions become `True`, pods are evicted, image pulls fail, or scheduling cannot place protected workloads.

## Safe prerequisites
Identify node pool and disruption budget. Confirm a replacement node can join before draining; do not drain a node hosting the last healthy replica.

## Diagnosis
1. Run `kubectl get nodes -l northstar.io/pool=production` and inspect the affected node condition.
2. Check `kubectl -n northstar-checkout get pods -o wide` for workload concentration.
3. Correlate pressure with log volume, ephemeral storage, or a recently deployed workload.

## Mitigation
1. Cordon the node: `kubectl cordon node/nstar-prod-a-07`.
2. Scale the node pool through the controller: `northstarctl capacity nodes add --pool production-a --count 1`.
3. Drain only evictable workloads using the approved controller after capacity is ready.

## Verification
Protected pods are ready elsewhere, pressure clears, and no PDB violations or service error increase occur for 15 minutes.

## Rollback and escalation
Uncordon only after the cause is resolved. Escalate to Compute Platform for repeated pressure or unsuccessful replacement registration.

## Evidence capture
Record node conditions, pod placement, capacity action ID, and eviction events.

## Owners and review date
Owner: Compute Platform. Review: 2026-09-29.

## Cross-references
[Kubernetes pod crashloop](kubernetes-pod-crashloop.md), [Checkout cache degradation](checkout-svc17-cache-degradation.md), [Postgres replica lag](postgres-replica-lag.md).
