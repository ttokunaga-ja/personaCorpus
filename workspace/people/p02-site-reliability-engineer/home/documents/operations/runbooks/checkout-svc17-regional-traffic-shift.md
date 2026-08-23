# Checkout SVC-17 regional traffic shift

## Purpose and trigger
Use to reduce exposure to a regional Checkout fault when one region has sustained 5xx above 2% and the peer has at least 35% headroom.

## Impact signals
Regional error asymmetry, ingress saturation, or an unavailable zonal dependency. Confirm global DNS and edge health before changing weights.

## Safe prerequisites
Incident commander approval, capacity lead acknowledgement, and a recorded starting split. Ensure the destination region is not in maintenance and Checkout database topology supports the shift.

## Diagnosis
1. Check regional capacity: `northstarctl capacity show --service checkout-svc17`.
2. Validate destination p95 below 650 ms and payment worker utilization below 70%.
3. Identify whether errors persist at ingress or begin downstream.

## Mitigation
1. Apply a 15% incremental weight: `northstarctl traffic shift --service checkout-svc17 --from nstar-a --to nstar-b --percent 15`.
2. Observe for 10 minutes; repeat once only if destination remains within thresholds.
3. Keep sessions sticky; do not redirect in-flight payment callbacks.

## Verification
Source 5xx declines, destination error rate stays below 0.5%, and both queue ages remain below five minutes for 20 minutes.

## Rollback and escalation
Reverse the most recent increment if destination p95 exceeds 900 ms or error rate exceeds 1%. Escalate to Regional Network when edge health is degraded in both regions.

## Evidence capture
Store traffic weights, capacity screenshots, approval time, and before/after SLO readings.

## Owners and review date
Owner: Traffic Engineering. Review: 2026-10-13.

## Cross-references
[Edge ingress 5xx spike](edge-ingress-5xx-spike.md), [regional disaster recovery](regional-disaster-recovery.md), [Checkout error budget breach](checkout-svc17-error-budget-breach.md).
