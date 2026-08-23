# Identity cache flush procedure

## Intent

Flush only the narrowly scoped identity cache that contains stale public
metadata, policy snapshots, or directory projection entries. Broad cache
removal can create a thundering herd against identity dependencies and should
not be used as first-line diagnosis.

## Identify the cache

| Cache | Typical stale symptom | Preferred action |
|---|---|---|
| Trusted key set | `unknown_kid` after a confirmed publication | Refresh key-set namespace |
| Policy snapshot | Decisions disagree with current approved policy | Invalidate affected policy version |
| Directory projection | Account status is behind source checkpoint | Refresh affected projection partition |
| Session metadata | Session lookup uses outdated non-security preference | Expire named session metadata shard |

## Procedure

1. Capture current cache age, hit ratio, backing-service latency, and reason
   for invalidation.
2. Select one namespace and one canary region.
3. Perform the supported scoped invalidation through the control plane.
4. Watch cache-miss rate and backing-service saturation for ten minutes.
5. Repeat for additional regions only when the backing dependency remains
   below 50% utilization and error rate is unchanged.

## Safety constraints

Never clear authentication-factor enrollment, revocation state, or all
namespaces at once. If stale data concerns account suspension or key trust,
involve Security Operations before invalidation and confirm the authoritative
feed is healthy first.

## Record

Log the namespace, region, cache generation before and after, operator, reason,
and observed effect. Do not paste cached identity payloads into the record.
