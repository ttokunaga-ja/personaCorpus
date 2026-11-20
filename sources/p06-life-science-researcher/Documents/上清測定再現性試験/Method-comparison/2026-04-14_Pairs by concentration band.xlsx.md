---
path: Documents/上清測定再現性試験/Method-comparison/2026-04-14_Pairs by concentration band.xlsx
kind: bennett-facility-comparison.xlsx
author: Melissa Watkins
derived_from: 
---

# April 14 comparison by concentration

Both sheets: B4 BCE-CE-2026-TBL-004-R00; E4 Melissa Watkins; B5 Apr 15, 2026; E5 ng/mL. Each row is matched by shared label against the April shared labels and accessions TSV.

## Matched samples

| Row | A Pair | B Bennett ID | C Bennett ng/mL | D Caldwell ID | E Caldwell ng/mL | F Status | G Shared label |
|---:|---:|---|---:|---|---:|---|---|
| 8 | 1 | BN-260414-01 | 1.87 | CA-260414-01 | 2.52 | Matched | MC30-01 |
| 9 | 2 | BN-260414-02 | 2.04 | CA-260414-02 | 2.68 | Matched | MC30-02 |
| 10 | 3 | BN-260414-03 | 2.13 | CA-260414-03 | 2.82 | Matched | MC30-03 |
| 11 | 4 | BN-260414-04 | 2.21 | CA-260414-04 | 2.98 | Matched | MC30-04 |
| 12 | 5 | BN-260414-05 | 2.29 | CA-260414-05 | 3.11 | Matched | MC30-05 |
| 13 | 6 | BN-260414-06 | 2.35 | CA-260414-06 | 3.15 | Matched | MC30-06 |
| 14 | 7 | BN-260414-07 | 2.48 | CA-260414-07 | 3.35 | Matched | MC30-07 |
| 15 | 8 | BN-260414-08 | 2.63 | CA-260414-08 | 3.39 | Matched | MC30-08 |
| 16 | 9 | BN-260414-09 | 3.37 | CA-260414-09 | 3.60 | Matched | MC30-09 |
| 17 | 10 | BN-260414-10 | 3.48 | CA-260414-10 | 3.75 | Matched | MC30-10 |
| 18 | 11 | BN-260414-11 | 3.62 | CA-260414-11 | 3.90 | Matched | MC30-11 |
| 19 | 12 | BN-260414-12 | 3.78 | CA-260414-12 | 4.05 | Matched | MC30-12 |
| 20 | 13 | BN-260414-13 | 3.94 | CA-260414-13 | 4.20 | Matched | MC30-13 |
| 21 | 14 | BN-260414-14 | 4.03 | CA-260414-14 | 4.12 | Matched | MC30-14 |
| 22 | 15 | BN-260414-15 | 4.12 | CA-260414-15 | 4.28 | Matched | MC30-15 |
| 23 | 16 | BN-260414-16 | 4.21 | CA-260414-16 | 4.42 | Matched | MC30-16 |
| 24 | 17 | BN-260414-17 | 4.36 | CA-260414-17 | 4.58 | Matched | MC30-17 |
| 25 | 18 | BN-260414-18 | 4.45 | CA-260414-18 | 4.64 | Matched | MC30-18 |
| 26 | 19 | BN-260414-19 | 4.58 | CA-260414-19 | 4.66 | Matched | MC30-19 |
| 27 | 20 | BN-260414-20 | 4.66 | CA-260414-20 | 4.80 | Matched | MC30-20 |
| 28 | 21 | BN-260414-21 | 6.08 | CA-260414-21 | 6.18 | Matched | MC30-21 |
| 29 | 22 | BN-260414-22 | 6.21 | CA-260414-22 | 6.30 | Matched | MC30-22 |
| 30 | 23 | BN-260414-23 | 6.32 | CA-260414-23 | 6.42 | Matched | MC30-23 |
| 31 | 24 | BN-260414-24 | 6.39 | CA-260414-24 | 6.45 | Matched | MC30-24 |
| 32 | 25 | BN-260414-25 | 6.48 | CA-260414-25 | 6.61 | Matched | MC30-25 |
| 33 | 26 | BN-260414-26 | 6.55 | CA-260414-26 | 6.74 | Matched | MC30-26 |
| 34 | 27 | BN-260414-27 | 6.67 | CA-260414-27 | 6.76 | Matched | MC30-27 |
| 35 | 28 | BN-260414-28 | 6.83 | CA-260414-28 | 6.97 | Matched | MC30-28 |
| 36 | 29 | BN-260414-29 | 7.17 | CA-260414-29 | 7.05 | Matched | MC30-29 |
| 37 | 30 | BN-260414-30 | 6.80 | CA-260414-30 | 6.62 | Matched | MC30-30 |

Row 38: A:B All pairs; C =COUNTIF(F8:F37,"Matched"); E =AVERAGE(E8:E37)-AVERAGE(C8:C37); G Signed mean difference, Caldwell minus Bennett, ng/mL. Result n=30, +0.30 ng/mL.

## By concentration band

| Row | A Band | B Bennett interval (ng/mL) | C Matched pairs | D Bennett mean | E Caldwell mean | F Mean difference | G Review status |
|---:|---|---|---|---|---|---|---|
| 8 | Low | <3.00 | =COUNTIF('Matched samples'!C8:C37,"<3") | =AVERAGEIF('Matched samples'!C8:C37,"<3",'Matched samples'!C8:C37) | =AVERAGEIF('Matched samples'!C8:C37,"<3",'Matched samples'!E8:E37) | =E8-D8 | Follow-up pending |
| 9 | Middle | ≥3.00 and <5.00 | =COUNTIFS('Matched samples'!C8:C37,">=3",'Matched samples'!C8:C37,"<5") | =AVERAGEIFS('Matched samples'!C8:C37,'Matched samples'!C8:C37,">=3",'Matched samples'!C8:C37,"<5") | =AVERAGEIFS('Matched samples'!E8:E37,'Matched samples'!C8:C37,">=3",'Matched samples'!C8:C37,"<5") | =E9-D9 | Descriptive |
| 10 | High | ≥5.00 | =COUNTIF('Matched samples'!C8:C37,">=5") | =AVERAGEIF('Matched samples'!C8:C37,">=5",'Matched samples'!C8:C37) | =AVERAGEIF('Matched samples'!C8:C37,">=5",'Matched samples'!E8:E37) | =E10-D10 | Descriptive |
| 11 | All pairs | | =COUNT('Matched samples'!C8:C37) | =AVERAGE('Matched samples'!C8:C37) | =AVERAGE('Matched samples'!E8:E37) | =E11-D11 | All 30 pairs |

Expected band counts 8, 12, 10; Bennett means 2.25, 4.05, 6.55; Caldwell means 3.00, 4.25, 6.61; signed means +0.75, +0.20, +0.06 ng/mL. Overall +0.30 ng/mL is calculated from all 30 pairs, not the simple average of band means. Differences are Caldwell minus Bennett. These descriptive results do not establish equivalence.

