# Observability pipeline backpressure

## Purpose and trigger
Use when telemetry ingest delay exceeds five minutes, exporter retry buffers exceed 70%, or alert evaluation data is incomplete.

## Impact signals
Delayed metrics, missing traces, exporter memory growth, and alerts firing late rather than service degradation itself.

## Safe prerequisites
Declare an observability incident and preserve raw-buffer retention. Do not disable audit streams or discard logs to make a dashboard green.

## Diagnosis
1. Run `northstarctl telemetry pipeline status --region nstar-a`.
2. Compare collector CPU, buffer depth, receiver availability, and cardinality change history.
3. Identify whether one tenant namespace is creating excessive label combinations.

## Mitigation
1. Apply the approved high-cardinality guard to the offending synthetic namespace.
2. Scale collectors one replica at a time: `kubectl -n northstar-observe scale deploy/telemetry-collector --replicas=5`.
3. Prioritize SLO metrics and incident logs in the delivery class controller.

## Verification
Ingest delay falls below 60 seconds, buffer use below 40%, and alert evaluator freshness is current for 15 minutes.

## Rollback and escalation
Remove the guard after the source is corrected. Escalate to Observability Platform if retention risk reaches 80% or two regions are delayed.

## Evidence capture
Record lag, buffer utilization, guard policy revision, scaling history, and loss assessment.

## Owners and review date
Owner: Observability Platform. Review: 2026-10-08.

## Cross-references
[Log-export delivery gap](log-export-delivery-gap.md), [message-bus consumer lag](message-bus-consumer-lag.md), [synthetic check triage](synthetic-check-failure-triage.md).
