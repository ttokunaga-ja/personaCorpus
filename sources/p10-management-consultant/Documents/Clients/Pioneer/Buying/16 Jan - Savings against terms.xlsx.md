---
path: Documents/Clients/Pioneer/Buying/16 Jan - Savings against terms.xlsx
kind: holt-pioneer-savings-reconciliation.xlsx
author: Deepa Desai
derived_from:
---
# Savings against terms

Document: HRA-PIO-BUY-SAV-2026-001 | v1.0
Date: 16 Jan 2026 | Prepared by: Deepa Desai | For: Nicholas Hargreaves and Meera Kulkarni

## Sheet: Unit prices

A2: Unit prices; A4: HRA-PIO-BUY-SAV-2026-001 v1.0; D4: 16 Jan 2026; F4: Deepa Desai.

| Row | A: Index | B: Term / supplier | C: Unit | D: Minimum | E: Net quantity | F: Baseline USD | G: Condition USD | H: Applied USD | I: Evidence and application |
|---|---:|---|---|---:|---:|---:|---:|---:|---|
| 7 | 1 | PTD-BUY-TRM-2025-001 Rev. 1 T1 / S018 shipping labels | case | 12000 | 10000 | 20 | 18.40 | 19.60 | 01 Oct–31 Dec; AP-62114–62119 total 10,200 less linked AP-62851 200. Threshold not met; Meera 15 Jan confirms. |
| 8 | 2 | Rev. 1 T2 / S042 sealing tape | case | 7000 | 7500 | 20 | 18 | 18 | 01 Oct–31 Dec; five 150-pack receipts × 10 cases; USD 180/pack. Threshold met. |
| 9 | 3 | Rev. 1 T3 / S036 protective sleeves before 01 Dec | case | | 6250 | 20 | 20 | 20 | 01 Oct–30 Nov receipts, including AP-62418 received 28 Nov, posted 02 Dec. |
| 10 | 4 | Rev. 1 T3 / S036 on or after 01 Dec | case | | 6250 | 20 | 18 | 18 | 01–31 Dec receipts; AP-62431 received 01 Dec. Meera confirms receipt date controls. |
| 11 | 5 | Rev. 1 T4 / S011 bin liners | case | | 5000 | 20 | 19.50 | 19.50 | 01 Oct–31 Dec; additional allowance requires separate approval, not evidenced or posted by cutoff. |
| 12 | 6 | Rev. 1 T5 / S008 packing dispensers | unit | | 10000 | 30 | 27.80 | 27.80 | 01 Oct–31 Dec; fixed price. |

Blank minimum means no minimum, not an unknown value. The 18.40 and 18.00 rates for T1 and T2 apply to all eligible volume only when their respective thresholds are met.

## Sheet: Savings bridge

A2: Savings bridge; A4: HRA-PIO-BUY-SAV-2026-001 v1.0; D4: 16 Jan 2026; F4: Deepa Desai.

| Row | A: Index | B: Condition | C: Baseline USD | D: Expected USD | E: Realised USD | F: Volume threshold USD | G: Other confirmed USD | H: To confirm USD | I: Basis / follow-up |
|---|---:|---|---|---|---|---|---|---|---|
| 7 | 1 | T1 / S018 | ='Unit prices'!E7*'Unit prices'!F7 | ='Unit prices'!E7*('Unit prices'!F7-'Unit prices'!G7) | ='Unit prices'!E7*('Unit prices'!F7-'Unit prices'!H7) | =('Unit prices'!H7-'Unit prices'!G7)*'Unit prices'!E7 | 0 | =D7-E7-F7-G7 | 10,000 × (19.60 − 18.40); actual tier confirmed. |
| 8 | 2 | T2 / S042 | ='Unit prices'!E8*'Unit prices'!F8 | ='Unit prices'!E8*('Unit prices'!F8-'Unit prices'!G8) | ='Unit prices'!E8*('Unit prices'!F8-'Unit prices'!H8) | 0 | 0 | =D8-E8-F8-G8 | Threshold met; 750 packs = 7,500 cases. |
| 9 | 3 | T3 / S036 | =SUM('Unit prices'!E9:E10)*'Unit prices'!F9 | =SUM('Unit prices'!E9:E10)*80%*('Unit prices'!F10-'Unit prices'!G10) | ='Unit prices'!E10*('Unit prices'!F10-'Unit prices'!H10) | 0 | =(SUM('Unit prices'!E9:E10)*80%-'Unit prices'!E10)*('Unit prices'!F10-'Unit prices'!G10) | =D9-E9-F9-G9 | October scenario 80% after switch; actual 6,250 cases after 01 Dec. Timing difference USD 7,500. |
| 10 | 4 | T4 / S011 | ='Unit prices'!E11*'Unit prices'!F11 | ='Unit prices'!E11*('Unit prices'!F11-'Unit prices'!G11+1.10) | ='Unit prices'!E11*('Unit prices'!F11-'Unit prices'!H11) | 0 | 0 | =D10-E10-F10-G10 | USD 5,500 scenario residual; approval and posting not evidenced. Confirmed additional credit amount: blank. |
| 11 | 5 | T5 / S008 | ='Unit prices'!E12*'Unit prices'!F12 | ='Unit prices'!E12*('Unit prices'!F12-'Unit prices'!G12) | ='Unit prices'!E12*('Unit prices'!F12-'Unit prices'!H12) | 0 | 0 | =D11-E11-F11-G11 | Fixed price applied. |
| 12 | Total | | =SUM(C7:C11) | =SUM(D7:D11) | =SUM(E7:E11) | =SUM(F7:F11) | =SUM(G7:G11) | =SUM(H7:H11) | Check: =D12-E12-SUM(F12:H12) |
| 13 | Rates | Q4, tax-exclusive | | =D12/C12 | =E12/C12 | =F12/C12 | =G12/C12 | =H12/C12 | Difference in percentage points: =(D12-E12)/C12*100 |

C12 = 1,000,000; D12 = 81,000 (8.1%); E12 = 56,000 (5.6%); F12 = 12,000; G12 = 7,500; H12 = 5,500; I12 = 0. The 5,500 residual is known; the additional credit amount remains unconfirmed and is not entered as realised.

Sources: 15 Oct - Q4 assumptions as priced.xlsx (80% switch assumption and USD 1.10/case allowance); PTD-BUY-TRM-2025-001 Rev. 1 (terms); Q4 quantities and invoice prices.csv (receipts and postings through 12 Jan); 15 Jan - Meera on returns and dates.eml (return and date confirmation, allowance evidence outstanding). Actual quantities revalue the October assumptions; exclude freight, tax and other items.
