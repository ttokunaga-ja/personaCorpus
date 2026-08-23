# Message-bus consumer lag

## Purpose and trigger
Use when any production consumer group has increasing lag for 10 minutes or misses its defined processing-age objective.

## Impact signals
Backlog grows despite stable producer volume, rebalances repeat, or poison-message retries dominate a partition.

## Safe prerequisites
Assign a stream owner and confirm topic retention. Do not reset offsets or remove messages during an incident.

## Diagnosis
1. Run `northstarctl stream group describe --group checkout-events-v2`.
2. Identify whether lag is partition skew, consumer saturation, rebalance churn, or downstream refusal.
3. Compare retry-topic rate with primary-topic throughput.

## Mitigation
1. Pause noncritical consumers sharing the same worker pool.
2. Scale the affected group within partition count: `northstarctl stream group scale --group checkout-events-v2 --members 6`.
3. Quarantine a confirmed poison message using the controller's approved dead-letter action.

## Verification
Lag decreases across every partition, rebalances stop, and oldest event age meets objective for 15 minutes.

## Rollback and escalation
Reduce members if downstream connection saturation appears. Escalate to Messaging Platform if quorum health or retention is at risk.

## Evidence capture
Record offsets, member count, partition skew, dead-letter action ID, and downstream error rate.

## Owners and review date
Owner: Messaging Platform. Review: 2026-10-07.

## Cross-references
[Payment queue saturation](checkout-svc17-payment-queue-saturation.md), [observability pipeline backpressure](observability-pipeline-backpressure.md), [log-export delivery gap](log-export-delivery-gap.md).
