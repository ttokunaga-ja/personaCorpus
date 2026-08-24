# HelioDesk support incident coordination

## Scope

This guide applies to the 2026-08-04 bridge for `INC-2026-0804-03` and its
association with `KI-1054` (notification delivery lag). The 17 linked unresolved
cases are a coordination cohort, not a finding that all cases have one root cause.

## Bridge roles

| Role | Responsibility |
| --- | --- |
| Incident commander | Owns incident state, SLA decisions, and restoration declaration. |
| Engineering liaison | Reports telemetry, suspected scope, mitigation, and evidence gaps. |
| Support lead | Maintains the case map, runs support checkpoints, and escalates deviations. |
| Case owners | Investigate each case and send accurate, case-specific customer updates. |
| Status coordinator | Drafts approved public-status wording and timestamps published updates. |

## Cadence and customer updates

- Acknowledge the bridge within 20 minutes of the 09:20 UTC link event.
- Send the first case-specific update by 10:20 UTC, then update each affected
  customer at least hourly while the case remains linked and unresolved.
- Hold bridge checkpoints at 11:20 and 12:20 UTC. At the four-hour SLA endpoint
  (13:20 UTC), publish confirmed mitigation/restoration or an approved next update
  with a named role and time.
- State observed symptoms and verified actions. Do not present the incident link
  or KI-1054 association as confirmation of root cause for an individual case.

## Link management and closure criteria

Case owners may unlink a case when its independent cause is confirmed, recording
the reason in the case system. Keep a case linked if investigation remains open
and its symptoms still match the operational cohort.

The incident may close only when the incident commander has restoration evidence,
the engineering liaison has documented remaining uncertainty, the status
coordinator has issued a final approved update, and Support has reviewed all 17
linked cases for a case-specific next step. Closing the incident does not close
individual support cases; unresolved cases retain ownership and follow-up.

Reference status page: https://status.heliodesk.example.invalid/incidents/INC-2026-0804-03
