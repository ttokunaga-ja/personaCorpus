---
path: Documents/Quality/Deviations/October measurement dates and hold scope.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---

# October measurement dates and proposed hold scope

## Sheet: Date reconciliation

B4: BCE-CE-2024-TBL-005-R00  
E4: Melissa Watkins and Anthony Mendoza  
B5: Oct 23, 2024  
E5: Result records; CE-Reader-2  
For: Robert Nguyen

| Row | A: No. | B: Measurement date | C: Result records | D: Proposed hold | E: Outside hold | F: Total classified | G: Date evidence / scope |
|---:|---:|---|---|---|---|---|---|
| 8 | 1 | 2024-10-08 | =COUNTIFS(Results!$B$2:$B$55,">="&B8,Results!$B$2:$B$55,"<"&B8+1,Results!$C$2:$C$55,"CE-Reader-2") | =0 | =C8 | =SUM(D8:E8) | After Oct 8 08:52 good check; routine review |
| 9 | 2 | 2024-10-09 | =COUNTIFS(Results!$B$2:$B$55,">="&B9,Results!$B$2:$B$55,"<"&B9+1,Results!$C$2:$C$55,"CE-Reader-2") | =C9 | =0 | =SUM(D9:E9) | Proposed hold |
| 10 | 3 | 2024-10-10 | =COUNTIFS(Results!$B$2:$B$55,">="&B10,Results!$B$2:$B$55,"<"&B10+1,Results!$C$2:$C$55,"CE-Reader-2") | =C10 | =0 | =SUM(D10:E10) | Proposed hold |
| 11 | 4 | 2024-10-11 | =COUNTIFS(Results!$B$2:$B$55,">="&B11,Results!$B$2:$B$55,"<"&B11+1,Results!$C$2:$C$55,"CE-Reader-2") | =C11 | =0 | =SUM(D11:E11) | Proposed hold |
| 12 | 5 | 2024-10-15 | =COUNTIFS(Results!$B$2:$B$55,">="&B12,Results!$B$2:$B$55,"<"&B12+1,Results!$C$2:$C$55,"CE-Reader-2") | =C12 | =0 | =SUM(D12:E12) | Proposed hold |
| 13 | 6 | 2024-10-17 | =COUNTIFS(Results!$B$2:$B$55,">="&B13,Results!$B$2:$B$55,"<"&B13+1,Results!$C$2:$C$55,"CE-Reader-2") | =C13 | =0 | =SUM(D13:E13) | Proposed hold |
| 14 | 7 | 2024-10-21 | =COUNTIFS(Results!$B$2:$B$55,">="&B14,Results!$B$2:$B$55,"<"&B14+1,Results!$C$2:$C$55,"CE-Reader-2") | =C14 | =0 | =SUM(D14:E14) | Acquired Oct 21; archive saved Oct 22 08:18 |
| 15 | 8 | 2024-10-22 | =COUNTIFS(Results!$B$2:$B$55,">="&B15,Results!$B$2:$B$55,"<"&B15+1,Results!$C$2:$C$55,"CE-Reader-2") | =0 | =C15 | =SUM(D15:E15) | Service completed 11:18 EDT; first study reading 11:32 EDT; routine review |
| 16 | Total |  | =SUM(C8:C15) | =SUM(D8:D15) | =SUM(E8:E15) | =SUM(F8:F15) | 54 records; 44 proposed hold, 10 outside |

Oct 8 is supported by BCE-CE-2024-LAB-004-R00; the six readings follow the 08:52 in-band check. OCT-21 is dated by its acquisition times and Lauren's BCE-CE-2024-LAB-006-R00, not the next-morning archive save. Mason certificate MLM-2024-1022-SV-014 records completion at Oct 22 11:18 EDT. The four OCT-22 study readings start at 11:32 EDT and are distinct from service reference indications. The six earlier dates are proposed for hold pending review; original results remain unchanged. Outside this proposed hold does not remove routine review.

## Sheet: Results

Row 1 contains the four original column names below; rows 2–55 contain the three exports in the order shown. Column B contains actual local date-time values, with seconds preserved, so the date counts on Date reconciliation use acquisition time. Concentrations are unchanged numeric values in ng/mL.

