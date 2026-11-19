---
path: Documents/Equipment/Calibration/CE-Reader-2 calibration history.xlsx
kind: bennett-equipment-review.xlsx
author: Anthony Mendoza
derived_from:
---

# Sheet: Calibration history
A2: CE-Reader-2 — last four calibration events
B4: BCE-CE-2026-TBL-002-R00; E4: Anthony Mendoza
B5: Jan 8, 2026; E5: CE-Reader-2

Serial: LXR-4187. Reference 5.00 ng/mL; signed deviation = indication minus reference. Inclusive acceptance band: −0.05 to +0.05 ng/mL for both as-found and as-left checks.

| Row | A: Visit date | B: Visit type | C: Service record | D: As-found | E: As-left | F: Result | G: Note |
|---|---|---|---:|---:|---|---|
| 8 | Nov 6, 2024 | Supplemental return | MLM-2024-1106-SV-019 | +0.02 | +0.00 | `=IF(AND(ABS(D8)<=0.05,ABS(E8)<=0.05),"In band","Outside band")` | Return after October service |
| 9 | Apr 22, 2025 | Scheduled routine | MLM-2025-0422-SV-012 | -0.03 | +0.01 | `=IF(AND(ABS(D9)<=0.05,ABS(E9)<=0.05),"In band","Outside band")` | Routine visit |
| 10 | Jul 22, 2025 | Supplemental | MLM-2025-0722-SV-026 | +0.04 | +0.01 | `=IF(AND(ABS(D10)<=0.05,ABS(E10)<=0.05),"In band","Outside band")` | Calibration at planned preventive visit |
| 11 | Oct 22, 2025 | Scheduled routine | MLM-2025-1022-SV-041 | +0.03 | -0.01 | `=IF(AND(ABS(D11)<=0.05,ABS(E11)<=0.05),"In band","Outside band")` | Routine visit |

A12:G12: Deviations in ng/mL at the 5.00 ng/mL reference; both values must be within the inclusive ±0.05 ng/mL band. The two supplemental visits are separate from the routine April/October schedule. Earlier context only: Oct 22, 2024 service MLM-2024-1022-SV-014; it is not one of these four events.
