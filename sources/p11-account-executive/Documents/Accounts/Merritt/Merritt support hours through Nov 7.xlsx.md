---
path: Documents/Accounts/Merritt/Merritt support hours through Nov 7.xlsx
kind: halstead-adoption.xlsx
author: Darren Vasquez
derived_from:
---
# Sheet: Support hours

A3: Merritt — recorded support through November 7
C4: HBC-MER-20251113-SVC-01
C5: November 13, 2025
C6: Darren Vasquez

| Row | A: Date | B: Work | C: Hours | D: Period tag | E: Record |
|---|---|---|---:|---|---|
| 8 | Date | Work | Hours | Period tag | Record |
| 9 | 2025-01-16 | Administrator consultation | 3 | Regular | Administrator questions |
| 10 | 2025-03-12 | Workflow organization | 4 | Regular | Workflow review |
| 11 | 2025-05-08 | Staff transition handoff | 4 | Regular | Handoff session |
| 12 | 2025-07-17 | Administrator consultation | 3 | Regular | Administrator review |
| 13 | 2025-09-09 | Workflow organization | 3 | Busy week | Workflow session |
| 14 | 2025-09-10 | Administrator consultation | 2 | Busy week | Administrator questions |
| 15 | 2025-09-11 | Staff transition handoff | 4 | Busy week | Transition session |
| 16 | 2025-10-16 | Workflow organization | 3 | Regular | Follow-up review |
| 17 | 2025-11-07 | Administrator consultation | 2 | Regular | Administrator questions |
| 18 | | Total recorded | =SUM(C9:C17) | | 28 hours |
| 19 | | Busy week within total | =SUMIF(D9:D17,"Busy week",C9:C17) | | 9 hours, included in row 18 |
| 20 | | Annual allowance | 40 | | Hours |
| 21 | | Balance at cutoff | =C20-C18 | | 12 hours |

Category checks: administrator consultation =SUMIF(B9:B17,"Administrator consultation",C9:C17) (10 hours); workflow organization =SUMIF(B9:B17,"Workflow organization",C9:C17) (10 hours); staff transition handoff =SUMIF(B9:B17,"Staff transition handoff",C9:C17) (8 hours).

Only work recorded through November 7 is included. Reservations not yet performed are not recorded hours and are not deducted from this balance; check upcoming reservations separately before offering dates. Prepared for Michelle.
