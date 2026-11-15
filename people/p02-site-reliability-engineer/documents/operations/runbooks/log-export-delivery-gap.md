# Log-export delivery gap

## Purpose and trigger
Use when a required operational log export has a delivery gap over ten minutes or its signed manifest is missing.

## Impact signals
Destination watermark lags source watermark, retry queue rises, or incident correlation cannot locate expected records.

## Safe prerequisites
Open a tracking incident, identify source and destination retention windows, and preserve export checkpoints. Never reconstruct records from customer payloads.

## Diagnosis
1. Run `northstarctl log-export status --stream checkout-audit --region nstar-b`.
2. Compare source offset, destination watermark, delivery errors, and manifest status.
3. Determine whether the gap is producer, exporter, network, or destination throttling.

## Mitigation
1. Pause low-priority export streams through the priority controller.
2. Resume the affected exporter from its recorded checkpoint: `northstarctl log-export resume --stream checkout-audit --checkpoint retained`.
3. Request destination capacity if retry delay continues.

## Verification
Watermarks converge, manifests verify, and the gap is fully reconciled before closing the record.

## Rollback and escalation
Return priority allocations when backlog is below 20%. Escalate to Compliance Operations if retention could expire before reconciliation.

## Evidence capture
Capture checkpoint IDs, watermark graphs, manifest verification, and estimated missing interval.

## Owners and review date
Owner: Log Export Operations. Review: 2026-10-04.

## Cross-references
[Observability pipeline backpressure](observability-pipeline-backpressure.md), [message-bus consumer lag](message-bus-consumer-lag.md), [incident commander handoff](incident-commander-handoff.md).
