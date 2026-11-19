---
path: Documents/上清保存時間試験/Storage-study/August extract check against July.xlsx
kind: bennett-culture-day.xlsx
author: Anthony Mendoza
derived_from:
---
# Sheet: Culture date

B4: BCE-CE-2025-TBL-006-R00
E4: Anthony Mendoza
B5: 2025-08-12
E5: Eligible count and concentration sum (ng/mL)

A7: No.; B7: Acquisition date; C7: Native eligible n; D7: Extract n; E7: Count difference; F7: Native eligible sum (ng/mL); G7: Extract sum (ng/mL).

| Row | A | B | C | D | E | F | G |
|---:|---:|---|---:|---:|---|---:|---:|
| 8 | 1 | 2025-07-14 | 12 | 12 | `=D8-C8` | 84.57 | 84.57 |
| 9 | 2 | 2025-07-15 | 11 | 11 | `=D9-C9` | 76.81 | 76.81 |
| 10 | 3 | 2025-07-16 | 11 | 11 | `=D10-C10` | 63.57 | 63.57 |
| 11 | Total | | `=SUM(C8:C10)` | `=SUM(D8:D10)` | `=SUM(E8:E10)` | `=SUM(F8:F10)` | `=SUM(G8:G10)` |

# Sheet: Sum differences

B4: BCE-CE-2025-TBL-006-R00
E4: Anthony Mendoza
B5: 2025-08-12
E5: Extract minus native eligible sum (ng/mL)

A7: No.; B7: Acquisition date; C7: Native sum; D7: Extract sum; E7: Signed sum difference; F7: Extract n; G7: Check.

| Row | A | B | C | D | E | F | G |
|---:|---:|---|---|---|---|---|---|
| 8 | 1 | 2025-07-14 | `='Culture date'!F8` | `='Culture date'!G8` | `=D8-C8` | `='Culture date'!D8` | 0 h |
| 9 | 2 | 2025-07-15 | `='Culture date'!F9` | `='Culture date'!G9` | `=D9-C9` | `='Culture date'!D9` | 24 h |
| 10 | 3 | 2025-07-16 | `='Culture date'!F10` | `='Culture date'!G10` | `=D10-C10` | `='Culture date'!D10` | 48 h |
| 11 | Total | | `=SUM(C8:C10)` | `=SUM(D8:D10)` | `=SUM(E8:E10)` | `=SUM(F8:F10)` | Across dates; reconciliation only |

Extract integrity: duplicate native IDs 0; blank readings 0; BN-250715-05 occurrences 0; BN-250716-07 occurrences 0. Independently checked native eligible records against BCE-CE-2025-TBL-004-R01, then compared with the Aug 6 eligible extract. The across-date concentration sum is for reconciliation, not a pooled study mean.
