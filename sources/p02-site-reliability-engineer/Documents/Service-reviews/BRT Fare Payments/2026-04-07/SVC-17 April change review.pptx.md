---
path: Documents/Service-reviews/BRT Fare Payments/2026-04-07/SVC-17 April change review.pptx
kind: brt-reliability-briefing.pptx
author: Daniel Jennings
derived_from:
---

## Slide 1 — Cover

Title: Fare Payments: SVC-17 change review

Management field:
BRT-RB-20260407-001
7 April 2026 | Daniel Jennings

Speaker notes: Discussion with Anjali Shah and Matthew Pickering, 10:00 UTC. The question is whether the available evidence supports a feature release in April, and what to leave out of May planning if it does not.

## Slide 2 — Heading and body

Title: April budget at the review cutoff

- 1 April 00:00 to 7 April 08:00 UTC, end exclusive: 600,000 eligible attempts.
- At the 99.9% objective, allowance is 600 failures; 570 used and 30 remaining.
- Remaining share: 30 / 600 = 5.0% of the allowance at the same cutoff.
- This is an attempt-based month-to-date balance, not a forecast for the rest of April or May.

Speaker notes: Source: BRT-EB-20260407-001. Both the count and the percentage use the same cumulative cutoff. The 30 remaining is a narrow margin for ordinary variation and any release-related failure; it does not quantify the risk of a particular change.

## Slide 3 — Table

Title: Four proposals awaiting a decision

| Feature | Question for acceptance |
|---|---|
| Checkout saved-card prompt | Is the saved-card label clear at checkout? |
| Receipt resend link | Does resend lead to the intended destination? |
| Pending-payment banner | Does the wording remain appropriate when the outcome is delayed? |
| Wallet button placement | Is the ordering of payment choices clear? |

Speaker notes: Matthew's queue has all four awaiting a decision. Apparent size alone does not establish either a safe release or the balance available in May.

## Slide 4 — Two columns

Title: Decision requested: April and May

Left — Keep the feature queue closed:
- Hold all four out of April–May production scheduling.
- Continue preparation away from production.
- Reassess with a fresh budget in June.

Right — Proceed in small stages:
- May reduce exposure per stage, but still consumes risk against the same narrow remaining margin.
- A smaller stage does not establish headroom or justify selecting another item.
- Requires a decision on acceptable risk and recorded closeout evidence.

Speaker notes: I recommend keeping the queue closed. Thirty failures remain in the observed allowance, and we cannot responsibly assume they will cover normal variation plus an additional release. A small stage limits scope, not the uncertainty of the remaining month. A June reassessment should consider a refreshed balance and Matthew's queue and evidence plan; it is not a release date or an approval.
