---
path: Documents/Clients/Marston/20 Sep - Clearing the July run pairs.xlsx
kind: Working sheet
author: Oliver Hewitt
derived_from:
---

# Clearing the July run pairs
Oliver Hewitt · For Connor Atkinson · 20 Sep 2024

## Sheet: Candidates

Row 1 contains column headings A–J below. Match the input ticket to the good-output record within each July site and line. Include a pair once, with B420 board, S3 box specification and both weights supported. The source references below are the keys for the input and output records at each site.

| Row | A Site | B Input ticket | C Output record | D Board | E Spec | F Input kg | G Good kg | H Source matched | I Include | J Finding |
|---|---|---|---|---|---|---:|---:|---|---|---|
| 2 | Trafford T2 | T01 | TO01 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 3 | Trafford T2 | T02 | TO02 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 4 | Trafford T2 | T03 | TO03 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 5 | Trafford T2 | T04 | TO04 | B420 | S3 | 2600 | 2280 | Yes | Yes | Keep original ticket and its matched output |
| 6 | Trafford T2 | T05 | TO05 | B420 | S3 | 2400 | 2120 | Yes | Yes | Paired |
| 7 | Trafford T2 | T06 | TO06 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 8 | Trafford T2 | T07 | TO07 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 9 | Trafford T2 | T08 | TO08 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 10 | Trafford T2 | T09 | TO09 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 11 | Trafford T2 | T10 | TO10 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 12 | Trafford T2 | T11 | TO11 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 13 | Trafford T2 | T12 | TO12 | B420 | S3 | 2500 | 2200 | Yes | Yes | Paired |
| 14 | Warrington W1 | W01 | WO01 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 15 | Warrington W1 | W02 | WO02 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 16 | Warrington W1 | W03 | WO03 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 17 | Warrington W1 | W04 | WO04 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 18 | Warrington W1 | W05 | WO05 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 19 | Warrington W1 | W06 | WO06 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 20 | Warrington W1 | W07 | WO07 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 21 | Warrington W1 | W08 | WO08 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 22 | Warrington W1 | W09 | WO09 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 23 | Warrington W1 | W10 | WO10 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 24 | Warrington W1 | W11 | WO11 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 25 | Warrington W1 | W12 | WO12 | B420 | S3 | 2500 | 2300 | Yes | Yes | Paired |
| 26 | Trafford T2 | T04 | TO04 | B420 | S3 | 2600 | 2280 | Yes | No | Rework return repeats the ticket and output at row 5; no additional input |
| 27 | Warrington W1 | W13 | WO13 | B510 | S3 | 2400 | 2190 | Yes | No | Thin B510 is a different board grade |
| 28 | Trafford T2 | T13 | | B420 | S3 | 2450 | | No | No | Good-output record outstanding; keep reference and weight blank |

## Sheet: Reconciliation

| Row | A Measure | B Result | C Expected after clearing |
|---|---|---|---|
| 2 | Trafford pairs | =COUNTIFS(Candidates!A2:A28,"Trafford T2",Candidates!I2:I28,"Yes") | 12 |
| 3 | Trafford input kg | =SUMIFS(Candidates!F2:F28,Candidates!A2:A28,"Trafford T2",Candidates!I2:I28,"Yes") | 30000 |
| 4 | Trafford good kg | =SUMIFS(Candidates!G2:G28,Candidates!A2:A28,"Trafford T2",Candidates!I2:I28,"Yes") | 26400 |
| 5 | Warrington pairs | =COUNTIFS(Candidates!A2:A28,"Warrington W1",Candidates!I2:I28,"Yes") | 12 |
| 6 | Warrington input kg | =SUMIFS(Candidates!F2:F28,Candidates!A2:A28,"Warrington W1",Candidates!I2:I28,"Yes") | 30000 |
| 7 | Warrington good kg | =SUMIFS(Candidates!G2:G28,Candidates!A2:A28,"Warrington W1",Candidates!I2:I28,"Yes") | 27600 |
| 8 | Excluded candidates | =COUNTIF(Candidates!I2:I28,"No") | 3 |
| 9 | Included with source match missing | =COUNTIFS(Candidates!I2:I28,"Yes",Candidates!H2:H28,"<>Yes") | 0 |
| 10 | Included with a blank weight | =COUNTIFS(Candidates!I2:I28,"Yes",Candidates!F2:F28,"")+COUNTIFS(Candidates!I2:I28,"Yes",Candidates!G2:G28,"") | 0 |

Connor, please check T04 against the rework return and keep T13 out while the output record is outstanding. If a missing weight is found, check it against the run before revisiting the decision. Keep unknown weights blank. Carry forward the included totals by line for the July comparison.
