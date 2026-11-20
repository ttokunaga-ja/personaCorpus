---
path: Documents/Medium-exchange/Medium exchange readings and decision basis.xlsx
kind: bennett-repeatability-summary.xlsx
author: Melissa Watkins
derived_from:
---

# Medium exchange readings and decision basis

## Decision basis

A2: Medium exchange readings and decision basis  
B4: BCE-CE-2026-TBL-008-R00  
E4: Melissa Watkins  
B5: 2026-09-15  
D5: Source CSV  
E5: June readings retained for the September review.csv

| Interval (h) | Basis round | Sample n | Group CV | CV criterion | Disposition | Evidence reference |
|---:|---|---:|---:|---:|---|---|
| 24 | MX-250626 | =COUNT('Reading trace'!F8:F17) | =STDEV.S('Reading trace'!F8:F17)/AVERAGE('Reading trace'!F8:F17) | 15.0% | =IF(D8<=E8,"Within","Above") | Reading trace 8–17, measured Jun 26, 2025 |
| 48 | MX-250626 | =COUNT('Reading trace'!F18:F27) | =STDEV.S('Reading trace'!F18:F27)/AVERAGE('Reading trace'!F18:F27) | 15.0% | =IF(D9<=E9,"Within","Above") | Reading trace 18–27, measured Jun 26, 2025 |
| 72 | MX-250626 | =COUNT('Reading trace'!F28:F37) | =STDEV.S('Reading trace'!F28:F37)/AVERAGE('Reading trace'!F28:F37) | 15.0% | =IF(D10<=E10,"Within",TEXT((D10-E10)*100,"0.0")&" percentage points above") | Reading trace 28–37, measured Jun 26, 2025 |
| 72 | MX-250617 | =COUNT('Reading trace'!F38:F47) | =STDEV.S('Reading trace'!F38:F47)/AVERAGE('Reading trace'!F38:F47) | | Historical comparison only | Reading trace 38–47, measured Jun 17, 2025 |

Row 12: A:B Adopted sample total. C12 `=SUM(C8:C10)`.

Row 13 note for Anthony Mendoza and Robert Nguyen: Use rows 8–10 for the interval decision. Each group must be at or below 15.0% CV. CV is sample standard deviation divided by mean, displayed as a percentage to one decimal. The three repeat CVs display as 8.0%, 12.0%, and 21.0%. The initial 72 h group also displays as 21.0% and is shown separately in row 11. The adopted total is 30 distinct study IDs, one reading per ID.

## Reading trace

A2: Reading trace for the September review  
B4: BCE-CE-2026-TBL-008-R00  
E4: Melissa Watkins  
B5: 2026-09-15  
D5: Source CSV  
E5: June readings retained for the September review.csv

The column headings occupy A7:G7. The adopted block occupies rows 8–37 and the historical comparison block immediately follows in rows 38–47. The block labels below identify the two sets and occupy no worksheet rows. CSV row numbers include the header as row 1. Original reader export row numbers are in the CSV's Original_output_row column.

### Adopted — matched-preparation repeat (rows 8–37)

