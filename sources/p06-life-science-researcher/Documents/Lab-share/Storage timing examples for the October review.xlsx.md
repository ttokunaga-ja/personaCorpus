---
path: Documents/Lab-share/Storage timing examples for the October review.xlsx
kind: bennett-culture-day.xlsx
author: Lauren Brooks
derived_from:
---
# Sheet: Culture date
B4: BCE-CE-2025-TBL-007-R00
E4: Lauren Brooks
B5: Sep 23, 2025
E5: Elapsed storage time, hours

For Melissa: worked administrative examples for the October discussion. The 24 h ceiling and referral route are proposed, pending Robert's decision; current operations have not changed. These are paperwork illustrations, not sample results, and contain no concentration estimates.

Row 7: A = No.; B = Case; C = Elapsed hours; D = Proposed boundary; E = Referral; F = Status; G = Reading of example.

| Row | A | B | C | D | E | F | G |
|---:|---:|---|---:|---|---|---|---|
| 8 | 1 | 23 h 40 min | `=23+40/60` | `=IF(C8="","Unknown",IF(C8<=24,"Within proposed limit","Over proposed limit"))` | `=IF(C8="","Anthony: time undocumented",IF(C8>24,"Anthony: over limit","No timing referral"))` | Pending | Known time within boundary |
| 9 | 2 | 24 h exactly | 24 | `=IF(C9="","Unknown",IF(C9<=24,"Within proposed limit","Over proposed limit"))` | `=IF(C9="","Anthony: time undocumented",IF(C9>24,"Anthony: over limit","No timing referral"))` | Pending | Boundary is inclusive |
| 10 | 3 | 24 h 15 min | `=24+15/60` | `=IF(C10="","Unknown",IF(C10<=24,"Within proposed limit","Over proposed limit"))` | `=IF(C10="","Anthony: time undocumented",IF(C10>24,"Anthony: over limit","No timing referral"))` | Pending | Known excess; refer before routine use |
| 11 | 4 | Unknown | blank | `=IF(C11="","Unknown",IF(C11<=24,"Within proposed limit","Over proposed limit"))` | `=IF(C11="","Anthony: time undocumented",IF(C11>24,"Anthony: over limit","No timing referral"))` | Pending | Missing time is not a known excess; refer before routine use |

Reference: BCE-CE-2025-MIN-005-R00, Sep 18 discussion. A blank C11 is an empty cell, not zero.
