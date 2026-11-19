---
path: Documents/Clients/Marston/25 Sep - Feeder replacement economics.xlsx
kind: holt-cost-diagnosis.xlsx
author: Deepa Desai
derived_from:
---

## Sheet: Equipment payback

A2: Trafford T2 — Feeder replacement economics  
A4: HRA-MAR-COST-CAP-2024-001 v1.0  
D4: 25 Sep 2024  
F4: Deepa Desai  
A5: Marston Carton Packaging Ltd · Internal review for Nicholas Hargreaves and Elizabeth Hargreaves

Row 6 contains the column headings.

| Row | A No. | B Cost or benefit | C Initial GBP | D Annual gross GBP | E Annual added cost GBP | F Annual net GBP | G Payback years | H Basis |
|---:|---:|---|---:|---:|---:|---:|---:|---|
| 7 | 1 | Replacement feeder | 326000 | | | | | Trafford T2 only |
| 8 | 2 | Installation | 38000 | | | | | One-off |
| 9 | 3 | Start-up validation | 20000 | | | | | One-off |
| 10 | 4 | Board recovered | | =D29*D28 | | | | Modelled annual kg × additional recovery after standardisation × fixed GBP/kg |
| 11 | 5 | Outsourcing avoided | | 14400 | | | | Annual saving assumed |
| 12 | 6 | Overtime avoided | | 9000 | | | | Annual saving assumed |
| 13 | 7 | Additional maintenance | | | 13000 | | | Recurring annual cost |
| 14 | | Base case | =SUM(C7:C9) | =SUM(D10:D12) | =E13 | =D14-E14 | =IF(AND(C14>0,F14>0),C14/F14,"") | Simple, undiscounted payback on annual net benefit |
| 15 | | Six-year requirement | =C14 | | | =C15/G15 | 6 | Net benefit required per year |
| 16 | | Annual net shortfall to six years | | | | =F15-F14 | | Additional annual net benefit needed at the same outlay |
| 17 | | Gross benefit shortfall | | 12000 | | | | Annual reduction applied in the downside |
| 18 | | Downside case | =C14 | =D14-D17 | =E14 | =D18-E18 | =IF(AND(C18>0,F18>0),C18/F18,"") | Same initial outlay and maintenance |

A19: Board throughput and recovery assumptions

Row 20 contains the source table headings. The four ledger periods begin at row 21.

| Row | B Period or input | D Value | H Unit and basis |
|---:|---|---:|---|
| 21 | Aug–Oct 2023 | 952000 | kg net B420 fed |
| 22 | Nov 2023–Jan 2024 | 1067000 | kg net B420 fed |
| 23 | Feb–Apr 2024 | 987000 | kg net B420 fed |
| 24 | May–Jul 2024 | 1012000 | kg net B420 fed |
| 25 | Observed 12 months to 31 Jul 2024 | =SUM(D21:D24) | kg, four ledger periods |
| 26 | Modelled annual input | =ROUND(D25,-5) | kg/year, nearest 100,000 kg, assuming comparable T2 throughput next year |
| 27 | Additional equipment recovery | 2.0% | Assumption after standardisation, to be verified |
| 28 | Fixed material rate | 0.62 | GBP/kg, June B420 basis from the 14 Aug fixed-volume price check |
| 29 | Additional board recovered | =D26*D27 | kg/year, modelled |

Include the source table and notes in the Equipment payback print range with the case above (A1:H33).

- **Row 30 · Source and scope.** Caroline Wood’s email of 06 Aug 2024, “Marston board costs and current work standard”, gives the figures in the message body. She extracted them on 06 Aug from the Trafford T2 production issues and returns ledger. The figures cover net B420 fed through the T2 feeder across all box specifications. Returns are deducted and re-fed board is counted once.
- **Row 31 · Comparison with other records.** The Jun–Jul purchase CSV records bought kg at Trafford and Warrington under one price contract. It omits other contracts and does not measure feed to T2. The 30,000 kg of matched July B420 / S3 runs at T2 is a subset of T2 throughput. Warrington’s matching 30,000 kg is outside the equipment case.
- **Row 32 · Benefit boundary.** Assume a comparable year of T2 operation and 2.0% additional recovery attributable to the feeder after common standards are in place. Exclude the 720 kg changeover and restart opportunity in Connor’s July comparison (HRA-MAR-COST-YLD-2024-001 v1.0). Value additional board at the fixed June rate.
- **Row 33 · For review.** The base case gives 6.4 years against six years at £64,000 net per year. A £12,000 fall in annual gross benefit gives 8.0 years. Before relying on the case for a quotation decision, confirm next year’s T2 throughput, measure residual recoverable loss after standardisation and check achievable outsourcing and overtime savings and the maintenance allowance.
