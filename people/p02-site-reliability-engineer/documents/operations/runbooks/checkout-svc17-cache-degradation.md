# Checkout SVC-17 cache degradation

## Purpose and trigger
Use when Checkout cache hit rate drops below 82%, eviction rate spikes, or cache timeout errors are contributing to request latency.

## Impact signals
Database read load rises, cart lookups slow, and API p95 increases without necessarily causing immediate 5xx.

## Safe prerequisites
Confirm cache cluster identity and ensure database read headroom exceeds 30%. Do not flush the shared cache, rotate credentials, or disable cache protection.

## Diagnosis
1. Run `northstarctl cache status --cluster checkout-session-cache`.
2. Compare memory pressure, hot-key distribution, connection resets, and database read QPS.
3. Check for a recent cache schema or TTL policy change.

## Mitigation
1. Roll back only the latest approved TTL policy if it explains evictions.
2. Add one cache shard through the capacity controller: `northstarctl cache scale --cluster checkout-session-cache --add-shards 1`.
3. Enable bounded stale reads for non-payment cart views: `northstarctl feature set --service checkout-svc17 --flag cart-stale-read --state enabled`.

## Verification
Hit rate returns above 88%, database read saturation stays below 65%, and Checkout p95 improves for 15 minutes.

## Rollback and escalation
Disable stale reads if cart consistency alarms fire. Escalate to Data Platform if shard movement fails or memory pressure persists.

## Evidence capture
Attach cache metrics, TTL change ID, shard operation ID, and stale-read interval.

## Owners and review date
Owner: Checkout Reliability. Review: 2026-10-06.

## Cross-references
[Database failover](checkout-svc17-database-failover.md), [payment queue saturation](checkout-svc17-payment-queue-saturation.md), [Kubernetes node pressure](kubernetes-node-pressure.md).
