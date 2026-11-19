---
path: Documents/Incidents/2026/2026-07-13/Faregate incident review working copy.docx
kind: brt-incident-review.docx
author: Daniel Jennings
derived_from:
---
# Faregate incident review — working copy v0.1

Reference  BRT-IR-20260714-001 | Planned issue  14 Jul 2026

Prepared by  Daniel Jennings

Unissued — for Matthew Pickering's markup

## Summary

On 13 July 2026, the Faregate checkout path experienced failed payment attempts. The first confirmed affected attempt was at 08:07 UTC; the on-call page arrived at 08:12 UTC. Recovery was observed at 08:41 UTC and the follow-up watch passed at 08:56 UTC. The affected interval was 34 minutes. The cause remains open.

Matthew and I reconciled 312 failed eligible attempts among 4,800 eligible Faregate checkout attempts in the interval from 08:07 inclusive to 08:41 exclusive UTC. This is a service-wide attempt count, not a count of people or a Westbury-only figure.

## Timeline and evidence

| UTC | Event | Evidence / qualification |
|---|---|---|
| 08:07 | First confirmed affected attempt | The dashboard observation read 09:07 BST. Matthew confirmed the clock's time zone, giving 08:07 UTC. |
| 08:12 | On-call page received | Initially used as the provisional start; this is the detection time. |
| 08:18 | Initial internal update | About 380 failure indications from the overview; the population and customer reach had not yet been reconciled. |
| 08:29 | rollback confirmed | Matthew paused the staged rollout and returned traffic to the last stable route. |
| 08:41 | Recovery observed | The payment path showed recovery; continued observation was needed. |
| 08:56 | Follow-up watch passed | Three successive five-minute windows contained 740, 820 and 900 eligible attempts, with no repeat incident failures. |
| 10:20 | Evidence readback | Matthew, Claire and I agreed the corrected start and the impact reconciliation. |

## Impact reconciliation

The initial overview contained 384 indications, rounded to about 380 in the 08:18 update. Matthew's reconciliation separates 312 unique eligible failed attempts, 48 additional retry indications for those same attempts, and 24 diagnostic indications outside the eligible population. Only the 312 eligible failures are counted against the service measure. The 4,800 denominator covers the incident window, not the full month. Neither the number of affected people nor a Westbury-specific count can be inferred from these aggregates.

## Response and change restriction

The payment path recovered after the route intervention, but the evidence available for this draft does not establish the cause of the failure or the cause of recovery. Matthew, please check the wording of the intervention and the inference in the timeline before issue.

Daniel's July calculation, BRT-EB-20260713-001, showed 1,250 allowed failures on 1,250,000 eligible Fare Payments attempts through 11:00 UTC on 13 July. With 1,220 used, 30 remained: 2.4% of the allowance. Anjali's BRT-CD-20260713-001 continues the existing freeze on normal production changes. The next Faregate rollout tranche and the routine payment reconciliation release remain held. Non-production investigation may continue; urgent production mitigation requires a separately recorded Operations approval.

The balance and queue are due for review at 10:00 UTC on 14 July. That checkpoint does not automatically release the held changes.

## Customer communication and open point

Claire issued BRT-CU-20260713-001 at 14:05 UTC and sent it to Stephen Cartledge at Westbury, copying me, at 14:15 UTC. It describes the service-wide count and continuing restriction and asks the desk to check the booking and payment outcome, or contact Support, before asking for a second payment.

Matthew: please review whether the evidence supports attributing recovery to the 08:29 intervention alone. We should keep the cause open until that comparison is made.

## 変更履歴

- 種類: コメント
  人物: Daniel Jennings
  日時: 2026-07-13 16:42 BST
  対象の文: rollback confirmed
  新しい文: Does the evidence support attributing recovery to this step alone?
