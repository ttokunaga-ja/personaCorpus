# Product Alpha R6 → R7 upgrade guide

**Release:** Product Alpha R7 · 2026-07-13 · Aster Loop 株式会社 · ADR-042

1. **Preflight.** Confirm the deployment inventory reports R6, the release-gate worker has access to the internal decision queue, and monitoring can read `gate_completed_ms`. Record the planned maintenance window; no customer data is required.
2. **Freeze promotion.** Pause new promotion requests, allow already accepted R6 gates to finish, and retain the R6 status-polling path.
3. **Deploy control plane.** Apply the R7 release-gate service. Verify that `POST /release-gates/evaluate` returns `202` and a non-empty synthetic `gate_id`.
4. **Deploy worker.** Roll the deploy worker pool in small batches. The worker must enqueue asynchronous gate evaluation rather than block for a terminal result.
5. **Validate terminal states.** Submit ten synthetic canary requests. For each request, poll `GET /release-gates/{gate_id}` until `accepted` or `rejected`; expected p95 completion is at or below 184 ms.
6. **Validate compatibility.** Run the R6 polling client against the R7 status endpoint. It must ignore `decision_ref` and correctly read terminal states.
7. **Promote.** Remove the promotion freeze only after the release owner confirms the validation evidence and ADR-042 decision reference are present.
8. **Recover if needed.** If terminal states remain pending beyond the trigger in [rollback notes](rollback-notes.md), stop promotion, preserve gate records, and execute the documented rollback commands. Do not delete queued records.

For known symptoms and mitigations, see [known issues](known-issues.md). For the full R7 change record, see [release notes](product-alpha-r7-release-notes.md).
