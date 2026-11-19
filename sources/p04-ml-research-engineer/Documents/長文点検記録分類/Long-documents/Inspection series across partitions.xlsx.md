---
path: Documents/長文点検記録分類/Long-documents/Inspection series across partitions.xlsx
kind: mercer-adjudication.xlsx
author: Joshua Ramirez
derived_from:
---
# Inspection series across partitions

Sheet: Adjudication

B3: MDR-LDI-20250807-AW-01-R01  
E3: August 7, 2025  
B4: Joshua Ramirez  
E4: 2,400 inspection records; series and input checks for Fatima Rahman

| Row | A: Ref. | B: Check | C: Partitions / field | D: Records or shared series | E: Result | F: Evidence / notes |
|---:|---:|---|---|---:|---|---|
| 6 | 1 | Partition records | Training | 1440 | Fixed | Complete records; series assigned together |
| 7 | 2 | Partition records | Development | 480 | Fixed | Complete records; series assigned together |
| 8 | 3 | Partition records | Evaluation | 480 | Fixed | Complete records; series assigned together |
| 9 |  | Total records | All three | =SUM(D6:D8) | 2,400 | Record counts, not series counts |
| 10 | 4 | Shared series | Training / development | 0 | None | Compare distinct series keys in both partitions, including linked reinspections and corrections |
| 11 | 5 | Shared series | Training / evaluation | 0 | None | Compare distinct series keys in both partitions, including linked reinspections and corrections |
| 12 | 6 | Shared series | Development / evaluation | 0 | None | Compare distinct series keys in both partitions, including linked reinspections and corrections |
| 13 |  | Total pair overlaps | Three pairs | =SUM(D10:D12) | 0 | Pairwise overlap count, not a record count |
| 14 | 7 | Input field check | Series key |  | Excluded | Ledger field used for grouping; not appended to body |
| 15 | 8 | Input field check | Reference class |  | Excluded | Scoring field; not appended to body |
| 16 | 9 | Input field check | Inspection body |  | Included | Opening, observations, additions, final disposition and signature |

The three overlap rows count shared distinct series keys in each pair; zero means no linked original, reinspection or correction series crosses that pair. Blank counts in the field checks are not zero observations. Class support is recorded separately in the August partition counts.
