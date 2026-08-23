# Team release board — shared snapshot

**Aster Loop 株式会社 · Product Alpha R7 · release window 2026-07-13**  
Decision baseline: **ADR-042** async release gate · observed synthetic baseline: **p95 184 ms**.

| Workstream | Role owner | Status | Dependency | Target date |
|---|---|---|---|---|
| Release gate evidence | Release Engineering | Ready for review | Synthetic canary metric packet | 2026-07-11 |
| Event compatibility | Platform Engineering | In progress | Dual-run checkpoint comparison | 2026-07-12 |
| Operational rollback | Reliability Operations | Ready | Restore drill and cursor audit | 2026-07-12 |
| Change communication | Documentation Steward | Scheduled | Approved gate disposition | 2026-07-13 |

## Shared operating rules

1. The Release Engineering role records the ADR-042 disposition only after the
   compatibility and rollback evidence is attached.
2. Platform Engineering stops a cohort when terminal decisions differ, a replay
   is non-idempotent, or a sustained latency window exceeds the agreed bound.
3. Reliability Operations owns the reversible cursor check and announces the
   recovery evidence to the shared channel; no individual contact details are
   stored in this snapshot.

## Current dependencies and next handoff

The compatibility stream needs a matched checkpoint digest before the window
opens. The rollback drill is independent but its evidence must be available to
the release-gate review. The p95 184 ms baseline is context, not automatic
approval: the role-owned decision remains ADR-042.
