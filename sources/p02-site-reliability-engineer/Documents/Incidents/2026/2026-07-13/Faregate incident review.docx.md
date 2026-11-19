---
path: Documents/Incidents/2026/2026-07-13/Faregate incident review.docx
kind: brt-incident-review.docx
author: Daniel Jennings
derived_from:
---
# Faregate incident review

Reference  BRT-IR-20260714-001 | Issued  14 Jul 2026

Prepared by  Daniel Jennings

Version 1.0 | Incident INC-260713 | For Anjali Shah, Matthew Pickering and Claire Hutchinson

## Summary

On 13 July, some BRT Fare Payments attempts through the Faregate checkout path failed. The first confirmed affected attempt was at 08:07 UTC and the on-call page arrived at 08:12 UTC. Recovery was observed at 08:41 UTC; the subsequent watch passed at 08:56 UTC. The interval from the first confirmed affected attempt to observed recovery was 34 minutes.

Matthew and I reconciled 312 failed eligible attempts among 4,800 eligible Faregate attempts in the interval from 08:07 inclusive to 08:41 exclusive UTC. The early dashboard count was a count of indications, not a count of customers. The cause of the failure and the reason it ended remain open.

## Timeline and evidence

The four milestones below were checked with Matthew and Claire at the 10:20 UTC readback on 13 July. All times are UTC.

| Milestone | Time | Evidence and interpretation |
|---|---|---|
| First confirmed affected attempt | 08:07 | The dashboard observation read 09:07 BST. Matthew confirmed the dashboard clock was in BST, equivalent to 08:07 UTC. This replaces the provisional start based on the page. |
| Detection | 08:12 | On-call page receipt. It is the alert time, five minutes after the first confirmed affected attempt, not the onset. |
| Recovery observed | 08:41 | The payment path showed recovery after traffic had been returned to the last stable route. This signal started the watch; it did not establish why the failures ended. |
| Verification | 08:56 | Three successive five-minute windows from 08:41 contained 740, 820 and 900 eligible attempts respectively, with no repeat incident failures. The watch covered 2,460 eligible attempts in total. |

At 08:18 UTC I reported about 380 failure indications from the first dashboard pass and said customer reach was still being checked. At 08:29 UTC Matthew paused the staged rollout and returned traffic to the last stable route. Recovery followed at 08:41 UTC. The sequence does not establish that the route change caused recovery, nor does it establish the cause of the failure.

## Impact reconciliation

The first overview contained 384 indications, rounded to about 380 in the initial update. At the 10:20 UTC readback Matthew reconciled the indications as follows:

| Disposition | Indications | Treatment |
|---|---:|---|
| Unique eligible failed attempts | 312 | Counted against the service measure. |
| Extra retry indications for those attempts | 48 | Excluded from the attempt count to avoid counting the same attempts again. |
| Diagnostic indications outside the eligible population | 24 | Excluded from the service measure. |
| Total overview indications | 384 | The first-pass signal count, not the confirmed failure count. |

The confirmed 312 failures are out of 4,800 eligible attempts during [08:07, 08:41) UTC. This is a service-wide Faregate attempt count. It is neither a count of people nor a Westbury-only count. The 4,800 is the incident-window denominator, not the monthly Fare Payments denominator.

## Response and change restriction

The 13 July 11:00 UTC snapshot in BRT-EB-20260713-001 used 1,250,000 eligible Fare Payments attempts for the month to that cutoff. At the 99.9% target, 1,250 failures were allowed; 1,220 had been used, leaving 30, or 2.4% of the allowance. Anjali's decision BRT-CD-20260713-001 continued the existing freeze on normal production changes.

At the 14 July 10:00 UTC checkpoint, Anjali, Matthew and I kept both the next Faregate rollout tranche and the routine payment reconciliation release on hold. No refreshed, reconciled July balance was presented. The 2.4% remains the 13 July snapshot, not a new checkpoint measurement, and the checkpoint did not release either change automatically. Non-production investigation may continue; urgent production mitigation needs a separately recorded Operations approval.

## Customer communication

Claire issued BRT-CU-20260713-001 on 13 July at 14:05 UTC and sent it to Stephen Cartledge at Westbury at 14:15 UTC, copying me. It explains the disruption and recovery, the service-wide count and the continuing restriction. For a missing receipt, the desk should check the booking and payment outcome or contact Support before asking for a second payment.

## Follow-up

| Owner | Action | Due |
|---|---|---|
| Matthew Pickering | Compare eligible attempts and incident failures on each side of 08:29 UTC, with boundaries and denominators explicit; check aggregate coverage, including retries and diagnostics, and bring the comparison and any gaps to Daniel. Do not attribute the failure or its end to the route change on timing alone. | 17 Jul 2026, 12:00 UTC |

Daniel Jennings
