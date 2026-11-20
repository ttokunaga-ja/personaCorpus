---
path: Documents/Equipment/Calibration/October calibration certificate check.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---

# Checking the October certificate figures

## Sheet: Indications
B4: BCE-CE-2024-TBL-004-R00; E4: Melissa Watkins; B5: Oct 22, 2024; E5: Indication and deviation (ng/mL). Prepared for Anthony Mendoza.

| Row | A: No. | B: Phase | C: Reference | D: Indication | E: Reader | F: Source timestamp (EDT) | G: Source |
|---:|---:|---|---:|---:|---|---|---|
| 8 | 1 | as-found | 5.00 | 5.11 | CE-Reader-2 | 2024-10-22 09:42:00 | Calibration indications.csv |
| 9 | 2 | as-found | 5.00 | 5.12 | CE-Reader-2 | 2024-10-22 09:43:00 | Calibration indications.csv |
| 10 | 3 | as-found | 5.00 | 5.13 | CE-Reader-2 | 2024-10-22 09:44:00 | Calibration indications.csv |
| 11 | 4 | as-left | 5.00 | 5.00 | CE-Reader-2 | 2024-10-22 11:16:00 | Calibration indications.csv |
| 12 | 5 | as-left | 5.00 | 5.01 | CE-Reader-2 | 2024-10-22 11:17:00 | Calibration indications.csv |
| 13 | 6 | as-left | 5.00 | 5.02 | CE-Reader-2 | 2024-10-22 11:18:00 | Calibration indications.csv |

## Sheet: Phase check
B4: BCE-CE-2024-TBL-004-R00; E4: Melissa Watkins; B5: Oct 22, 2024; E5: ng/mL. For Anthony Mendoza.

| Row | A: No. | B: Phase | C: Mean indication | D: Reference | E: Indicated minus reference | F: Band result | G: Certificate comparison |
|---:|---:|---|---|---|---|---|---|
| 8 | 1 | as-found | =AVERAGE(Indications!D8:D10) | 5.00 | =C8-D8 | =IF(ABS(E8)<=0.05,"Within","Outside") | 5.12; +0.12, high side |
| 9 | 2 | as-left | =AVERAGE(Indications!D11:D13) | 5.00 | =C9-D9 | =IF(ABS(E9)<=0.05,"Within","Outside") | 5.01; +0.01 |

Management note: MLM-2024-1022-SV-014 records calibration completion Oct 22 at 11:18 EDT. The check band is ±0.05 ng/mL; deviations are indicated minus reference. These are service reference readings, not study results.
