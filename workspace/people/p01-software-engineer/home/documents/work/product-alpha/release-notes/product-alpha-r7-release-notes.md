# Product Alpha R7 release notes

**Release date:** 2026-07-13  
**Organization:** Aster Loop 株式会社  
**Decision record:** [ADR-042: asynchronous release gate](../../../../../../../../work-items/decision-records/adr-042-async-release-gate.md)

## Highlights

Product Alpha R7 makes the release gate asynchronous. Deployment admission now records a durable gate request, evaluates checks without holding the deploy worker, and publishes a signed internal decision event to the release timeline. The resulting p95 gate completion time is **184 ms** across the synthetic canary observation window.

The new flow reduces deploy-worker contention and gives operators a distinct `pending`, `accepted`, or `rejected` decision before promotion. The behavior is governed by **ADR-042** and is compatible with R6 clients that continue to poll the existing status endpoint.

## Breaking changes

- The legacy synchronous `POST /release-gates/evaluate` endpoint returns `202 Accepted` with a `gate_id`; it no longer waits for a terminal decision.
- Automation that treated an HTTP `200` from the legacy endpoint as a promotion signal must instead wait for a terminal gate state.
- Gate decision payloads now include `decision_ref`; consumers must ignore unknown fields to preserve forward compatibility.

## Non-breaking changes

- `GET /release-gates/{gate_id}` remains available and exposes `pending`, `accepted`, and `rejected` states.
- Release timelines include elapsed milliseconds and the ADR-042 decision reference.
- Synthetic canary telemetry records p50, p95, and p99 latency without customer identifiers.

## Upgrade and rollback

Follow the ordered [upgrade guide](upgrade-guide.md) for R6 → R7 preflight, deployment, validation, and recovery. If a terminal decision cannot be observed within the operational window, use the [rollback notes](rollback-notes.md); the compatibility window and data caveats are explicit there.

## Validation snapshot

| Signal | Result |
|---|---:|
| Gate completion p95 | 184 ms |
| Canary decision completion | 48 / 48 synthetic requests |
| Resolved warnings | 1 |
| Release decision | Approved under ADR-042 |

Known operational limitations and their owners are maintained in [known issues](known-issues.md). The role sign-off and test totals are captured in [release QA summary](release-qa-summary.txt).

## References

- [Release metadata](release-metadata.yaml)
- [Artifact digest fixture](artifact-digests.json)
- [Change summary](change-summary.csv)
- [Accessible HTML preview](release-notes-preview.html)
