# Identity regional traffic shift

## Purpose

This procedure moves identity request traffic between approved regions during
a regional service degradation. It is for controlled routing changes, not a
substitute for policy, key, or database recovery.

## Readiness gates

| Gate | Requirement |
|---|---|
| Capacity | Destination has at least 40% headroom at current request mix |
| Trust data | Key-set and policy snapshot age are under 60 seconds |
| Sessions | Revocation feed is current and session-store health is green |
| MFA | Provider path succeeds at normal baseline in the destination |
| Ownership | Incident commander and destination on-call acknowledge |

## Shift plan

Start with 5% of interactive traffic and hold for ten minutes. Compare login
completion, token validation, MFA completion, and evaluator latency against
the unaffected region. Increase in 15% steps only when each signal remains
inside its service objective. Keep service-to-service exchanges regional until
their owner explicitly joins the change.

If the destination degrades, stop increasing weight and return the most recent
increment to the prior healthy route. Do not make simultaneous policy or key
changes during the shift; they obscure the source of a regression.

## Closeout

Record the traffic percentages, timestamps, observed headroom, and final
routing state. Once the impaired region recovers, restore traffic in smaller
increments than the emergency shift and retain enhanced monitoring for one
hour.
