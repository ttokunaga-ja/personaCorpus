---
path: Documents/上清測定再現性試験/Method-comparison/January 16 Caldwell and Bennett comparison.xlsx
kind: bennett-facility-comparison.xlsx
author: Melissa Watkins
derived_from:
---

# January 16 Caldwell and Bennett comparison

## Sheet: Matched samples

- B4: BCE-CE-2025-TBL-003-R00
- E4: Melissa Watkins
- B5: Jan 16, 2025
- E5: ng/mL

Use numeric cells for concentrations, displayed at source precision (two decimals in C and three in E). Both native identifier columns are text. Row 7 contains the column headers.

| Row | A: Pair | B: Bennett record ID | C: Bennett (ng/mL) | D: Caldwell accession | E: Caldwell (ng/mL) | F: Match status | G: Shared label / source reference |
|---:|---:|---|---:|---|---:|---|---|
| 8 | 1 | BN-250106-01 | 6.30 | CA-250107-01 | 6.086 | Matched | MC18-01 · Jan 6 / Jan 7 exports |
| 9 | 2 | BN-250106-02 | 6.87 | CA-250107-02 | 6.242 | Matched | MC18-02 · Jan 6 / Jan 7 exports |
| 10 | 3 | BN-250106-03 | 7.59 | CA-250107-03 | 7.867 | Matched | MC18-03 · Jan 6 / Jan 7 exports |
| 11 | 4 | BN-250106-04 | 4.71 | CA-250107-04 | 5.142 | Matched | MC18-04 · Jan 6 / Jan 7 exports |
| 12 | 5 | BN-250106-05 | 4.67 | CA-250107-05 | 5.117 | Matched | MC18-05 · Jan 6 / Jan 7 exports |
| 13 | 6 | BN-250106-06 | 7.29 | CA-250107-06 | 7.711 | Matched | MC18-06 · Jan 6 / Jan 7 exports |
| 14 | 7 | BN-250106-07 | 5.87 | CA-250107-08 | 5.667 | Matched | MC18-07 · Jan 6 / Jan 7 exports · Price confirmation Jan 9 |
| 15 | 8 | BN-250106-08 | 6.79 | CA-250107-07 | 7.553 | Matched | MC18-08 · Jan 6 / Jan 7 exports · Price confirmation Jan 9 |
| 16 | 9 | BN-250106-09 | 7.36 | CA-250107-09 | 7.203 | Matched | MC18-09 · Jan 6 / Jan 7 exports |
| 17 | 10 | BN-250106-10 | 4.47 | CA-250107-10 | 5.663 | Matched | MC18-10 · Jan 6 / Jan 7 exports |
| 18 | 11 | BN-250106-11 | 5.64 | CA-250107-11 | 5.551 | Matched | MC18-11 · Jan 6 / Jan 7 exports |
| 19 | 12 | BN-250106-12 | 5.36 | CA-250107-12 | 5.184 | Matched | MC18-12 · Jan 6 / Jan 7 exports |
| 20 | 13 | BN-250106-13 | 5.12 | CA-250107-13 | 5.770 | Matched | MC18-13 · Jan 6 / Jan 7 exports |
| 21 | 14 | BN-250106-14 | 7.87 | CA-250107-14 | 7.835 | Matched | MC18-14 · Jan 6 / Jan 7 exports |
| 22 | 15 | BN-250106-15 | 4.92 | CA-250107-15 | 5.132 | Matched | MC18-15 · Jan 6 / Jan 7 exports |
| 23 | 16 | BN-250106-16 | 6.05 | CA-250107-16 | 6.590 | Matched | MC18-16 · Jan 6 / Jan 7 exports |
| 24 | 17 | BN-250106-17 | 5.15 | CA-250107-17 | 4.730 | Matched | MC18-17 · Jan 6 / Jan 7 exports |
| 25 | 18 | BN-250106-18 | 4.63 | CA-250107-18 | 5.713 | Matched | MC18-18 · Jan 6 / Jan 7 exports |

| Cell | Content |
|---|---|
| A26:B26 | Matched pairs |
| C26 | `=COUNTIF(F8:F25,"Matched")` |
| D26 | Comparison summary |
| E26 | `='Pair calculations'!E26` |
| F26 | `='Pair calculations'!E27` |
| G26 | Mean absolute relative difference (E) / inclusive limit (F) |

E26 displays 8.2% to one decimal percentage point. F26 displays 10%. Both retain the underlying ratio values.

- A27:G27: January comparison only. October hold data were excluded and remain under BCE-CE-2024-HLD-001-R00.
- A28:G28: Original files: 2025-01-06 Bennett comparison readings.csv and 2025-01-07 Caldwell comparison readings.csv. Match key: January shared labels and accessions.tsv.
- A29:G29: The Jan 8 positional worksheet flagged two conflicting labels. Angela Price's Jan 9 reply confirmed CA-250107-08 for MC18-07 and CA-250107-07 for MC18-08. The Caldwell export stays in its native accession order.

## Sheet: Pair calculations

- B4: BCE-CE-2025-TBL-003-R00
- E4: Melissa Watkins
- B5: Jan 16, 2025
- E5: ng/mL and relative difference

Row 7 contains the following column headers. Enter the formulas in row 8 and fill through row 25 with relative references. Every calculation row corresponds to the same row on Matched samples. The concentrations and per-pair ratios are numeric and carry full precision.

| Column | Header | Row 8 formula |
|---|---|---|
| A | Shared label | `=LEFT('Matched samples'!G8,7)` |
| B | Bennett (ng/mL) | `='Matched samples'!C8` |
| C | Caldwell (ng/mL) | `='Matched samples'!E8` |
| D | Caldwell minus Bennett (ng/mL) | `=C8-B8` |
| E | Absolute difference relative to Bennett | `=ABS(D8)/B8` |

| Label cell | Label | Value cell | Value or formula |
|---|---|---|---|
| A26 | Mean absolute relative difference | E26 | `=AVERAGE(E8:E25)` |
| A27 | Inclusive limit | E27 | 0.10 |
| A28 | Aggregate criterion | E28 | `=IF(E26<=E27,"Meets criterion","Exceeds criterion")` |
| A29 | Individual pairs above 10% | E29 | `=COUNTIF(E8:E25,">"&E27)` |

A30:E30 — Give each of the 18 paired percentages equal weight and compare their unrounded mean with the inclusive 10% limit in BCE-CE-2024-PLN-003-R01. Individual pair differences are retained for review.
