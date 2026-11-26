---
path: Documents/Night-shift/2024-11-15 Bin returns and saves.xlsx
kind: carver-field-coding.xlsx
author: Brian Chen
derived_from:
---

# Bin returns and saves

B3:D3 — CWR-NS-20241115-DEC-01  
F3 — v1.0  
B4:C4 — 2024-11-15  
E4:F4 — Brian Chen

## Return cues

Header A6:F6. Detail starts at row 7.

| Scene | Jennifer · first pass | Brian · first pass | Agreed cue | First pass match | Observation behind the call |
|---|---|---|---|---|---|
| T1 | Same aisle | Same aisle | Same aisle | =IF(B7=C7,"Agree","Review") | E-14 was saved in E before the move. No Save at F. Back in E, the worker used the open E-14 row before checking E-15. |
| T2 | Same aisle | Same aisle | Same aisle | =IF(B8=C8,"Agree","Review") | F-06 was saved in F and no Save followed at E. Back in F, the worker pointed to F-06 before looking to the shelf labels for F-07. Holding the tablet alone would not qualify. |
| T3 | Other aisle | Other aisle | Other aisle | =IF(B9=C9,"Agree","Review") | At 20:05 the worker was back in E, but the last Save was F-12 in F at 20:02. The worker matched the E shelf label, stock and own tally to resume at E-22. |
| T4 | Same aisle | Unconfirmed | Same aisle | =IF(B10=C10,"Agree","Review") | My first pass left the margin tick unresolved. Jennifer's walk notes put F-19 entry at 20:13 and the E passage at 20:15. At 20:17 the worker used the 20:12 F-18 Save to orient in F. F-19 was recounted at 20:19 and saved at 20:22. |

| Cell | Formula | Label in column B |
|---|---|---|
| A11 | =COUNTIF(E7:E10,"Agree") | First-pass matches |
| A12 | =COUNTIF(D7:D10,"Same aisle") | Same-aisle confirmed bin used |
| A13 | =COUNTIF(D7:D10,"Other aisle") | Last confirmed bin in another aisle |
| A14 | =COUNTA(A7:A10) | Returns observed |

A15:F15 note — Same aisle requires an observed Save before return, a match between its aisle and the return aisle, and evidence the worker used that bin to orient. Other aisle requires a confirmed Save in a different aisle at the point of return. Each row is one return by the same worker with the same tablet. The four rows are scenes across the two November visits.

## Save sequence

Times are local (CST). One row per trip. The last confirmed bin at return is based on the sequence of observed Saves.

| Scene | Return aisle and time | Confirmed bin before move | Entry / Save before and during move | Last confirmed just before return | Resumed check / subsequent Save |
|---|---|---|---|---|---|
| T1 | E · Nov 7 19:58 | E-14 | 19:51 E-14 Save; 19:54 F entrance, no Save there | E-14 · E | 20:00 E-15 check; 20:03 E-15 Save |
| T2 | F · Nov 7 20:16 | F-06 | 20:08 F-06 Save; 20:11 E entrance, no Save there | F-06 · F | 20:18 F-07 check; 20:21 F-07 Save |
| T3 | E · Nov 14 20:05 | E-21 | 19:51 E-21 Save; 19:56 move to F; 20:02 F-12 checked and saved | F-12 · F | 20:08 E-22 check; 20:11 E-22 Save |
| T4 | F · Nov 14 20:17 | F-18 | 20:12 F-18 Save; 20:13 F-19 entry only in F; 20:15 pass E entrance, no Save there | F-18 · F | 20:19 F-19 recount; 20:22 F-19 Save |

Source note — Tablet move event log, Nov 15, checked against Jennifer's Nov 7 and Nov 14 walk notes. Keep Entry and Save separate even when both concern the same row.
