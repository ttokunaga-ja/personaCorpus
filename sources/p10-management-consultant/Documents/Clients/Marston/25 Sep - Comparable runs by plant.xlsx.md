---
path: Documents/Clients/Marston/25 Sep - Comparable runs by plant.xlsx
kind: holt-cost-diagnosis.xlsx
author: Connor Atkinson
derived_from:
---

# Comparable July runs by plant
HRA-MAR-COST-YLD-2024-001 v1.0 · 25 Sep 2024 · Connor Atkinson  
For Nicholas Hargreaves and Siobhan Barrett

## Sheet: Yield comparison

A2: Comparable July runs by plant  
A4: HRA-MAR-COST-YLD-2024-001 v1.0  
D4: 25 Sep 2024  
F4: Connor Atkinson

| Row | A No. | B Site / line | C Material | D Input kg | E Good kg | F Yield | G Gap, points | H Basis |
|---|---:|---|---|---:|---:|---|---|---|
| 7 | 1 | Trafford T2 | B420 / S3 | 30000 | 26400 | =IF(D7>0,E7/D7,"") | | 12 matched July runs; Trafford baseline |
| 8 | 2 | Warrington W1 | B420 / S3 | 30000 | 27600 | =IF(D8>0,E8/D8,"") | =(F8-F7)*100 | 12 matched July runs; 4.0 points above Trafford |
| 9 | | Combined | | =SUM(D7:D8) | =SUM(E7:E8) | =E9/D9 | | 24 runs across the two lines |

The July weight records give 88.0% for Trafford and 92.0% for Warrington. Match input and output by run using Oliver Hewitt's 20 Sep clearing sheet. Use the same board and box specification at both sites. September visit observations serve as process evidence outside these runs.

## Sheet: Opportunity

| Row | A July loss source | B Trafford kg | C Warrington kg | D Difference kg | E Gap, points | F Treatment |
|---|---|---|---|---|---|---|
| 2 | Changeover to first-off approval | 1020 | 600 | =B2-C2 | =D2/'Yield comparison'!D7*100 | 420 kg in the standardisation hypothesis |
| 3 | Jam restart to approval | 780 | 480 | =B3-C3 | =D3/'Yield comparison'!D7*100 | 300 kg in the standardisation hypothesis |
| 4 | Other losses | 1800 | 1320 | =B4-C4 | =D4/'Yield comparison'!D7*100 | 480 kg or 1.6 points outside hypothesis |
| 5 | All losses | =SUM(B2:B4) | =SUM(C2:C4) | =B5-C5 | =D5/'Yield comparison'!D7*100 | Check against the 4.0-point observed gap |
| 6 | Changeover and restart opportunity | | | =SUM(D2:D3) | =D6/'Yield comparison'!D7*100 | 720 kg on 30,000 kg of Trafford input gives 2.4 points |

| Row | A Trafford scenario | B Good weight kg | C Input weight kg | D Yield | E Basis |
|---|---|---|---|---|---|
| 9 | July observed | ='Yield comparison'!E7 | ='Yield comparison'!D7 | =B9/C9 | Cleared July runs |
| 10 | With the two event-window gaps closed | =B9+D6 | =C9 | =B10/C10 | 90.4% at the same input, to be tested |

The first table on this sheet begins with its headings at row 1. The scenario headings are at row 8. Loss allocations come from Deepa Desai's 23 Sep weight reconciliation. If the two event-window losses could be brought to the Warrington level, the same input would yield 27,120 kg of good output at Trafford. The 720 kg is specific to these runs. Assess repeatability before projecting any period saving. The 14 Aug fixed-volume price sheet answers the separate purchasing question. The B420 unit price increase has no place in the yield numerator or denominator.
