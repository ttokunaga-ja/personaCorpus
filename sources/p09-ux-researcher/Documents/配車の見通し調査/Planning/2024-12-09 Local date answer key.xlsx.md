---
path: Documents/配車の見通し調査/Planning/2024-12-09 Local date answer key.xlsx
kind: carver-local-date-key.xlsx
author: Jennifer Salazar
derived_from:
---

# Local date answer key
CWR-DR-20241209-DEC-01 | v1.0 | December 9, 2024 | Jennifer Salazar

## Sheet: Local date key
B–D are Excel date-time values, local to each named location. Dates are compared, not elapsed times.

| Row | A Route | B Departs local | C Window opens local | D Window closes local | E Local date check | F Time zones and answer basis |
|---|---|---|---|---|---|---|
| 7 | R-611 | 2024-12-05 22:30 | 2024-12-06 06:00 | 2024-12-06 07:00 | =IF(INT(C7)<>INT(D7),"Window crosses midnight",IF(INT(C7)=INT(B7)+1,"Next-day start",IF(INT(C7)=INT(B7),"Same-day start","Other"))) | Chicago CST to Detroit EST; Oakridge ODC-DSP-20241202-REF-01 R-611. December 6, 06:00–07:00; R-614 has identical clock times on December 13. Times alone are not the answer. |
| 8 | R-612 | 2024-12-05 23:10 | 2024-12-06 05:45 | 2024-12-06 06:30 | =IF(INT(C8)<>INT(D8),"Window crosses midnight",IF(INT(C8)=INT(B8)+1,"Next-day start",IF(INT(C8)=INT(B8),"Same-day start","Other"))) | Chicago CST to Toledo EST; Oakridge ODC-DSP-20241202-REF-01 R-612. December 6, 05:45–06:30. |
| 9 | R-613 | 2024-12-06 00:20 | 2024-12-06 04:30 | 2024-12-06 05:00 | =IF(INT(C9)<>INT(D9),"Window crosses midnight",IF(INT(C9)=INT(B9)+1,"Next-day start",IF(INT(C9)=INT(B9),"Same-day start","Other"))) | Detroit EST to Chicago CST; Oakridge ODC-DSP-20241202-REF-01 R-613. Both local dates December 6. |
| 10 | R-614 | 2024-12-12 23:00 | 2024-12-13 06:00 | 2024-12-13 07:00 | =IF(INT(C10)<>INT(D10),"Window crosses midnight",IF(INT(C10)=INT(B10)+1,"Next-day start",IF(INT(C10)=INT(B10),"Same-day start","Other"))) | Chicago CST to Detroit EST; Oakridge ODC-DSP-20241202-REF-01 R-614. December 13, 06:00–07:00; R-611 has identical clock times on December 6. Times alone are not the answer. |
| 11 | R-615 | 2024-12-12 19:30 | 2024-12-12 23:30 | 2024-12-13 00:30 | =IF(INT(C11)<>INT(D11),"Window crosses midnight",IF(INT(C11)=INT(B11)+1,"Next-day start",IF(INT(C11)=INT(B11),"Same-day start","Other"))) | Chicago CST to Indianapolis EST; Oakridge ODC-DSP-20241202-REF-01 R-615. Opening December 12, closing December 13; name both dates. |
| 12 | R-616 | 2024-12-13 06:10 | 2024-12-13 08:00 | 2024-12-13 08:45 | =IF(INT(C12)<>INT(D12),"Window crosses midnight",IF(INT(C12)=INT(B12)+1,"Next-day start",IF(INT(C12)=INT(B12),"Same-day start","Other"))) | Chicago CST to Milwaukee CST; Oakridge ODC-DSP-20241202-REF-01 R-616. Both local dates December 13. |

Next-day start: =COUNTIF(E7:E12,"Next-day start") → 3. Same-day start: =COUNTIF(E7:E12,"Same-day start") → 2. Window crosses midnight: =COUNTIF(E7:E12,"Window crosses midnight") → 1. Categories are exclusive; the window crossing takes precedence. The December 9 arrival-window diagram shows R-611 and R-612 overlapping on December 6; R-614 is a different Friday.
