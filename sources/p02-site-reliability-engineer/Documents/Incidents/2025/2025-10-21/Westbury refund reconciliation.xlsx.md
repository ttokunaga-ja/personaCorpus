---
path: Documents/Incidents/2025/2025-10-21/Westbury refund reconciliation.xlsx
kind: brt-refund-reconciliation.xlsx
author: Daniel Jennings
derived_from:
---
# Reconciliation

Sheet: Reconciliation

- B3: BRT-RF-20251021-001
- E3: Daniel Jennings
- B4: 21 Oct 2025
- E4: Westbury refunds due 20–21 October UTC; snapshot 21 October 2025 10:00 UTC

| Row | A: Refund due date (UTC) | B: Late refunds | C: Matched to original booking | D: Submitted | E: Awaiting submission | F: Unmatched booking |
|---|---|---:|---:|---:|---|---|
| 7 | 20 Oct 2025 | 36 | 36 | 32 | =B7-D7 | =B7-C7 |
| 8 | 21 Oct 2025 | 26 | 26 | 22 | =B8-D8 | =B8-C8 |
| 9 | Total | =SUM(B7:B8) | =SUM(C7:C8) | =SUM(D7:D8) | =SUM(E7:E8) | =SUM(F7:F8) |

A10:F10: Cohort comprises refunds still outstanding at their individual due times, all due by the snapshot. All 62 were checked against original bookings. Submission is a processing stage, not confirmation of receipt in the customer's account; bank arrival has not been verified.

The total formulas give 62 late, 62 matched, 54 submitted, 8 awaiting submission and 0 unmatched.
