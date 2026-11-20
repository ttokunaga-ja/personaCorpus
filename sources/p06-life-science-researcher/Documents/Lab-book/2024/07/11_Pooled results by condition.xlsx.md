---
path: Documents/Lab-book/2024/07/11_Pooled results by condition.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---

BCE-CE-2024-TBL-002-R00 | Melissa Watkins | Jul 11, 2024 | July 11 readout; concentration in ng/mL; CV in percentage points

## Sheet: Imported readout

A7:F7 are the headers below; rows 8–31 contain the complete saved export. CV values are percentage numbers (15 means 15%).

| A Run ID | B Measurement date | C Rack slot | D Condition | E Mean concentration (ng/mL) | F Within-sample CV (%) |
| --- | --- | --- | --- | --- | --- |
| RPT-240711-01 | 2024-07-11 | 01 | Reference | 7.20 | 9 |
| RPT-240711-02 | 2024-07-11 | 02 | Reference | 7.30 | 10 |
| RPT-240711-03 | 2024-07-11 | 03 | Reference | 7.40 | 9 |
| RPT-240711-04 | 2024-07-11 | 04 | Reference | 7.30 | 10 |
| RPT-240711-05 | 2024-07-11 | 05 | Buffered | 7.30 | 9 |
| RPT-240711-06 | 2024-07-11 | 06 | Buffered | 7.40 | 10 |
| RPT-240711-07 | 2024-07-11 | 07 | Buffered | 7.50 | 10 |
| RPT-240711-08 | 2024-07-11 | 08 | Buffered | 7.40 | 11 |
| RPT-240711-09 | 2024-07-11 | 09 | Supplemented | 7.40 | 10 |
| RPT-240711-10 | 2024-07-11 | 10 | Supplemented | 7.50 | 10 |
| RPT-240711-11 | 2024-07-11 | 11 | Supplemented | 7.60 | 11 |
| RPT-240711-12 | 2024-07-11 | 12 | Supplemented | 7.50 | 11 |
| RPT-240711-13 | 2024-07-11 | 01 | Reference | 6.60 | 12 |
| RPT-240711-14 | 2024-07-11 | 02 | Reference | 6.70 | 12 |
| RPT-240711-15 | 2024-07-11 | 03 | Reference | 6.80 | 13 |
| RPT-240711-16 | 2024-07-11 | 04 | Reference | 6.70 | 13 |
| RPT-240711-17 | 2024-07-11 | 05 | Buffered | 6.70 | 10 |
| RPT-240711-18 | 2024-07-11 | 06 | Buffered | 6.80 | 15 |
| RPT-240711-19 | 2024-07-11 | 07 | Buffered | 6.90 | 17 |
| RPT-240711-20 | 2024-07-11 | 08 | Buffered | 6.80 | 19 |
| RPT-240711-21 | 2024-07-11 | 09 | Supplemented | 6.80 | 12 |
| RPT-240711-22 | 2024-07-11 | 10 | Supplemented | 6.90 | 13 |
| RPT-240711-23 | 2024-07-11 | 11 | Supplemented | 7.00 | 18 |
| RPT-240711-24 | 2024-07-11 | 12 | Supplemented | 6.90 | 21 |

## Sheet: Pooled summary

B4: BCE-CE-2024-TBL-002-R00; E4: Melissa Watkins; B5: Jul 11, 2024; E5: July 11 measurement; counts, ng/mL, CV (%).

| Cell | A Metric | B Reference | C Buffered | D Supplemented | E All |
| --- | --- | --- | --- | --- | --- |
| 7 | Metric | Reference | Buffered | Supplemented | All |
| 8 | Sample count | =COUNTIF('Imported readout'!$D$8:$D$31,B$7) | =COUNTIF('Imported readout'!$D$8:$D$31,C$7) | =COUNTIF('Imported readout'!$D$8:$D$31,D$7) | =SUM(B8:D8) |
| 9 | Mean concentration (ng/mL) | =AVERAGEIF('Imported readout'!$D$8:$D$31,B$7,'Imported readout'!$E$8:$E$31) | =AVERAGEIF('Imported readout'!$D$8:$D$31,C$7,'Imported readout'!$E$8:$E$31) | =AVERAGEIF('Imported readout'!$D$8:$D$31,D$7,'Imported readout'!$E$8:$E$31) | =AVERAGE('Imported readout'!E8:E31) |
| 10 | Mean individual CV (%) | =AVERAGEIF('Imported readout'!$D$8:$D$31,B$7,'Imported readout'!$F$8:$F$31) | =AVERAGEIF('Imported readout'!$D$8:$D$31,C$7,'Imported readout'!$F$8:$F$31) | =AVERAGEIF('Imported readout'!$D$8:$D$31,D$7,'Imported readout'!$F$8:$F$31) | =AVERAGE('Imported readout'!F8:F31) |
| 11 | Samples over 15% CV | =COUNTIFS('Imported readout'!$D$8:$D$31,B$7,'Imported readout'!$F$8:$F$31,">15") | =COUNTIFS('Imported readout'!$D$8:$D$31,C$7,'Imported readout'!$F$8:$F$31,">15") | =COUNTIFS('Imported readout'!$D$8:$D$31,D$7,'Imported readout'!$F$8:$F$31,">15") | =SUM(B11:D11) |

Expected displayed results, in Reference / Buffered / Supplemented / All order: counts 8 / 8 / 8 / 24; concentration 7.00 / 7.10 / 7.20 / 7.10; mean CV 11.00 / 12.625 / 13.25 / 12.2917%; flags 0 / 2 / 2 / 4. A CV of exactly 15% is within criterion. All 24 rows are included; culture date has not yet been joined.
