---
path: Documents/Lab-book/2024/07/July 11 readout by culture date.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---

BCE-CE-2024-TBL-003-R00 | Melissa Watkins | Jul 17, 2024 | July 11 readout by culture date; concentration ng/mL and CV (%)

## Sheet: Joined readout

A7:H7 are the headers below; rows 8–31 contain the complete July 11 export matched to the dated handoff crosswalk on full run ID. CV is in percentage numbers. Rack slot and condition agree with the crosswalk on each row.

| A Run ID | B Measurement date | C Rack slot | D Condition | E Mean concentration (ng/mL) | F Within-sample CV (%) | G Culture date | H Above 15% |
| --- | --- | --- | --- | --- | --- | --- | --- |
| RPT-240711-01 | 2024-07-11 | 01 | Reference | 7.20 | 9 | 2024-07-08 | =IF(F8>15,1,0) |
| RPT-240711-02 | 2024-07-11 | 02 | Reference | 7.30 | 10 | 2024-07-08 | =IF(F9>15,1,0) |
| RPT-240711-03 | 2024-07-11 | 03 | Reference | 7.40 | 9 | 2024-07-08 | =IF(F10>15,1,0) |
| RPT-240711-04 | 2024-07-11 | 04 | Reference | 7.30 | 10 | 2024-07-08 | =IF(F11>15,1,0) |
| RPT-240711-05 | 2024-07-11 | 05 | Buffered | 7.30 | 9 | 2024-07-08 | =IF(F12>15,1,0) |
| RPT-240711-06 | 2024-07-11 | 06 | Buffered | 7.40 | 10 | 2024-07-08 | =IF(F13>15,1,0) |
| RPT-240711-07 | 2024-07-11 | 07 | Buffered | 7.50 | 10 | 2024-07-08 | =IF(F14>15,1,0) |
| RPT-240711-08 | 2024-07-11 | 08 | Buffered | 7.40 | 11 | 2024-07-08 | =IF(F15>15,1,0) |
| RPT-240711-09 | 2024-07-11 | 09 | Supplemented | 7.40 | 10 | 2024-07-08 | =IF(F16>15,1,0) |
| RPT-240711-10 | 2024-07-11 | 10 | Supplemented | 7.50 | 10 | 2024-07-08 | =IF(F17>15,1,0) |
| RPT-240711-11 | 2024-07-11 | 11 | Supplemented | 7.60 | 11 | 2024-07-08 | =IF(F18>15,1,0) |
| RPT-240711-12 | 2024-07-11 | 12 | Supplemented | 7.50 | 11 | 2024-07-08 | =IF(F19>15,1,0) |
| RPT-240711-13 | 2024-07-11 | 01 | Reference | 6.60 | 12 | 2024-07-10 | =IF(F20>15,1,0) |
| RPT-240711-14 | 2024-07-11 | 02 | Reference | 6.70 | 12 | 2024-07-10 | =IF(F21>15,1,0) |
| RPT-240711-15 | 2024-07-11 | 03 | Reference | 6.80 | 13 | 2024-07-10 | =IF(F22>15,1,0) |
| RPT-240711-16 | 2024-07-11 | 04 | Reference | 6.70 | 13 | 2024-07-10 | =IF(F23>15,1,0) |
| RPT-240711-17 | 2024-07-11 | 05 | Buffered | 6.70 | 10 | 2024-07-10 | =IF(F24>15,1,0) |
| RPT-240711-18 | 2024-07-11 | 06 | Buffered | 6.80 | 15 | 2024-07-10 | =IF(F25>15,1,0) |
| RPT-240711-19 | 2024-07-11 | 07 | Buffered | 6.90 | 17 | 2024-07-10 | =IF(F26>15,1,0) |
| RPT-240711-20 | 2024-07-11 | 08 | Buffered | 6.80 | 19 | 2024-07-10 | =IF(F27>15,1,0) |
| RPT-240711-21 | 2024-07-11 | 09 | Supplemented | 6.80 | 12 | 2024-07-10 | =IF(F28>15,1,0) |
| RPT-240711-22 | 2024-07-11 | 10 | Supplemented | 6.90 | 13 | 2024-07-10 | =IF(F29>15,1,0) |
| RPT-240711-23 | 2024-07-11 | 11 | Supplemented | 7.00 | 18 | 2024-07-10 | =IF(F30>15,1,0) |
| RPT-240711-24 | 2024-07-11 | 12 | Supplemented | 6.90 | 21 | 2024-07-10 | =IF(F31>15,1,0) |

