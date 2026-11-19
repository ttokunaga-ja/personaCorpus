---
path: Documents/Accounts/Hollis/March pilot - review counts.xlsx
kind: halstead-adoption.xlsx
author: Michelle Whitaker
derived_from:
---

# Sheet: Department Adoption

- A3: Hollis pilot — weekly review counts
- C4: HBC-HOL-20250318-ADP-01 v1.0
- C5: March 18, 2025
- C6: Michelle Whitaker

| Row | A: No. | B: Pilot cohort | C: Pilot seats | D: Weekly active | E: Utilization | F: Review note |
|---|---:|---|---:|---:|---|---|
| 9 | 1 | Hollis one-site pilot, Mar 10–16 | =COUNTA('Seat detail'!A2:A31) | =COUNTIF('Seat detail'!D2:D31,1) | =D9/C9 | Six seats had no qualifying send; ask shift leads. |

A10:F10: Week of March 10–16, 2025, Denver local dates. A seat is active once if it sent at least one operational branch notice or handoff; sign-ins, views and practice sends are excluded. Six seats had no qualifying send; the reason is not established. Source: HMS-IT-20250318-01 v1.0 and Hollis pilot activity - week of Mar 10.csv. The count describes the 30-seat pilot, not production usage.

# Sheet: Seat detail

Each row is one roster seat from Hollis pilot activity - week of Mar 10.csv, for March 10–16. D is 1 when either operational send count is positive, otherwise 0. No duplicate seat IDs. The expected values are 30 seats, 24 active, six with no qualifying send, and 80.0% on the main sheet.

| Row | A: Seat ID | B: Operational branch notices | C: Operational handoffs | D: Weekly active (1/0) |
|---:|---|---:|---:|---|
| 2 | P01 | 2 | 1 | =IF(OR(B2>0,C2>0),1,0) |
| 3 | P02 | 0 | 2 | =IF(OR(B3>0,C3>0),1,0) |
| 4 | P03 | 1 | 0 | =IF(OR(B4>0,C4>0),1,0) |
| 5 | P04 | 3 | 1 | =IF(OR(B5>0,C5>0),1,0) |
| 6 | P05 | 0 | 1 | =IF(OR(B6>0,C6>0),1,0) |
| 7 | P06 | 2 | 0 | =IF(OR(B7>0,C7>0),1,0) |
| 8 | P07 | 1 | 1 | =IF(OR(B8>0,C8>0),1,0) |
| 9 | P08 | 0 | 3 | =IF(OR(B9>0,C9>0),1,0) |
| 10 | P09 | 2 | 2 | =IF(OR(B10>0,C10>0),1,0) |
| 11 | P10 | 1 | 0 | =IF(OR(B11>0,C11>0),1,0) |
| 12 | P11 | 0 | 1 | =IF(OR(B12>0,C12>0),1,0) |
| 13 | P12 | 1 | 2 | =IF(OR(B13>0,C13>0),1,0) |
| 14 | P13 | 2 | 0 | =IF(OR(B14>0,C14>0),1,0) |
| 15 | P14 | 0 | 1 | =IF(OR(B15>0,C15>0),1,0) |
| 16 | P15 | 1 | 1 | =IF(OR(B16>0,C16>0),1,0) |
| 17 | P16 | 2 | 0 | =IF(OR(B17>0,C17>0),1,0) |
| 18 | P17 | 0 | 2 | =IF(OR(B18>0,C18>0),1,0) |
| 19 | P18 | 1 | 0 | =IF(OR(B19>0,C19>0),1,0) |
| 20 | P19 | 3 | 1 | =IF(OR(B20>0,C20>0),1,0) |
| 21 | P20 | 0 | 1 | =IF(OR(B21>0,C21>0),1,0) |
| 22 | P21 | 1 | 1 | =IF(OR(B22>0,C22>0),1,0) |
| 23 | P22 | 2 | 0 | =IF(OR(B23>0,C23>0),1,0) |
| 24 | P23 | 0 | 2 | =IF(OR(B24>0,C24>0),1,0) |
| 25 | P24 | 1 | 0 | =IF(OR(B25>0,C25>0),1,0) |
| 26 | P25 | 0 | 0 | =IF(OR(B26>0,C26>0),1,0) |
| 27 | P26 | 0 | 0 | =IF(OR(B27>0,C27>0),1,0) |
| 28 | P27 | 0 | 0 | =IF(OR(B28>0,C28>0),1,0) |
| 29 | P28 | 0 | 0 | =IF(OR(B29>0,C29>0),1,0) |
| 30 | P29 | 0 | 0 | =IF(OR(B30>0,C30>0),1,0) |
| 31 | P30 | 0 | 0 | =IF(OR(B31>0,C31>0),1,0) |
