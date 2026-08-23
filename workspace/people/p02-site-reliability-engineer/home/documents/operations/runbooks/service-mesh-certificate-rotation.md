# Service mesh certificate rotation

## Purpose and trigger
Use for scheduled mesh certificate rotation or when certificate expiry is within 14 days and automated rotation has not completed.

## Impact signals
mTLS handshake failures, expiring-certificate alerts, or workloads reporting an unexpected trust bundle revision.

## Safe prerequisites
Use the approved maintenance window, inventory workload revisions, and confirm two valid trust bundles overlap. Never copy private key material into tickets or terminals.

## Diagnosis
1. Run `northstarctl mesh certificates status --environment production`.
2. Confirm issuer health, clock synchronization, and sidecar readiness by region.
3. Identify workloads still using the retiring bundle.

## Mitigation
1. Publish the signed next trust bundle with `northstarctl mesh bundle activate --name prod-bundle-2026q3`.
2. Restart canary workloads first: `kubectl -n northstar-checkout rollout restart deploy/checkout-svc17`.
3. Roll remaining namespaces in batches after a 10-minute handshake observation.

## Verification
All workloads report the new bundle, mTLS failures remain zero, and Checkout and Identity synthetic paths pass.

## Rollback and escalation
Restore the previous overlapping bundle if handshake failures begin. Escalate to Platform Security if issuer validation fails or overlap is absent.

## Evidence capture
Keep bundle revision, rollout batches, expiry status, and synthetic results.

## Owners and review date
Owner: Platform Security and Mesh Operations. Review: 2026-10-15.

## Cross-references
[Identity token validation degradation](identity-token-validation-degradation.md), [edge ingress 5xx spike](edge-ingress-5xx-spike.md), [Kubernetes pod crashloop](kubernetes-pod-crashloop.md).
