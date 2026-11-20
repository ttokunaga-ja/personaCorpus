---
path: Documents/長文点検記録分類/Cost-discussions/Aug 11 - Matched-volume costs.xlsx
kind: mercer-adjudication.xlsx
author: Brandon Romero
created: 2026-08-07 11:14
derived_from:
---
# Adjudication

A2: Monthly cost — same original-record volume  
B3: MDR-LDI-20260807-AW-01-R02  
E3: August 11, 2026  
B4: Brandon Romero  
E4: Long-document inspection classification; same monthly volume for both conditions

| Row | A: Item | B: Opening only | C: Second read | D: Unit | E: Basis | F: Note |
|---:|---|---:|---:|---|---|---|
| 6 | Common monthly volume | 100000 | =B6 | original records / month | Normal month; enter the shared volume in B6 | Both columns use B6; reruns are not extra receipts |
| 7 | Variable cost per 10,000 originals | =SUM(B8:B10) | =SUM(C8:C10) | USD / 10,000 | Input preparation and measured resource time | Includes both inference passes in second-read hours |
| 8 | Input preparation | 40 | 40 | USD / 10,000 | Common allowance | Same for both conditions |
| 9 | Host | =30*2 | =40*2 | USD / 10,000 | Mean 30 / 40 hours × USD 2 per hour | Three timed runs per condition |
| 10 | Accelerator | =16*12.5 | =24*12.5 | USD / 10,000 | Mean 16 / 24 hours × USD 12.50 per hour | Same equipment and concurrency |
| 11 | Monthly variable cost | =B6/10000*B7 | =C6/10000*C7 | USD / month | Shared original-record volume | Proportional scaling |
| 12 | Standby monitoring | 600 | 600 | USD / month | Monthly fixed allowance | Added once per condition |
| 13 | Estimated monthly total | =SUM(B11:B12) | =SUM(C11:C12) | USD / month | Variable cost plus standby | At 100,000: 3,600 / 4,800 |
| 14 | Ceiling | 4500 | =B14 | USD / month | Application processing cost ceiling | Not a research funding request |
| 15 | Total less ceiling | =B13-B14 | =C13-C14 | USD / month | Negative is below ceiling | At 100,000: -900 / +300 |
| 16 | Increment: input preparation | 0 | =(C8-B8)*$B$6/10000 | USD / month | Difference in preparation | Common allowance |
| 17 | Increment: host | 0 | =(C9-B9)*$B$6/10000 | USD / month | Difference in host | At 100,000: 200 |
| 18 | Increment: accelerator | 0 | =(C10-B10)*$B$6/10000 | USD / month | Difference in accelerator | At 100,000: 1,000 |
| 19 | Increment: standby | 0 | =C12-B12 | USD / month | Difference in fixed allowance | Zero |
| 20 | Total increment | 0 | =SUM(C16:C19) | USD / month | Second read less opening only | At 100,000: 1,200 |
| 21 | Opening plus accelerator increment |  | =B13+C18 | USD / month | Other costs remain at opening-only levels | At 100,000: 4,600; 100 over ceiling |
| 22 | Scenario volume | Opening only | Second read | Unit | Ceiling | Second read less ceiling |
| 23 | 80000 | =$B$7*A23/10000+$B$12 | =$C$7*A23/10000+$C$12 | USD / month | =$B$14 | =C23-E23 |
| 24 | 100000 | =$B$7*A24/10000+$B$12 | =$C$7*A24/10000+$C$12 | USD / month | =$B$14 | =C24-E24 |
| 25 | 120000 | =$B$7*A25/10000+$B$12 | =$C$7*A25/10000+$C$12 | USD / month | =$B$14 | =C25-E25 |

Row 26, A:F: Scenario totals are USD 3,000 / 3,960, 3,600 / 4,800 and 4,200 / 5,640 respectively. The timing sample is separate from the scored evaluation: each of three runs uses the same 10,000 originals, with 2,500 second reads under the proposal. Extrapolation holds that share and the unit costs constant. Unmeasured reruns and human recovery are not included in the estimate or treated as headroom.
