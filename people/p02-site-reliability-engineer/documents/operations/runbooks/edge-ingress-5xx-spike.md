# Edge ingress 5xx spike

## Purpose and trigger
Use when edge-origin 5xx exceeds 1% for five minutes in a Northstar region.

## Impact signals
Client failures occur before application logs, edge retries rise, or a single route is disproportionately affected.

## Safe prerequisites
Confirm alert scope with synthetic probes from two zones. Freeze ingress policy changes and preserve the active route revision.

## Diagnosis
1. Query `northstarctl edge status --region nstar-a --service checkout-svc17`.
2. Compare TLS handshake failures, origin connection resets, and route-level error rates.
3. Verify the upstream service endpoints are ready before modifying edge weights.

## Mitigation
1. Roll back the latest approved edge route revision if correlated.
2. Drain one unhealthy edge pool: `northstarctl edge pool drain --pool nstar-a-edge-2`.
3. If origin capacity permits, shift 10% through the regional traffic control.

## Verification
Edge 5xx remains below 0.2%, TLS failures normalize, and synthetic checkout completes for 15 minutes.

## Rollback and escalation
Re-enable the pool if the alternative pool crosses 75% utilization. Escalate to Edge Network if two pools fail or handshake errors persist.

## Evidence capture
Capture route revision, pool health, probe IDs, and ingress request samples with identifiers redacted.

## Owners and review date
Owner: Edge Network Reliability. Review: 2026-09-25.

## Cross-references
[Regional traffic shift](checkout-svc17-regional-traffic-shift.md), [synthetic check triage](synthetic-check-failure-triage.md), [service mesh certificate rotation](service-mesh-certificate-rotation.md).
