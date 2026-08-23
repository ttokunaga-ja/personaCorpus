# Identity on-call handoff

**Shift date:** 2026-07-13  
**Outgoing role:** Identity Platform on-call  
**Incoming role:** Identity Platform on-call  
**Service state:** Stable with enhanced monitoring after regional policy-evaluator fallback

## Current posture

The north policy evaluator recovered after a snapshot backlog. Interactive
traffic was returned from the west warm pool after 15 clean minutes. Current
snapshot age is below the 60-second objective and no error-rate threshold is
active. Retain elevated observation through the next peak sign-in period.

## Watch items

| Area | Signal | Escalate if | First contact |
|---|---|---|---|
| Policy snapshots | Oldest applied age | Above 90 seconds for five minutes | Identity Platform |
| Token validation | Unknown-key rejection rate | Above 0.05% | Identity and Security Operations |
| Sessions | Read failures | Above 1% for five minutes | Session Store on-call |
| MFA | Challenge completion | Below 97% for five minutes | MFA provider liaison |
| Audit | Destination lag | Above ten minutes | Audit pipeline owner |

## Open operational work

- Validate audit decision counts across the cross-region fallback interval.
- Review snapshot-consumer queue watermark alert thresholds.
- Confirm west warm-pool capacity accounting was returned to its ordinary
  reserve calculation.

## Handoff expectations

Read the active incident record before changing traffic, keys, session-store
ownership, or rate limits. Record meaningful actions with UTC timestamps and
aggregate metrics. Keep tokens, directory attributes, access codes, and other
sensitive authentication material out of handoff notes.

## Escalation principles

Prefer capacity and routing mitigations that preserve verification and MFA.
Any proposal to weaken token checks, authorization policy, or break-glass
controls requires the incident commander and Security Operations to join the
decision immediately.
