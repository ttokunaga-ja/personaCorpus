---
path: Documents/長文点検記録分類/Cost-discussions/Aug 7 monthly scratch.xlsx
kind: mercer-adjudication.xlsx
author: Brandon Romero
derived_from:
---
# Adjudication

A2: Monthly cost scratch — initial volume assumptions  
B3: MDR-LDI-20260807-AW-01-R01  
E3: August 7, 2026  
B4: Brandon Romero  
E4: Long-document inspection classification; preliminary monthly estimate

| Row | A: Item | B: Opening only | C: Second read | D: Unit | E: Basis | F: Note |
|---:|---|---:|---:|---|---|---|
| 6 | Original records per month | 80000 | 60000 | original records | 80,000: another use; 60,000: provisional daytime allocation | Different workloads; both provisional |
| 7 | Variable cost per 10,000 originals | =B8+B9+B10 | =C8+C9+C10 | USD | Inputs and rates: Brandon's working cost assumptions, rows 8–10; hours: Timed passes on the same intake mix | Both passes included in second-read hours |
| 8 | Input preparation | 40 | 40 | USD / 10,000 originals | Brandon's input preparation allowance | Common assumption |
| 9 | Host | =30*2 | =40*2 | USD / 10,000 originals | Mean active hours 30 / 40 from Timed passes; Brandon's working host rate USD 2 / hour | Three runs per condition |
| 10 | Accelerator | =16*12.5 | =24*12.5 | USD / 10,000 originals | Mean active hours 16 / 24 from Timed passes; Brandon's working accelerator rate USD 12.50 / hour | Same equipment and concurrency |
| 11 | Monthly variable cost | =B6/10000*B7 | =C6/10000*C7 | USD / month | Provisional volumes in row 6 | Not a like-for-like comparison |
| 12 | Standby monitoring | 600 | 600 | USD / month | Brandon's monthly standby allowance | Once per month in either column |
| 13 | Estimated monthly total | =SUM(B11:B12) | =SUM(C11:C12) | USD / month | Rows 11–12 | 3,000 / 3,120 at the different provisional volumes |

Row 14, A:F: The timed runs use 10,000 original records each; the second-read runs include 2,500 reruns. Reruns add resource time, not original records. The preparation allowance and hourly rates are working cost inputs, not measurements from the timing file. Align the monthly volumes before comparing totals.
