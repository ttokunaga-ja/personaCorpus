# Identity session-store failover

## Purpose and trigger
Use when the active Identity session store is unavailable, has write errors above 1%, or its controller declares a replica eligible.

## Impact signals
New sessions fail, session refreshes loop, and Checkout authentication success falls. Existing stateless token flows may remain normal.

## Safe prerequisites
Name an Identity lead, pause session-schema changes, verify replica lag below ten seconds, and enable the read-only session inventory. Do not delete sessions to relieve pressure.

## Diagnosis
1. Inspect `northstarctl session-store status --cluster identity-sessions`.
2. Confirm candidate replication continuity and fencing status.
3. Check whether service-mesh errors are the actual cause.

## Mitigation
1. Enable a brief refresh grace period: `northstarctl identity session-grace set --seconds 120`.
2. Request managed failover: `northstarctl session-store failover request --cluster identity-sessions --candidate replica-b1`.
3. After leadership is confirmed, restart only session writers: `kubectl -n northstar-identity rollout restart deploy/session-writer`.

## Verification
Session create and refresh success exceed 99.8%, leader is singular, and refresh backlog clears for 15 minutes.

## Rollback and escalation
Turn off grace mode once recovery is stable. If failover fencing does not complete in eight minutes, halt writer restarts and page Identity Storage Critical.

## Evidence capture
Preserve leader transitions, grace interval, lag evidence, and session success time series.

## Owners and review date
Owner: Identity Storage. Review: 2026-10-01.

## Cross-references
[Identity token validation degradation](identity-token-validation-degradation.md), [Postgres replica lag](postgres-replica-lag.md), [incident commander handoff](incident-commander-handoff.md).
