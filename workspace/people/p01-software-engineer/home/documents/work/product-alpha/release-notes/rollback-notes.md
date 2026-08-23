# Product Alpha R7 rollback notes

**Compatibility window:** R6 polling clients and R7 gate records are supported together for 14 days after 2026-07-13. Keep the R7 status endpoint available throughout the window.

## Rollback triggers

- Gate completion p95 exceeds 250 ms for 15 consecutive minutes.
- More than 2% of synthetic canary requests remain `pending` for over 5 minutes.
- A terminal decision is missing its ADR-042 `decision_ref`.

## Data caveats

Gate requests and terminal decision events are append-only operational records. Do not purge queued or completed records during rollback; R6 workers can continue to read terminal status, while R7 records retain the asynchronous decision reference. Replaying a request creates a new `gate_id` and must not replace the original record.

## Commands

```sh
alpha-release promotion pause --reason "R7 async gate rollback"
alpha-release worker set-version --target r6 --batch-size 2
alpha-release control-plane set-version --target r6-compatible
alpha-release gate verify --sample synthetic-canary --expect-terminal
alpha-release promotion status
```

## Evidence to retain

Record the trigger time, synthetic request count, p50/p95/p99 observations, affected component version, and the final R6-compatible status check. Attach the result to the R7 timeline with the **ADR-042** reference. See [upgrade guide](upgrade-guide.md) for the forward path.
