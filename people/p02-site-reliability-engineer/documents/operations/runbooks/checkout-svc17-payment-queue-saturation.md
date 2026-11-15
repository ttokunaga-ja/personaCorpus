# Checkout SVC-17 payment queue saturation

## Purpose and trigger
Use when `payment-intent-dispatch` depth exceeds 18,000 messages or oldest-message age exceeds 12 minutes.

## Impact signals
Customers may see pending confirmation; authorization calls slow while Checkout API availability can remain green. Watch retry amplification and payment-provider rejection percentage.

## Safe prerequisites
Assign a queue operator and payment liaison. Confirm idempotency keys are enabled before scaling consumers; do not purge a production queue.

## Diagnosis
1. Query partitions: `northstarctl queue describe --queue payment-intent-dispatch`.
2. Compare producer rate, consumer rate, retry topic volume, and downstream authorization latency.
3. Sample only synthetic correlation IDs from the incident console; avoid payload inspection.

## Mitigation
1. Pause noncritical replay jobs: `northstarctl queue consumer pause --group settlement-replay`.
2. Increase Checkout payment consumers by two replicas, maximum eight: `kubectl -n northstar-checkout scale deploy/payment-worker --replicas=6`.
3. If provider latency is causal, activate the approved 60-second retry floor and notify Payments.

## Verification
Depth must decline for 15 minutes, oldest age fall below five minutes, and duplicate suppression remain zero in the synthetic monitor.

## Rollback and escalation
Scale back one replica at a time if database connections exceed 75% of pool capacity. Escalate if retries exceed 8% or queue age grows after scaling.

## Evidence capture
Record queue offsets, scaling events, retry-floor change, and provider latency panels.

## Owners and review date
Owner: Checkout Async Processing. Review: 2026-09-30.

## Cross-references
[Message-bus consumer lag](message-bus-consumer-lag.md), [cache degradation](checkout-svc17-cache-degradation.md), [error budget breach](checkout-svc17-error-budget-breach.md).
