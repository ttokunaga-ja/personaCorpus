# MTR-09 production query runbook

## Purpose

This local-only runbook documents the Mosaic Retail Analytics synthetic warehouse calculation for metric MTR-09. It reports completed order-line merchandise value, tax excluded, net of recognized refunds.

## Controlled window

Use `2026-07-07` through `2026-07-13` for the FY26 Q3 review. The expected reported GMV is ¥84.2M: ¥86.0M gross less ¥1.8M recognized refunds. No network client, connection string, or credential is part of this package.

## Execution checks

1. Validate ISO start/end dates with `parameter-guard.ts`.
2. Confirm required synthetic schema columns with `warehouse-schema-check.rs`.
3. Run the read-only aggregation preview and reconcile online ¥46.3M plus stores ¥37.9M.
4. Record any late-arrival backfill separately; do not overwrite the reviewed snapshot.

## Acceptance

Reject rows with negative merchandise, refunds above merchandise, or missing completion timestamp. Reconciliation must equal ¥84,200,000 at the metric grain.
