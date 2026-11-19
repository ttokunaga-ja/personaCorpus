# Daniel Jennings — shared facts

## Faregate payment interruption (INC-260713), 13 July 2026

- BRT Fare Payments, Faregate checkout path. File times are BST (UTC+01:00); incident evidence is UTC. The dashboard observation at 09:07 BST converts to 08:07 UTC, the first confirmed affected attempt. The on-call page arrived at 08:12 UTC and supplied the provisional onset.
- Matthew paused the staged rollout and returned traffic to the last stable route at 08:29 UTC. Recovery was observed at 08:41 UTC (34 minutes after onset); the follow-up watch passed at 08:56 UTC. Cause and causal attribution to the intervention remain open.
- The 08:41–08:56 UTC watch counted 740, 820 and 900 eligible attempts in successive five-minute windows, with zero repeat incident failures (2,460 attempts total).
- Initial internal update at 08:18 UTC gave about 380 failure indications, rounded from 384. Reconciled at 10:20 UTC: 312 eligible failed attempts + 48 duplicate indications from retries within those same attempts + 24 ineligible diagnostic indications = 384. The 312 were among 4,800 eligible Faregate checkout attempts in [08:07, 08:41) UTC.
- The 312 is the service-wide incident checkout-attempt count, not a people count or a Westbury-only count. Four hourly Faregate buckets starting 07:00 through 10:00 UTC have eligible totals 7,960 / 8,640 / 9,100 / 8,800 and confirmed incident failures 0 / 312 / 0 / 0.
- Evidence readback on July 13 at 10:20 UTC: Daniel Jennings, Matthew Pickering and Claire Hutchinson. Matthew owns the impact reconciliation, Daniel the budget calculation, Claire the customer wording. Matthew's cause-evidence comparison is due July 17 at 12:00 UTC.

## July Fare Payments budget and change freeze

- BRT-EB-20260713-001 (Daniel, July 13) applies the 99.9% target to eligible Fare Payments attempts, July 1 00:00 through July 13 11:00 UTC, end exclusive. July 1–12: 1,200,000 eligible and 908 failed. July 13 through cutoff: 50,000 eligible and 312 failed.
- At the cutoff 1,250,000 attempts allow 1,250 failures. Of those, 1,220 were used and 30 remained, so the remaining allowance was 2.4%.
- BRT-CD-20260713-001, issued July 13 at 12:10 UTC by Anjali Shah, continues the existing normal production change freeze for BRT Fare Payments on the strength of BRT-EB-20260713-001. Hold the next Faregate rollout tranche and the routine payment reconciliation release.
- Non-production investigation may continue. Urgent production mitigation requires a separately recorded Operations approval. July 14 10:00 UTC queue checkpoint (Anjali, Daniel and Matthew): both items remain held, with no newly reconciled budget balance or automatic release.

## Faregate internal update and incident review

- “Faregate checkout: initial estimate” thread, July 13: (1) Daniel → Matthew, Claire and Anjali at 08:18 UTC (09:18 BST), with another internal update promised by 09:15 UTC; (2) Matthew → Daniel, Claire and Anjali at 09:10 UTC (10:10 BST), “Re: Faregate checkout: initial estimate”.
- BRT-IR-20260714-001 is Daniel's July 14 incident review reference, distinct from INC-260713. Working v0.1 saved July 13 15:42 UTC, Matthew's marked v0.1 returned July 14 08:35 UTC, and issued v1.0 released July 14 13:30 UTC. The first two copies are unissued.

## Westbury desk guidance after Faregate recovery

- “Westbury desk wording for pending receipts” thread, July 13: (1) Stephen Cartledge → Claire Hutchinson, Cc Daniel, at 13:02 UTC (14:02 BST); (2) Claire → Stephen, Cc Daniel, at 14:15 UTC (15:15 BST), “Re: Westbury desk wording for pending receipts”.
- Claire's BRT-CU-20260713-001, issued July 13 at 14:05 UTC, is sent as Faregate recovery and change restriction.pdf with her reply. Desk guidance is to check the booking and payment outcome or contact Support before asking for a second payment.
