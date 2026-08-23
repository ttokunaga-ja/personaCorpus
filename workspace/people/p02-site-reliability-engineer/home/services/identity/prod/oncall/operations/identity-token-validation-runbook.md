# Identity token validation runbook

## Purpose

Use this runbook when the edge rejects otherwise expected identity tokens, or
when token-validation latency threatens the 250 ms p95 objective. Validation
is deliberately local: it must not require a live call to the issuer for each
request.

## Fast triage

1. Classify the rejection using `reason`: `expired`, `unknown_kid`,
   `issuer_mismatch`, `audience_mismatch`, or `signature_invalid`.
2. Compare rejection rate by deployment region and client application.
3. Check the trusted-key cache age and last successful key-set refresh.
4. Inspect clock-skew telemetry at the verifier and issuer boundaries.
5. Correlate with the most recent approved key publication event.

## Expected checks

| Check | Healthy signal | Escalate when |
|---|---|---|
| Issuer | Exact configured issuer name | Any unexpected issuer gains volume |
| Audience | Service-specific audience match | A valid client fails in one region only |
| Expiry | Within configured lifetime and skew | Rejects cluster around a narrow clock offset |
| Key ID | Present in active or overlap key set | New key ID persists for more than 10 minutes |
| Signature | Verification succeeds with trusted public key | Failures rise without a key publication |

## Safe remediation

Refresh the verifier key-set cache through the deployment control plane, then
observe a single canary region for ten minutes. If an emergency key overlap is
needed, require two approvers from Identity Platform and Security Operations,
record the start and end time, and preserve the prior key set for rollback.
Never disable signature, issuer, audience, or expiry validation to restore
traffic.

## Evidence to capture

Capture aggregate rejection counts, cache age, deployment revision, and key
publication event ID. Redact token values and authorization headers from logs;
identifiers are sufficient for correlation.
