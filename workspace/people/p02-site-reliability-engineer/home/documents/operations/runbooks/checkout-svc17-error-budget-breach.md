# Checkout SVC-17 error-budget breach

## Purpose and trigger
Use when the 30-day availability budget for **Northstar Transit Cloud** **Checkout SVC-17** falls below **error budget 2.4%**. This runbook was opened for **INC-260713** on **2026-07-13**; it remains the operational baseline.

## Impact signals
- `checkout_success_ratio` below 99.5%, increasing `checkout_http_5xx_total`, or failed payment authorization retries.
- The error-budget alert is firing in two regions for more than 10 minutes.

## Safe prerequisites
Declare incident command, freeze nonessential Checkout changes, and record the dashboard time range. Use the read-only `northstarctl` profile; do not drain nodes or change payment routing before a second on-call confirms impact.

## Diagnosis
1. Compare `nstar-a` and `nstar-b` success ratio, p95 latency, and request volume.
2. Run `northstarctl metrics query --service checkout-svc17 --window 15m` and correlate errors by dependency.
3. Check recent approved changes with `northstarctl change list --service checkout-svc17 --since 2h`.

## Mitigation
1. Pause the current Checkout rollout: `kubectl -n northstar-checkout rollout pause deploy/checkout-svc17`.
2. If one region is unhealthy, shift no more than 15% traffic using the approved regional traffic-shift runbook.
3. If a dependency dominates errors, apply its dedicated runbook and keep payment retries capped at the documented safe rate.

## Verification
Observe two consecutive 10-minute windows with success ratio at or above 99.7%, stable queue age, and no new alert events. Resume rollout only after the incident commander approves.

## Rollback and escalation
Undo the traffic step if destination p95 grows above 900 ms. Escalate to Payments and Database on-call for dependency error share above 20%; escalate to the reliability duty manager if budget consumption continues for 30 minutes.

## Evidence capture
Save dashboard links, change IDs, command output summaries, mitigation timestamps, and the alert fingerprint in the incident record.

## Owners and review date
Owner: Checkout Reliability. Review: 2026-10-13.

## Cross-references
[Database failover](checkout-svc17-database-failover.md), [regional traffic shift](checkout-svc17-regional-traffic-shift.md), [policy exception](error-budget-policy-exception.md).
