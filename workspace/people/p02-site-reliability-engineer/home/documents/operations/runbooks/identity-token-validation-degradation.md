# Identity token-validation degradation

## Purpose and trigger
Use when Identity token validation latency exceeds 400 ms or invalid-validation responses rise above 1% without a matching client release.

## Impact signals
Checkout may reject authenticated requests, while session creation remains healthy. Separate signature failures from key-discovery timeouts.

## Safe prerequisites
Open an Identity incident and preserve key-cache state. Confirm the active issuer set from the configuration inventory; never bypass signature verification.

## Diagnosis
1. Run `northstarctl metrics query --service identity --metric token_validation_latency --window 10m`.
2. Compare key-cache hit rate, issuer endpoint availability, and clock-skew warnings.
3. Check change history for trust-policy or certificate updates.

## Mitigation
1. Pin validation to the last known-good issuer metadata through `northstarctl identity issuer pin --profile stable-issuer-v4`.
2. Scale validation workers by two only if CPU exceeds 70%.
3. If one issuer is failing, isolate it with the approved issuer-routing rule and notify its owning team.

## Verification
Validation p95 falls below 250 ms, signature success returns to baseline, and Checkout authenticated success is stable for 15 minutes.

## Rollback and escalation
Remove the pin after issuer health is independently confirmed. Escalate to Identity Security for unexplained signature failures or any suspected key mismatch.

## Evidence capture
Record issuer health, cache metrics, policy revision, and affected synthetic journey IDs.

## Owners and review date
Owner: Identity Reliability. Review: 2026-10-13.

## Cross-references
[Service mesh certificate rotation](service-mesh-certificate-rotation.md), [Identity session-store failover](identity-session-store-failover.md), [synthetic check triage](synthetic-check-failure-triage.md).
