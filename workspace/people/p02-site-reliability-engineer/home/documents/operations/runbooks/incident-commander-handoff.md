# Incident commander handoff

## Purpose and trigger
Use whenever incident command transfers between shifts, roles, or locations while a Northstar production incident remains active.

## Impact signals
The risk is loss of decision context: duplicate mitigations, missed timers, unclear authority, or an unowned customer-impact update.

## Safe prerequisites
Both commanders join the incident bridge. The outgoing commander remains accountable until the incoming commander explicitly accepts command; do not transfer solely in chat.

## Diagnosis
1. Review incident scope, declared severity, service and regional impact, and current hypotheses.
2. Confirm active mitigations, rollback points, approvals, timers, and pending escalations.
3. Verify the current dashboard time range and evidence location.

## Mitigation
1. Outgoing commander states: situation, actions taken, actions prohibited, and next decision deadline.
2. Incoming commander repeats the summary and accepts command in the incident record.
3. Reassign communications, technical lead, and scribe roles by name; announce the handoff once.

## Verification
All open actions have owners and due times, the bridge roster is current, and the next update is scheduled.

## Rollback and escalation
If context is incomplete, defer transfer and page a backup commander. Escalate if no qualified commander is available within 15 minutes.

## Evidence capture
Record handoff timestamp, outgoing/incoming roles, decision log, open actions, and bridge attendee summary.

## Owners and review date
Owner: Incident Management. Review: 2026-09-18.

## Cross-references
[Error-budget policy exception](error-budget-policy-exception.md), [regional disaster recovery](regional-disaster-recovery.md), [log-export delivery gap](log-export-delivery-gap.md).
