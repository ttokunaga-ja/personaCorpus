# Identity directory sync lag investigation

## Scope

Directory sync propagates account status and group membership from the
authoritative directory into identity policy snapshots. A lag event can cause
late recognition of a new role or a delayed removal; it does not justify
manually granting permissions in the identity service.

## Triage checklist

1. Read the latest source checkpoint and target-applied checkpoint.
2. Measure oldest unapplied event age and queue depth by tenant class.
3. Check for malformed-event quarantine volume and consumer restarts.
4. Compare directory API latency with snapshot write latency.
5. Confirm whether the concern is an addition, removal, or account-status
   change; removals receive the highest priority.

## Mitigation

For removals or suspensions delayed more than five minutes, page the directory
and identity owners together. Enable the priority revocation lane only for
status-removal events; it preserves ordering within that lane and records a
separate audit reason. For ordinary additions, restore consumer throughput and
allow ordered replay to converge.

## Completion criteria

The backlog is complete when the oldest event is under 60 seconds, quarantine
volume is stable, and sampled source-to-policy records match in sequence. Add
the highest observed age, cause category, and whether any removal used the
priority lane to the incident record.

## Data handling

Use opaque event IDs and aggregate counts in SRE notes. Directory attributes,
names, and membership values belong only in the protected system of record.
