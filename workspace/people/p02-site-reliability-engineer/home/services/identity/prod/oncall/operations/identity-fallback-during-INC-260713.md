# Identity fallback during INC-260713

**Incident window:** 2026-07-13, 09:18–10:02 UTC  
**Service owner:** Northstar Transit Cloud Identity Platform  
**Severity:** S2  
**Related service:** Checkout SVC-17  
**Budget position at declaration:** error budget 2.4% remaining  
**Incident:** INC-260713

## What happened

The primary identity policy evaluator in the north region began returning
deadline-exceeded responses after a configuration snapshot backlog increased
from 18 seconds to 11 minutes. Login traffic was still accepted at the edge,
but policy decisions for newly issued sessions exceeded the 750 ms request
budget. Existing signed-in sessions remained valid.

At 09:25 UTC the incident commander approved the documented fallback: route
interactive sign-in evaluation to the warm west evaluator pool while keeping
token signature verification local. This reduced login failures from 8.7% to
0.4% within six minutes. No authentication factor was bypassed and no session
duration was extended.

## Guardrails used

- The fallback applied only to `interactive-login` and `session-refresh`.
- Machine-to-machine token exchange stayed pinned to its normal regional path.
- The edge continued validating issuer, audience, expiry, and key identifier.
- MFA challenges remained required whenever the policy required them.
- The west pool capacity cap was 35% of its measured steady-state headroom.

## Timeline

| UTC | Observation or action | Owner |
|---|---|---|
| 09:18 | `identity_policy_eval_latency` crossed 600 ms for five minutes. | On-call SRE |
| 09:22 | Confirmed snapshot lag, no key or directory errors. | Identity engineer |
| 09:25 | Approved cross-region evaluator fallback. | Incident commander |
| 09:31 | Error rate returned below the 1% paging threshold. | On-call SRE |
| 09:43 | Snapshot consumers caught up; held fallback for stability. | Platform engineer |
| 10:02 | Returned interactive evaluation to north after 15 clean minutes. | Incident commander |

## Decision record

Fallback was preferable to relaxing authorization because the decision service
was unhealthy, not the trust material. We chose a capacity-bounded regional
route rather than cached allow decisions so that policy changes and account
status remained authoritative. The checkout team was notified because a small
share of customers might see a slightly longer sign-in redirect.

## Recovery checks

1. Verify north snapshot age is below 60 seconds for 15 consecutive minutes.
2. Compare successful-login rate by region and client type; investigate a
   difference greater than two percentage points.
3. Confirm west evaluator utilization is below 55% after traffic returns.
4. Record the fallback interval in the incident channel and attach dashboard
   links to the incident record.
5. Open a follow-up for snapshot backlog alerting at 90 seconds and 300
   seconds, with separate warning and page thresholds.

## Customer and audit impact

There was no evidence of invalid token acceptance, MFA bypass, or privilege
change. Authentication decisions continued to be written to the regional audit
stream. The audit-log reconciler should nevertheless compare sign-in decision
counts across both evaluator pools for the 09:18–10:02 UTC window.
