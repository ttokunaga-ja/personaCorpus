---
path: Documents/上清保存時間試験/Storage-study/Storage time group means and baseline.xlsx
kind: bennett-storage-summary.xlsx
author: Melissa Watkins
derived_from:
---
# Storage time group means and baseline

For Anthony Mendoza and Robert Nguyen. BCE-CE-2025-TBL-005-R00; Aug 7, 2025; Melissa Watkins. Source: July eligible storage readings.csv, Aug 6, 2025.

## Sheet: Group summary
B4: BCE-CE-2025-TBL-005-R00; E4: Melissa Watkins; B5: Aug 7, 2025; E5: July eligible storage readings.csv

Row 7: A Storage time (h); B Planned n; C Eligible n; D Mean (ng/mL); E Ratio to 0 h; F Change from 0 h; G Note.

| Row | A | B | C | D | E | F | G |
|---:|---:|---:|---|---|---|---|---|
| 8 | 0 | 12 | `=COUNTIF('Eligible readings'!$C$8:$C$41,A8)` | `=IF(C8=0,"",AVERAGEIF('Eligible readings'!$C$8:$C$41,A8,'Eligible readings'!$E$8:$E$41))` | `=IF(OR(C8=0,$C$8=0,$D$8=0),"",D8/$D$8)` | `=IF(ISNUMBER(E8),E8-1,"")` | Baseline |
| 9 | 24 | 12 | `=COUNTIF('Eligible readings'!$C$8:$C$41,A9)` | `=IF(C9=0,"",AVERAGEIF('Eligible readings'!$C$8:$C$41,A9,'Eligible readings'!$E$8:$E$41))` | `=IF(OR(C9=0,$C$8=0,$D$8=0),"",D9/$D$8)` | `=IF(ISNUMBER(E9),E9-1,"")` | Eligible records |
| 10 | 48 | 12 | `=COUNTIF('Eligible readings'!$C$8:$C$41,A10)` | `=IF(C10=0,"",AVERAGEIF('Eligible readings'!$C$8:$C$41,A10,'Eligible readings'!$E$8:$E$41))` | `=IF(OR(C10=0,$C$8=0,$D$8=0),"",D10/$D$8)` | `=IF(ISNUMBER(E10),E10-1,"")` | Eligible records |

Row 11: A:B Total eligible; C11 `=SUM(C8:C10)` (34).

Expected displayed means 7.0475, 6.9827 and 5.7791 ng/mL; ratios 100.0%, 99.1% and 82.0%; signed changes 0.0%, −0.9% and −18.0%. Ratios use unrounded means. Two identity-disputed records are excluded under BCE-CE-2025-DEV-001-R00; no values were substituted.

## Sheet: Eligible readings
B4: BCE-CE-2025-TBL-005-R00; E4: Melissa Watkins; B5: Aug 7, 2025; E5: July eligible storage readings.csv

Row 7: A No.; B Native ID; C Storage time (h); D Matched label; E Concentration (ng/mL); F Measurement date; G Handoff record.

| Row | A | B | C | D | E | F | G |
|---:|---:|---|---:|---|---:|---|---|
| 8 | 1 | BN-250714-01 | 0 | ST00-01 | 7.12 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 9 | 2 | BN-250714-02 | 0 | ST00-02 | 6.84 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 10 | 3 | BN-250714-03 | 0 | ST00-03 | 7.25 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 11 | 4 | BN-250714-04 | 0 | ST00-04 | 6.92 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 12 | 5 | BN-250714-05 | 0 | ST00-05 | 7.08 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 13 | 6 | BN-250714-06 | 0 | ST00-06 | 6.77 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 14 | 7 | BN-250714-07 | 0 | ST00-07 | 7.31 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 15 | 8 | BN-250714-08 | 0 | ST00-08 | 6.98 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 16 | 9 | BN-250714-09 | 0 | ST00-09 | 7.16 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 17 | 10 | BN-250714-10 | 0 | ST00-10 | 6.89 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 18 | 11 | BN-250714-11 | 0 | ST00-11 | 7.04 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 19 | 12 | BN-250714-12 | 0 | ST00-12 | 7.21 | 2025-07-14 | BCE-CE-2025-LAB-003-R00 |
| 20 | 13 | BN-250715-01 | 24 | ST24-01 | 6.92 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 21 | 14 | BN-250715-02 | 24 | ST24-02 | 7.13 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 22 | 15 | BN-250715-03 | 24 | ST24-03 | 6.74 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 23 | 16 | BN-250715-04 | 24 | ST24-04 | 7.06 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 24 | 17 | BN-250715-06 | 24 | ST24-06 | 7.20 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 25 | 18 | BN-250715-07 | 24 | ST24-07 | 6.97 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 26 | 19 | BN-250715-08 | 24 | ST24-08 | 6.81 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 27 | 20 | BN-250715-09 | 24 | ST24-09 | 7.09 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 28 | 21 | BN-250715-10 | 24 | ST24-10 | 6.95 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 29 | 22 | BN-250715-11 | 24 | ST24-11 | 7.15 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 30 | 23 | BN-250715-12 | 24 | ST24-12 | 6.79 | 2025-07-15 | BCE-CE-2025-LAB-004-R00 |
| 31 | 24 | BN-250716-01 | 48 | ST48-01 | 5.81 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 32 | 25 | BN-250716-02 | 48 | ST48-02 | 5.65 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 33 | 26 | BN-250716-03 | 48 | ST48-03 | 5.92 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 34 | 27 | BN-250716-04 | 48 | ST48-04 | 5.74 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 35 | 28 | BN-250716-05 | 48 | ST48-05 | 5.89 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 36 | 29 | BN-250716-06 | 48 | ST48-06 | 5.70 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 37 | 30 | BN-250716-08 | 48 | ST48-08 | 5.83 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 38 | 31 | BN-250716-09 | 48 | ST48-09 | 5.69 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 39 | 32 | BN-250716-10 | 48 | ST48-10 | 5.76 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 40 | 33 | BN-250716-11 | 48 | ST48-11 | 5.88 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |
| 41 | 34 | BN-250716-12 | 48 | ST48-12 | 5.70 | 2025-07-16 | BCE-CE-2025-LAB-005-R00 |

Row 42: Eligible records only; omissions per BCE-CE-2025-DEV-001-R00. Original exports remain unchanged.
