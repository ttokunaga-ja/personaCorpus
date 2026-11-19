---
path: Documents/Clients/Whitfield/Diagnosis/07 Jun - What the event labels actually mean.docx
kind: holt-handoffs.docx
author: Connor Atkinson
derived_from:
---

# What the event labels actually mean

| Document | Date | Prepared by |
|---|---|---|
| HRA-WHI-DIAG-EV-2024-004 | 07 Jun 2024 | Connor Atkinson |

For Nicholas Hargreaves

I checked the event meanings with Neha and the AP records contact. The unit to match is a submission to a particular approver. If the file comes back for evidence, identify the renewed submission before pairing its routing with its review.

## Event pairs for the timing table

| Interval | First event to use | Closing event to use | Substitution and its effect |
|---|---|---|---|
| 03 · Entry work | Entry started at 03 | Entry completed at 03 | Receipt as the opening event would include time before keying, including any completeness check. |
| 07 · Cost-centre wait | Complete package routed at 06 to the responsible cost-centre owner | Cost-centre review starts at 07 | A receipt or notification before formal routing adds time before the package was ready. A later notification can omit some of the wait. Using approval completion adds the period from review start to the recorded decision. |
| 09 · Finance wait | Checked file with cost-centre approval and exception closure routed from 08 | Finance review starts at 09 | The cost-centre approval alone may precede the Finance readiness check and actual hand-off. |
| 12 · Bank release wait | Checked payment run routed after both Treasury checks at 11 | Bank release review starts at 12 | A notification while either check is outstanding adds checking or correction time. A notice that arrives after routing misses the intervening wait. Approval completion adds the review interval. Later bank execution belongs at 13. |

For 12, the two check records establish readiness. Use the associated routing time as the start of the wait if routing happens later. Review start closes the wait and opens the review interval. Completion on its own only establishes that a decision was recorded.

## Read-back against the cleared extract

All times below are JST (+09:00).

| Case | Interval | From | To | Recorded duration or decision |
|---|---|---|---|---|
| WB-0417 | 03 entry work | 14 May 10:10 | 14 May 10:28 | 18 min of entry |
| WB-0417 | Entry end to routing | 14 May 10:28 | 14 May 10:35 | 7 min outside the 07 queue |
| WB-0417 | 07 approval wait | 14 May 10:35 | 15 May 11:05 | 24 h 30 min from complete routing to review start |
| WB-0432 | 07 wait after return | 09 May 15:50 | 10 May 09:10 | 17 h 20 min after the renewed routing. The 11:20 return and 15:40 re-match on 09 May precede the hand-off. |
| WB-0439 | 07 approval wait | 16 May 10:15 | Review start unavailable | Keep the wait unmeasured. The approval completed on 17 May at 11:00, which cannot supply the missing boundary. |

I will carry these definitions into the pair check. The three export cases illustrate how to read the labels. The 09 and 12 instructions above describe which events to seek in the authorised records.

Reference · Japan payment events – cleared extract, 05 Jun 2024. Internal to the Whitfield team.
