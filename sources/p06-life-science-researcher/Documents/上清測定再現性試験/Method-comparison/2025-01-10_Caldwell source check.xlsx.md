---
path: Documents/上清測定再現性試験/Method-comparison/2025-01-10_Caldwell source check.xlsx
kind: bennett-facility-comparison.xlsx
author: Anthony Mendoza
derived_from:
---

# January comparison source check

## Sheet: Matched samples

- B4: BCE-CE-2025-TBL-002-R00
- E4: Anthony Mendoza
- B5: Jan 10, 2025
- E5: ng/mL

Both native ID columns are text. Concentrations are numeric and retain the two Bennett and three Caldwell decimal places shown.

| Row | A Pair | B Bennett record ID | C Bennett ng/mL | D Caldwell accession | E Caldwell ng/mL | F Source check | G Shared label and check reference |
|---:|---:|---|---:|---|---:|---|---|
| 8 | 1 | BN-250106-01 | 6.30 | CA-250107-01 | 6.086 | Checked | MC18-01 · Both exports |
| 9 | 2 | BN-250106-02 | 6.87 | CA-250107-02 | 6.242 | Checked | MC18-02 · Both exports |
| 10 | 3 | BN-250106-03 | 7.59 | CA-250107-03 | 7.867 | Checked | MC18-03 · Both exports |
| 11 | 4 | BN-250106-04 | 4.71 | CA-250107-04 | 5.142 | Checked | MC18-04 · Both exports |
| 12 | 5 | BN-250106-05 | 4.67 | CA-250107-05 | 5.117 | Checked | MC18-05 · Both exports |
| 13 | 6 | BN-250106-06 | 7.29 | CA-250107-06 | 7.711 | Checked | MC18-06 · Both exports |
| 14 | 7 | BN-250106-07 | 5.87 | CA-250107-08 | 5.667 | Checked | MC18-07 · Price confirmed CA-250107-08 on Jan 9 |
| 15 | 8 | BN-250106-08 | 6.79 | CA-250107-07 | 7.553 | Checked | MC18-08 · Price confirmed CA-250107-07 on Jan 9 |
| 16 | 9 | BN-250106-09 | 7.36 | CA-250107-09 | 7.203 | Checked | MC18-09 · Both exports |
| 17 | 10 | BN-250106-10 | 4.47 | CA-250107-10 | 5.663 | Checked | MC18-10 · Both exports |
| 18 | 11 | BN-250106-11 | 5.64 | CA-250107-11 | 5.551 | Checked | MC18-11 · Both exports |
| 19 | 12 | BN-250106-12 | 5.36 | CA-250107-12 | 5.184 | Checked | MC18-12 · Both exports |
| 20 | 13 | BN-250106-13 | 5.12 | CA-250107-13 | 5.770 | Checked | MC18-13 · Both exports |
| 21 | 14 | BN-250106-14 | 7.87 | CA-250107-14 | 7.835 | Checked | MC18-14 · Both exports |
| 22 | 15 | BN-250106-15 | 4.92 | CA-250107-15 | 5.132 | Checked | MC18-15 · Both exports |
| 23 | 16 | BN-250106-16 | 6.05 | CA-250107-16 | 6.590 | Checked | MC18-16 · Both exports |
| 24 | 17 | BN-250106-17 | 5.15 | CA-250107-17 | 4.730 | Checked | MC18-17 · Both exports |
| 25 | 18 | BN-250106-18 | 4.63 | CA-250107-18 | 5.713 | Checked | MC18-18 · Both exports |

A26:B26 — Pairs checked. C26 — `=COUNTIF(F8:F25,"Checked")`

A27:G27 — Melissa, I traced the IDs through January shared labels and accessions.tsv back to the dated Bennett and Caldwell exports. The Jan 9 confirmation resolves the two order conflicts from the Jan 8 worksheet.

A28:G28 — Source files: 2025-01-06_Bennett readings.csv and 2025-01-07_Caldwell readings.csv. For the two exceptions, see Angela Price's 09:47 EST reply, “Re: Can you check Caldwell accessions 07 and 08.”

## Sheet: Source eligibility

- B4: BCE-CE-2025-TBL-002-R00
- E4: Anthony Mendoza
- B5: Jan 10, 2025
- E5: Dates and counts

| Row | A Shared label | B Bennett source date | C Caldwell source date |
|---:|---|---|---|
| 8 | MC18-01 | 2025-01-06 | 2025-01-07 |
| 9 | MC18-02 | 2025-01-06 | 2025-01-07 |
| 10 | MC18-03 | 2025-01-06 | 2025-01-07 |
| 11 | MC18-04 | 2025-01-06 | 2025-01-07 |
| 12 | MC18-05 | 2025-01-06 | 2025-01-07 |
| 13 | MC18-06 | 2025-01-06 | 2025-01-07 |
| 14 | MC18-07 | 2025-01-06 | 2025-01-07 |
| 15 | MC18-08 | 2025-01-06 | 2025-01-07 |
| 16 | MC18-09 | 2025-01-06 | 2025-01-07 |
| 17 | MC18-10 | 2025-01-06 | 2025-01-07 |
| 18 | MC18-11 | 2025-01-06 | 2025-01-07 |
| 19 | MC18-12 | 2025-01-06 | 2025-01-07 |
| 20 | MC18-13 | 2025-01-06 | 2025-01-07 |
| 21 | MC18-14 | 2025-01-06 | 2025-01-07 |
| 22 | MC18-15 | 2025-01-06 | 2025-01-07 |
| 23 | MC18-16 | 2025-01-06 | 2025-01-07 |
| 24 | MC18-17 | 2025-01-06 | 2025-01-07 |
| 25 | MC18-18 | 2025-01-06 | 2025-01-07 |

Enter source dates as dates. The calculation columns use the following headers in row 7 and formulas in row 8, filled through row 25 with relative references.

| Cell | Header | Row 8 formula |
|---|---|---|
| D7 | Native IDs present | `=COUNTA('Matched samples'!B8,'Matched samples'!D8)` |
| E7 | Key duplicates | `=IF(D8=2,COUNTIF($A$8:$A$25,A8)+COUNTIF('Matched samples'!$B$8:$B$25,'Matched samples'!B8)+COUNTIF('Matched samples'!$D$8:$D$25,'Matched samples'!D8)-3,"")` |
| F7 | Missing readings | `=2-COUNT('Matched samples'!C8,'Matched samples'!E8)` |
| G7 | October source rows | `=COUNTIFS(B8:C8,">="&DATE(2024,10,1),B8:C8,"<"&DATE(2024,11,1))` |

| Summary label | Label cell | Value cell | Formula |
|---|---|---|---|
| Unique paired keys | A26 | D26 | `=COUNTIFS(D8:D25,2,E8:E25,0,A8:A25,"<>")` |
| Missing readings | A27 | F27 | `=SUM(F8:F25)` |
| October source rows | A28 | G28 | `=SUM(G8:G25)` |

A29:G29 — All 18 pairs trace to Jan 6 Bennett and Jan 7 Caldwell records, with both native IDs and numeric readings present. No October readings were used. The 44 results in BCE-CE-2024-HLD-001-R00 remain in the separate hold record.
