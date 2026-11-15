# Customer Alpha escalation runbook

**Scope:** the 9 unresolved Alpha cases in the frozen 2026-08-04 09:00 UTC snapshot, including breached P1 `HD-260804-2718` (`open`).

## Start an escalation

1. Confirm case ID, priority, created time, and 4-hour first-response deadline in the active queue.
2. For breached `HD-260804-2718`, keep the P1 bridge open; name the Support Incident Lead and Engineering Liaison.
3. Send a meaningful acknowledgement from `alpha-escalations@heliodesk.invalid` that states the owner and next-update time.
4. Post a timestamped event to `escalation-events.jsonl` and update the CSV status.

## Manage the bridge

- Every 30 minutes, the Support Incident Lead records customer impact, current hypothesis, action owner, and next checkpoint.
- The Engineering Liaison confirms whether a mitigation is safe before it is promised externally.
- The Customer Success Partner consolidates all nine cases into one customer narrative; do not expose other customers’ details.
- For the frozen snapshot, work the Alpha at-risk cases `HD-260804-2841`, `HD-260804-2726`, and `HD-260804-2763`; document recovery for breached `HD-260804-2718`.

## Exit criteria

Close the escalation only after the first meaningful response is recorded, the customer has a next-update time, and the case owner has accepted follow-through. For missed SLAs, log the cause and corrective action before handoff.
