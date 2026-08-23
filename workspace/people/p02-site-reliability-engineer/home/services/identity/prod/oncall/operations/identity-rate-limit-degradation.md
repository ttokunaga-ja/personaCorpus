# Identity rate-limit degradation response

## Signal

Page when the identity gateway returns rate-limit responses above 3% for ten
minutes, or when a single client category consumes more than 30% of login
evaluation capacity. The rate limiter protects the policy evaluator and MFA
orchestrator from cascading failure.

## Diagnose before changing limits

| Question | Evidence |
|---|---|
| Is the load concentrated? | Requests by client application, route, and region |
| Is it retry amplification? | Retry count and inter-request interval |
| Is the backend slow? | Evaluator queue depth and upstream latency |
| Is it abusive traffic? | Anonymous attempt distribution and edge challenge results |

## Degradation ladder

1. Preserve the strict limit for anonymous password and recovery routes.
2. Apply retry-after guidance and exponential backoff headers to interactive
   clients.
3. Prioritize authenticated session refresh over new low-trust sign-in
   attempts when the evaluator queue is saturated.
4. Increase a verified first-party client allowance only with its service
   owner and incident commander approval, for a maximum of 30 minutes.
5. Restore normal limits gradually after backend utilization remains below 60%
   for 20 minutes.

## Prohibitions

Do not set an unlimited bucket, disable bot challenges, or exempt an unknown
client identifier during an incident. Those actions move pressure downstream
and erase a useful protection boundary.

## Handoff note

Record the affected route, limiting dimension, temporary allowance, expiry,
and post-recovery rejection rate. Include only aggregate client labels, never
end-user identifiers.
