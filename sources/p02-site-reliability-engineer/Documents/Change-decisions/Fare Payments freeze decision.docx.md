---
path: Documents/Change-decisions/Fare Payments freeze decision.docx
kind: brt-change-decision.docx
author: Anjali Shah
derived_from:
---
# Fare Payments freeze stays in place

Reference  BRT-CD-20260713-001 | Issued  13 Jul 2026

Prepared by  Anjali Shah

For Matthew Pickering, Daniel Jennings and Claire Hutchinson

## Decision

Issued at 12:10 UTC on 13 July 2026. Continue the existing freeze on normal BRT Fare Payments production changes. Hold the next Faregate rollout tranche and the routine payment reconciliation release in the queue. The observed recovery and completed watch do not themselves lift the freeze.

## Basis and restrictions

Daniel's BRT-EB-20260713-001 puts the July error budget at 2.4% remaining as at 13 July 11:00 UTC: 1,250 allowed failures on 1,250,000 eligible attempts, 1,220 used and 30 left. This is a snapshot through the stated cutoff. With so little allowance left, keep normal changes on hold while we review the balance and the queue.

Investigation outside production may continue, including Matthew's failure-count check. Any urgent production mitigation requires separately recorded Operations approval; this decision does not authorise it.

## Review conditions

At 10:00 UTC on 14 July, Anjali, Daniel and Matthew will recheck the balance and the two queued items. The checkpoint is a review, not an automatic release: the freeze and holds continue unless Operations makes and records a further decision. Claire may use this decision when explaining the restriction to Westbury.

Anjali Shah
Operations Director
