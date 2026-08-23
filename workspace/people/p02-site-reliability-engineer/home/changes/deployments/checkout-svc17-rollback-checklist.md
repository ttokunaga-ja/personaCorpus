# Checkout SVC-17 rollback checklist

Use this checklist for a failed mitigation or release affecting the Checkout SVC-17 production deployment. The objective is to return to the last known-good replica-set revision without widening impact.

## Entry criteria

- The incident commander has declared rollback, or the change owner has met a documented rollback threshold.
- The current deployment revision and observed error-rate window are recorded in the incident timeline.
- Synthetic checkout probes and payment-adapter health are available for post-rollback comparison.

## Procedure

1. Pause the rollout and confirm no second change is targeting Checkout SVC-17.
2. Select the prior healthy revision recorded before the change window.
3. Restore the revision with a rolling update limited to one unavailable replica.
4. Watch readiness, checkout availability, adapter retry ratio, and p95 latency for fifteen minutes.
5. Record the observed state, decision time, and any remaining customer-impact symptoms in the deployment log.

## Exit criteria

Rollback is complete when every replica is ready, synthetic checkout probes are healthy, and the incident commander accepts the post-rollback signal review. Keep the failed revision available only for analysis; do not retry it during the active incident.
