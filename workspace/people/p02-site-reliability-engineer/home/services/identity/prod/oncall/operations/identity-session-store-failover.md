# Identity session-store failover

## When to use

Invoke this procedure when session reads fail above 1% for five minutes, the
primary store reports quorum loss, or replication delay exceeds 45 seconds.
The service may continue accepting already validated bearer tokens; do not
assume that a session-store incident permits new session issuance without
review.

## Preconditions

- Incident commander has declared an identity incident.
- A database operator confirms the target replica is writable and within the
  approved recovery-point objective.
- Identity and Security Operations approve the failover window.
- The audit stream consumer is healthy or its buffering limit is understood.

## Procedure

1. Freeze nonessential session preference writes and announce the change in
   the incident channel.
2. Measure replica lag, read error rate, and active-session lookup latency.
3. Shift one low-risk region's session reads to the standby endpoint.
4. After five minutes, compare `session_not_found` and refresh-token failure
   rates against the pre-change baseline.
5. Shift remaining regions in two stages, stopping if either metric rises by
   more than 0.5 percentage points.
6. Keep the former primary fenced from application writes until database
   ownership is formally recovered.

## Consistency behavior

During the stated recovery-point window, a small number of recently revoked
sessions can appear active on the standby. Compensate by applying the compact
revocation feed before widening the traffic shift. Do not shorten session
lifetimes globally: that increases customer impact while providing no evidence
that the standby is current.

## Exit criteria

Return only after the original store has stable quorum, replication catch-up is
zero for 20 minutes, and the revocation-feed checkpoint matches the active
session writer. Document the final reader and writer ownership in the on-call
handoff.