Crosswalk: July 8 and 10 rack crosswalk.tsv, reconciled July 16 by Lauren Brooks and Benjamin Ortiz. Full run IDs are unique; slots 01–12 are reused on both dates. All 24 full IDs match once.

## Sheet: Counts

B4: BCE-CE-2024-TBL-003-R00; E4: Melissa Watkins; B5: Jul 17, 2024; E5: count; measurement date Jul 11, 2024.

| Row | B Culture date | C Reference | D Buffered | E Supplemented | F Total |
| --- | --- | --- | --- | --- | --- |
| 7 | Culture date | Reference | Buffered | Supplemented | Total |
| 8 | 2024-07-08 | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,C$7) | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,D$7) | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,E$7) | =SUM(C8:E8) |
| 9 | 2024-07-10 | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,C$7) | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,D$7) | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,E$7) | =SUM(C9:E9) |
| 10 | All | =SUM(C8:C9) | =SUM(D8:D9) | =SUM(E8:E9) | =SUM(F8:F9) |

Displayed: Jul 8 4 / 4 / 4 / 12; Jul 10 4 / 4 / 4 / 12; total 8 / 8 / 8 / 24.

## Sheet: Flags over 15%

B4: BCE-CE-2024-TBL-003-R00; E4: Melissa Watkins; B5: Jul 17, 2024; E5: flags; measurement date Jul 11, 2024.

| Row | B Culture date | C Reference | D Buffered | E Supplemented | F Total |
| --- | --- | --- | --- | --- | --- |
| 7 | Culture date | Reference | Buffered | Supplemented | Total |
| 8 | 2024-07-08 | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,C$7,'Joined readout'!$F$8:$F$31,">15") | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,D$7,'Joined readout'!$F$8:$F$31,">15") | =COUNTIFS('Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,E$7,'Joined readout'!$F$8:$F$31,">15") | =SUM(C8:E8) |
| 9 | 2024-07-10 | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,C$7,'Joined readout'!$F$8:$F$31,">15") | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,D$7,'Joined readout'!$F$8:$F$31,">15") | =COUNTIFS('Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,E$7,'Joined readout'!$F$8:$F$31,">15") | =SUM(C9:E9) |
| 10 | All | =SUM(C8:C9) | =SUM(D8:D9) | =SUM(E8:E9) | =SUM(F8:F9) |

Displayed: Jul 8 0 / 0 / 0 / 0; Jul 10 0 / 2 / 2 / 4; total 0 / 2 / 2 / 4. Four samples per date-condition cell; day totals 0/12 and 4/12, overall 4/24. Measurement date Jul 11, 2024. Values exactly 15% are within criterion.

## Sheet: Concentration means

B4: BCE-CE-2024-TBL-003-R00; E4: Melissa Watkins; B5: Jul 17, 2024; E5: mean; measurement date Jul 11, 2024.

| Row | B Culture date | C Reference | D Buffered | E Supplemented | F Total |
| --- | --- | --- | --- | --- | --- |
| 7 | Culture date | Reference | Buffered | Supplemented | Total |
| 8 | 2024-07-08 | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,C$7) | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,D$7) | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B8,'Joined readout'!$D$8:$D$31,E$7) | =AVERAGEIF('Joined readout'!$G$8:$G$31,B8,'Joined readout'!$E$8:$E$31) |
| 9 | 2024-07-10 | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,C$7) | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,D$7) | =AVERAGEIFS('Joined readout'!$E$8:$E$31,'Joined readout'!$G$8:$G$31,$B9,'Joined readout'!$D$8:$D$31,E$7) | =AVERAGEIF('Joined readout'!$G$8:$G$31,B9,'Joined readout'!$E$8:$E$31) |
| 10 | All | =AVERAGEIF('Joined readout'!$D$8:$D$31,C$7,'Joined readout'!$E$8:$E$31) | =AVERAGEIF('Joined readout'!$D$8:$D$31,D$7,'Joined readout'!$E$8:$E$31) | =AVERAGEIF('Joined readout'!$D$8:$D$31,E$7,'Joined readout'!$E$8:$E$31) | =AVERAGE('Joined readout'!E8:E31) |

Displayed means (ng/mL): Jul 8 7.30 / 7.40 / 7.50 / 7.40; Jul 10 6.70 / 6.80 / 6.90 / 6.80; overall 7.00 / 7.10 / 7.20 / 7.10. All readings included.