| Seq. | Interval (h) | Sample ID | Prep. date | Measurement round | Result (ng/mL) | Source row / date |
|---:|---:|---|---|---|---:|---|
| 1 | 24 | MX24-01 | 2025-06-23 | MX-250626 | 10.40 | CSV 2 / 2025-06-26 |
| 2 | 24 | MX24-02 | 2025-06-23 | MX-250626 | 9.08 | CSV 3 / 2025-06-26 |
| 3 | 24 | MX24-03 | 2025-06-23 | MX-250626 | 10.92 | CSV 4 / 2025-06-26 |
| 4 | 24 | MX24-04 | 2025-06-23 | MX-250626 | 9.60 | CSV 5 / 2025-06-26 |
| 5 | 24 | MX24-05 | 2025-06-23 | MX-250626 | 10.13 | CSV 6 / 2025-06-26 |
| 6 | 24 | MX24-06 | 2025-06-23 | MX-250626 | 8.81 | CSV 7 / 2025-06-26 |
| 7 | 24 | MX24-07 | 2025-06-23 | MX-250626 | 10.66 | CSV 8 / 2025-06-26 |
| 8 | 24 | MX24-08 | 2025-06-23 | MX-250626 | 9.87 | CSV 9 / 2025-06-26 |
| 9 | 24 | MX24-09 | 2025-06-23 | MX-250626 | 11.19 | CSV 10 / 2025-06-26 |
| 10 | 24 | MX24-10 | 2025-06-23 | MX-250626 | 9.34 | CSV 11 / 2025-06-26 |
| 11 | 48 | MX48-01 | 2025-06-23 | MX-250626 | 10.99 | CSV 12 / 2025-06-26 |
| 12 | 48 | MX48-02 | 2025-06-23 | MX-250626 | 8.61 | CSV 13 / 2025-06-26 |
| 13 | 48 | MX48-03 | 2025-06-23 | MX-250626 | 10.20 | CSV 14 / 2025-06-26 |
| 14 | 48 | MX48-04 | 2025-06-23 | MX-250626 | 11.78 | CSV 15 / 2025-06-26 |
| 15 | 48 | MX48-05 | 2025-06-23 | MX-250626 | 9.41 | CSV 16 / 2025-06-26 |
| 16 | 48 | MX48-06 | 2025-06-23 | MX-250626 | 10.59 | CSV 17 / 2025-06-26 |
| 17 | 48 | MX48-07 | 2025-06-23 | MX-250626 | 8.22 | CSV 18 / 2025-06-26 |
| 18 | 48 | MX48-08 | 2025-06-23 | MX-250626 | 11.39 | CSV 19 / 2025-06-26 |
| 19 | 48 | MX48-09 | 2025-06-23 | MX-250626 | 9.80 | CSV 20 / 2025-06-26 |
| 20 | 48 | MX48-10 | 2025-06-23 | MX-250626 | 9.01 | CSV 21 / 2025-06-26 |
| 21 | 72 | MX72-01 | 2025-06-23 | MX-250626 | 11.73 | CSV 22 / 2025-06-26 |
| 22 | 72 | MX72-02 | 2025-06-23 | MX-250626 | 8.27 | CSV 23 / 2025-06-26 |
| 23 | 72 | MX72-03 | 2025-06-23 | MX-250626 | 13.12 | CSV 24 / 2025-06-26 |
| 24 | 72 | MX72-04 | 2025-06-23 | MX-250626 | 9.65 | CSV 25 / 2025-06-26 |
| 25 | 72 | MX72-05 | 2025-06-23 | MX-250626 | 7.57 | CSV 26 / 2025-06-26 |
| 26 | 72 | MX72-06 | 2025-06-23 | MX-250626 | 10.35 | CSV 27 / 2025-06-26 |
| 27 | 72 | MX72-07 | 2025-06-23 | MX-250626 | 6.88 | CSV 28 / 2025-06-26 |
| 28 | 72 | MX72-08 | 2025-06-23 | MX-250626 | 12.43 | CSV 29 / 2025-06-26 |
| 29 | 72 | MX72-09 | 2025-06-23 | MX-250626 | 8.96 | CSV 30 / 2025-06-26 |
| 30 | 72 | MX72-10 | 2025-06-23 | MX-250626 | 11.04 | CSV 31 / 2025-06-26 |

### Historical comparison — initial 72 h group (rows 38–47)

| Seq. | Interval (h) | Sample ID | Prep. date | Measurement round | Result (ng/mL) | Source row / date |
|---:|---:|---|---|---|---:|---|
| 31 | 72 | MX72-01 | 2025-06-12 | MX-250617 | 6.36 | CSV 32 / 2025-06-17 |
| 32 | 72 | MX72-02 | 2025-06-12 | MX-250617 | 7.95 | CSV 33 / 2025-06-17 |
| 33 | 72 | MX72-03 | 2025-06-12 | MX-250617 | 8.75 | CSV 34 / 2025-06-17 |
| 34 | 72 | MX72-04 | 2025-06-12 | MX-250617 | 9.32 | CSV 35 / 2025-06-17 |
| 35 | 72 | MX72-05 | 2025-06-12 | MX-250617 | 9.77 | CSV 36 / 2025-06-17 |
| 36 | 72 | MX72-06 | 2025-06-13 | MX-250617 | 10.23 | CSV 37 / 2025-06-17 |
| 37 | 72 | MX72-07 | 2025-06-13 | MX-250617 | 10.45 | CSV 38 / 2025-06-17 |
| 38 | 72 | MX72-08 | 2025-06-13 | MX-250617 | 11.25 | CSV 39 / 2025-06-17 |
| 39 | 72 | MX72-09 | 2025-06-13 | MX-250617 | 12.27 | CSV 40 / 2025-06-17 |
| 40 | 72 | MX72-10 | 2025-06-13 | MX-250617 | 13.64 | CSV 41 / 2025-06-17 |

Row 48 note: Match readings on measurement round and sample ID together. Rows 8–37 are the adopted repeat readings (MX-250626). Rows 38–47 are the initial 72 h comparison readings (MX-250617), which reuse the same ten 72 h labels. Preparation dates and reader headings are reconciled in BCE-CE-2026-TBL-007-R00.
