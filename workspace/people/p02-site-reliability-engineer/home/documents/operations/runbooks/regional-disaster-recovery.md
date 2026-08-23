# Regional disaster recovery

## Purpose and trigger
Use for a declared regional outage or loss of critical regional dependencies that cannot be mitigated by incremental traffic shifting.

## Impact signals
Multiple zones unavailable, edge and service failures persist, control-plane health is impaired, or regional recovery estimate exceeds the service objective.

## Safe prerequisites
Require executive incident declaration, disaster-recovery lead, and confirmation that the recovery region has capacity and valid data-replication posture. Stop nonessential changes globally.

## Diagnosis
1. Assess region status with `northstarctl region health --region nstar-a`.
2. Verify recovery-region Checkout, Identity, database, queue, and observability readiness.
3. Confirm replication recovery point and identify functions that must remain disabled.

## Mitigation
1. Declare the affected region unavailable in the traffic controller.
2. Shift traffic in the preapproved recovery sequence: edge, stateless services, async consumers, then write-capable flows.
3. Enable the recovery-region capacity reservation: `northstarctl capacity reservation activate --name dr-nstar-b`.
4. Keep reconciliation and deferred-payment processing gated until data leadership approves.

## Verification
Synthetic Checkout and Identity journeys pass from two zones, recovery-region saturation is below 70%, and data reconciliation checkpoints advance.

## Rollback and escalation
Do not fail back while the source region is unstable. Escalate to executive crisis management for multi-region impact, data-integrity uncertainty, or recovery-region exhaustion.

## Evidence capture
Capture declaration time, region health, traffic stages, replication recovery point, capacity actions, and reconciliation status.

## Owners and review date
Owner: Disaster Recovery Council. Review: 2026-11-01.

## Cross-references
[Regional traffic shift](checkout-svc17-regional-traffic-shift.md), [Checkout database failover](checkout-svc17-database-failover.md), [incident commander handoff](incident-commander-handoff.md).
