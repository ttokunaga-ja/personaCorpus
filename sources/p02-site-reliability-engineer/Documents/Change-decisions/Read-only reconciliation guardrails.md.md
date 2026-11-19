---
path: Documents/Change-decisions/Read-only reconciliation guardrails.md
kind: text
author: Daniel Jennings
derived_from:
---
# Read-only reconciliation guardrails

For Matthew and the on-call team. Recorded by Daniel after the review with Anjali and Matthew at 11:00 UTC on 18 August 2026.

## One window

On 19 August, 09:00–09:30 UTC (10:00–10:30 BST), Matthew may run shadow reads for up to 10% of eligible reconciliation jobs. Keep the existing production path in place. Comparison results must neither write nor post payments. This is one window, not a release of the normal reconciliation change or the next Faregate rollout tranche.

## Before starting

Matthew owns the start decision. Verify that the normal worker pool is in place and the workload mix matches the representative mix used for the 2,800 eligible attempts/min ceiling. Verify that the comparison cannot write or post payments. Daniel checks headroom is at least 25% before the window. If any condition cannot be verified, do not start.

## During the window

Daniel watches the p95 of one-minute eligible attempt rates over the trailing 15 minutes, before and during the run. Calculate headroom as (2,800 − p95 rate) / 2,800 against the verified ceiling, with the pool and mix qualification above. Matthew stops immediately if headroom falls below 25%, any output discrepancy appears, or any customer-facing regression occurs. Do not continue merely because the other measures remain healthy.

Record the observations, including any stop or the reason for standing down, for the Operations review at 11:00 UTC on 19 August. The continuing holds are set out in BRT-CD-20260713-001; the exception is recorded in BRT-CD-20260818-001. Any expansion needs Anjali's further decision.
