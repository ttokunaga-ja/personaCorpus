---
path: Google Drive/My Drive/Learner-support/Tuesday duty follow-up.xlsx
kind: elmwood-outcome-reconciliation.xlsx
author: Heather Callahan
derived_from:
---

# Tuesday duty follow-up

Sheet: Duty follow-up

| Cell | Entry |
|---|---|
| A2 | Tuesday duty follow-up |
| C3 | ELM-CRW-2024-REC-002 v1.0 |
| G3 | Heather Callahan |
| C4 | 2024-10-31 (date value, displayed as 31 Oct 2024) |
| G4 | Reading Conditions at Work |
| A5:G5 (merged) | Reception and Learner Support · Session 8 · Print allowance, ELM-CRW-2024-HO-009 v1.0 |

Row 6 contains the seven column headings. The row numbers at left are worksheet addresses. Use Excel dates in B:D and date-time values in E:F, in Toronto local time (EDT). A blank in F means no takeover.

| Row | A: Reference | B: Absence confirmed | C: Session and material matched | D: Due by end of | E: Delivered to learner | F: Arjun took over | G: What was needed to complete delivery |
|---|---|---|---|---|---|---|---|
| 7 | T01 | 2024-10-29 | 2024-10-29 | =B7+1 | 2024-10-30 09:20 | | Complete two-page Session 8 exercise. |
| 8 | T02 | 2024-10-29 | 2024-10-29 | =B8+1 | 2024-10-30 11:05 | | Complete two-page Session 8 exercise. |
| 9 | T03 | 2024-10-29 | 2024-10-29 | =B9+1 | 2024-10-30 14:10 | | Complete two-page Session 8 exercise. |
| 10 | T04 | 2024-10-29 | 2024-10-29 | =B10+1 | 2024-10-31 10:20 | 2024-10-29 16:12 | Kathleen asked Arjun to obtain page 2 and check both large-print pages against v1.0 before delivery. |

| Cell or range | Entry |
|---|---|
| A11 | `=SUMPRODUCT(--(E7:E10>0),--(E7:E10<(D7:D10+1)))` |
| B11:E11 (merged) | Delivered by end of next calendar day |
| F11 | `=COUNT(F7:F10)` |
| G11 | Taken over by Arjun |
| A12:G12 (merged) | Source: duty-handoff.csv. These counts cover only the four Session 8 absences confirmed on 29 Oct. Keep them separate from the 21 Oct review and from any estimate of results in other sessions. |
| A13:G13 (merged) | T04: see Arjun's Large print proof notes.txt (30 Oct, 11:02 EDT). The one-sided copy was held because the learner needed the worked answers on page 2. Both pages of the same version went out on 31 Oct. |

The delivery count uses midnight after the due date as its upper bound, so any recorded time on the due date counts. The takeover count uses only the receipt timestamps in F. With the four records above the two results are 3 and 1.
