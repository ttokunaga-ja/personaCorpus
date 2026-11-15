# Kubernetes pod crashloop

## Purpose and trigger
Use when a production Checkout, Identity, or platform pod enters `CrashLoopBackOff` more than three times in ten minutes.

## Impact signals
Replica availability drops, restart counters rise, readiness fails, or a rollout stalls. Distinguish a process crash from node-level eviction.

## Safe prerequisites
Freeze the affected rollout and retain the current ReplicaSet. Confirm whether enough healthy replicas satisfy the disruption budget before restart actions.

## Diagnosis
1. Inspect `kubectl -n northstar-checkout describe pod checkout-svc17-example`.
2. Review previous container output with `kubectl -n northstar-checkout logs --previous pod/checkout-svc17-example`.
3. Check exit reason, config revision, secret reference health without reading secret contents, and dependency readiness.

## Mitigation
1. If correlated to the latest image, undo the rollout: `kubectl -n northstar-checkout rollout undo deploy/checkout-svc17`.
2. If memory-limited, raise no limits during incident; scale healthy replicas only within the approved HPA maximum.
3. For a configuration regression, restore the last approved ConfigMap revision using the deployment controller.

## Verification
New pods remain ready with zero restarts for 15 minutes; service error and latency remain within SLO.

## Rollback and escalation
Reapply the previous revision only after change review. Escalate to the service owner for repeated crashes with no deploy correlation, or Compute Platform for runtime failures across workloads.

## Evidence capture
Save pod UID, exit code, image digest, revision history, and redacted previous-log excerpt.

## Owners and review date
Owner: Kubernetes Operations. Review: 2026-10-05.

## Cross-references
[Kubernetes node pressure](kubernetes-node-pressure.md), [service mesh certificate rotation](service-mesh-certificate-rotation.md), [Checkout error budget breach](checkout-svc17-error-budget-breach.md).
