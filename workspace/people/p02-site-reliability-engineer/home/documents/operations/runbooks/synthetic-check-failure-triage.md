# Synthetic check failure triage

## Purpose and trigger
Use when a Checkout or Identity synthetic journey fails twice from independent Northstar probe zones.

## Impact signals
Probe failure, latency regression, DNS variance, or a mismatch between synthetic and real request telemetry.

## Safe prerequisites
Capture the probe run ID and assertion step. Treat probes as evidence, not customer data; do not replay a transaction outside the synthetic tenant.

## Diagnosis
1. Inspect `northstarctl synthetic run show --id syn-nstar-260713-042`.
2. Compare DNS, TLS, ingress, Identity, Checkout, and payment-stub timing.
3. Confirm whether failures reproduce from a second zone and whether the alert threshold is correctly configured.

## Mitigation
1. If only one probe zone fails, route the probe to its standby zone and open an Edge investigation.
2. If application failure is confirmed, invoke the relevant service runbook and freeze related deployments.
3. Suppress duplicate notifications for 30 minutes only after an incident exists.

## Verification
Three consecutive runs succeed from two zones with all assertions under their journey thresholds.

## Rollback and escalation
Remove temporary probe routing after zone health recovers. Escalate to Monitoring Platform when results conflict with independent service telemetry.

## Evidence capture
Store run IDs, assertion traces, zone selection, suppression ID, and comparison dashboard links.

## Owners and review date
Owner: Observability Reliability. Review: 2026-09-22.

## Cross-references
[Edge ingress 5xx spike](edge-ingress-5xx-spike.md), [Checkout error budget breach](checkout-svc17-error-budget-breach.md), [observability pipeline backpressure](observability-pipeline-backpressure.md).
