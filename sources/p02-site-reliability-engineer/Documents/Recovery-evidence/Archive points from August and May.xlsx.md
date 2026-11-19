---
path: Documents/Recovery-evidence/Archive points from August and May.xlsx
kind: brt-recovery-point-comparison.xlsx
author: Daniel Jennings
derived_from:
---

# Recovery point comparison

## Recovery points

B3: BRT-RP-20260511-001  
E3: Daniel Jennings  
B4: 11 May 2026  
E4: Ticket archive checks — 19 August 2024 and 11 May 2026  
For: Sanjay Mehta and Anjali Shah

| Row | A: No. | B: Reference (UTC) | C: Coverage point (UTC) | D: Age (minutes) | E: Limit (minutes) | F: Result |
|---|---:|---|---|---|---:|---|
| 7 | 1 | 2024-08-19 10:00 | 2024-08-19 09:38 | `=ROUND((B7-C7)*1440,0)` | 15 | 22 minutes; 7 over limit |
| 8 | 2 | 2026-05-11 10:00 | 2026-05-11 09:51 | `=ROUND((B8-C8)*1440,0)` | 15 | 9 minutes; 6 inside limit |
| 9 | Maximum observed age |  |  | `=MAX(D7:D8)` |  | Two verified exercise checks only |

A10:F10 note: The May point is 13 minutes younger than the August point. This compares observed point ages, not service recovery times. The maximum is restricted to these two checks; neither the cause of the difference nor a bound for future points is established. Both points precede their reference; eligibility was confirmed separately from coverage. August completion was 09:44 UTC and May completion 09:55 UTC.
