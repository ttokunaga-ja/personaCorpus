---
path: Documents/Equipment/Calibration/September hold register.xlsx
kind: bennett-equipment-review.xlsx
author: Lauren Brooks
derived_from:
---

# September hold register

## Sheet: Hold register

A2: September hold register  
B4: BCE-CE-2024-TBL-012-R00 · E4: Lauren Brooks  
B5: Aug 15, 2024 · E5: CE-Reader-2

For Melissa Watkins. Source: CE-Reader-2 reservation diary as updated Aug 15. Includes the three openings from BCE-CE-2024-TBL-009-R00. Use date values in B. Row 7 contains the A:G headings in the order below.

| Row | A Reader | B Date | C Time (EDT) | D Hold owner | E Reservation status | F Decision reference | G Run authorization |
|---:|---|---|---|---|---|---|---|
| 8 | CE-Reader-2 | 2024-09-10 | 09:00–10:30 | Lauren Brooks | Tentative | BCE-CE-2024-DEC-001-R00 | Pending |
| 9 | CE-Reader-2 | 2024-09-12 | 09:00–10:30 | Lauren Brooks | Released Aug 15 | BCE-CE-2024-DEC-001-R00 | Not applicable |
| 10 | CE-Reader-2 | 2024-09-12 | 13:00–14:30 | Lauren Brooks | Tentative | BCE-CE-2024-DEC-001-R00 | Pending |

| Label cell | Label | Count cell | Formula | Expected count |
|---|---|---|---|---:|
| A11 | Tentative rows | C11 | `=COUNTIF(E8:E10,"Tentative")` | 2 |
| D11 | Released rows | F11 | `=COUNTIF(E8:E10,"Released*")` | 1 |

A12 note: Revisit the two retained holds against the reader diary at Robert's Sep 3 decision. Both remain tentative. The run authorization column stays pending until Robert issues a separate decision.