```csv
result_id,local_timestamp_edt,reader_tag,concentration_ng_ml
OCT-08-01,2024-10-08 09:12:14,CE-Reader-2,4.74
OCT-08-02,2024-10-08 09:15:47,CE-Reader-2,5.08
OCT-08-03,2024-10-08 09:19:20,CE-Reader-2,4.92
OCT-08-04,2024-10-08 09:22:51,CE-Reader-2,5.27
OCT-08-05,2024-10-08 09:26:19,CE-Reader-2,4.81
OCT-08-06,2024-10-08 09:29:56,CE-Reader-2,5.13
OCT-09-01,2024-10-09 13:06:21,CE-Reader-2,5.17
OCT-09-02,2024-10-09 13:09:48,CE-Reader-2,4.83
OCT-09-03,2024-10-09 13:13:16,CE-Reader-2,5.36
OCT-09-04,2024-10-09 13:16:50,CE-Reader-2,4.95
OCT-09-05,2024-10-09 13:20:25,CE-Reader-2,5.02
OCT-09-06,2024-10-09 13:23:59,CE-Reader-2,4.69
OCT-10-01,2024-10-10 10:01:07,CE-Reader-2,4.78
OCT-10-02,2024-10-10 10:04:42,CE-Reader-2,5.19
OCT-10-03,2024-10-10 10:08:11,CE-Reader-2,5.06
OCT-10-04,2024-10-10 10:11:44,CE-Reader-2,4.88
OCT-10-05,2024-10-10 10:15:18,CE-Reader-2,5.41
OCT-10-06,2024-10-10 10:18:53,CE-Reader-2,4.96
OCT-10-07,2024-10-10 10:22:25,CE-Reader-2,5.11
OCT-10-08,2024-10-10 10:25:58,CE-Reader-2,4.71
OCT-11-01,2024-10-11 14:08:32,CE-Reader-2,5.29
OCT-11-02,2024-10-11 14:12:06,CE-Reader-2,4.87
OCT-11-03,2024-10-11 14:15:39,CE-Reader-2,5.14
OCT-11-04,2024-10-11 14:19:12,CE-Reader-2,4.76
OCT-11-05,2024-10-11 14:22:45,CE-Reader-2,5.05
OCT-11-06,2024-10-11 14:26:17,CE-Reader-2,5.33
OCT-15-01,2024-10-15 09:37:19,CE-Reader-2,4.91
OCT-15-02,2024-10-15 09:40:53,CE-Reader-2,5.22
OCT-15-03,2024-10-15 09:44:24,CE-Reader-2,4.64
OCT-15-04,2024-10-15 09:47:58,CE-Reader-2,5.09
OCT-15-05,2024-10-15 09:51:31,CE-Reader-2,5.37
OCT-15-06,2024-10-15 09:55:05,CE-Reader-2,4.85
OCT-15-07,2024-10-15 09:58:38,CE-Reader-2,5.12
OCT-15-08,2024-10-15 10:02:10,CE-Reader-2,4.99
OCT-15-09,2024-10-15 10:05:43,CE-Reader-2,5.26
OCT-15-10,2024-10-15 10:09:17,CE-Reader-2,4.73
OCT-17-01,2024-10-17 13:21:04,CE-Reader-2,5.04
OCT-17-02,2024-10-17 13:24:37,CE-Reader-2,4.79
OCT-17-03,2024-10-17 13:28:11,CE-Reader-2,5.32
OCT-17-04,2024-10-17 13:31:45,CE-Reader-2,4.93
OCT-17-05,2024-10-17 13:35:17,CE-Reader-2,5.18
OCT-17-06,2024-10-17 13:38:51,CE-Reader-2,4.67
OCT-17-07,2024-10-17 13:42:24,CE-Reader-2,5.25
OCT-17-08,2024-10-17 13:45:58,CE-Reader-2,4.89
OCT-21-01,2024-10-21 10:04:12,CE-Reader-2,4.86
OCT-21-02,2024-10-21 10:07:38,CE-Reader-2,5.23
OCT-21-03,2024-10-21 10:11:05,CE-Reader-2,4.97
OCT-21-04,2024-10-21 10:14:41,CE-Reader-2,5.31
OCT-21-05,2024-10-21 10:18:09,CE-Reader-2,4.72
OCT-21-06,2024-10-21 10:21:34,CE-Reader-2,5.08
OCT-22-01,2024-10-22 11:32:00,CE-Reader-2,4.68
OCT-22-02,2024-10-22 11:35:26,CE-Reader-2,5.16
OCT-22-03,2024-10-22 11:39:03,CE-Reader-2,4.91
OCT-22-04,2024-10-22 11:42:29,CE-Reader-2,5.04
```

## Sheet: Run crosswalk

Lauren Brooks's eight-row dated run crosswalk:

```tsv
Run reference	Reader	Measurement date	Record count	Date evidence	Notes
OCT-08	CE-Reader-2	2024-10-08	6	Original acquisition timestamps; BCE-CE-2024-LAB-004-R00	Comparison: after 08:52 good check
OCT-09	CE-Reader-2	2024-10-09	6	Original acquisition timestamps	
OCT-10	CE-Reader-2	2024-10-10	8	Original acquisition timestamps	
OCT-11	CE-Reader-2	2024-10-11	6	Original acquisition timestamps	
OCT-15	CE-Reader-2	2024-10-15	10	Original acquisition timestamps	
OCT-17	CE-Reader-2	2024-10-17	8	Original acquisition timestamps	
OCT-21	CE-Reader-2	2024-10-21	6	Original acquisition timestamps; BCE-CE-2024-LAB-006-R00	Archive saved Oct 22 at 08:18 EDT
OCT-22	CE-Reader-2	2024-10-22	4	Original acquisition timestamps; MLM-2024-1022-SV-014	First study reading 11:32 EDT; service complete 11:18 EDT
```
