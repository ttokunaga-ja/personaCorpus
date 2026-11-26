---
path: Documents/Recovery-evidence/Ticket archive interval comparison.xlsx
kind: brt-archive-interval-comparison.xlsx
author: Daniel Jennings
derived_from:
---

# Ticket archive interval comparison

## Interval comparison

B3: BRT-IC-20240916-001  
E3: Daniel Jennings  
B4: 16 Sep 2024  
E4: Isolated ticket archive · 9 and 11 Sep 2024 · 09:00–15:00 UTC each day

A5:F5: For Sanjay Mehta and Anjali Shah. Source: RJS archive hourly observations.csv.

| Row | A: No. | B: Interval (min) | C: Observation window (UTC) | D: Observed starts | E: Open predecessor at start | F: Evidence and follow-up |
|---:|---:|---:|---|---:|---:|---|
| 7 | 1 | 15 | 09 Sep 2024 · 09:00–10:00 | 4 | 0 | RJS tally and 9 Sep observer sheet. Clear opening queue. |
| 8 | 2 | 15 | 09 Sep 2024 · 10:00–11:00 | 4 | 0 | RJS tally and 9 Sep observer sheet agree. |
| 9 | 3 | 15 | 09 Sep 2024 · 11:00–12:00 | 4 | 0 | RJS tally and 9 Sep observer sheet agree. |
| 10 | 4 | 15 | 09 Sep 2024 · 12:00–13:00 | 4 | 0 | Busier hour. RJS tally and observer sheet show no flags. |
| 11 | 5 | 15 | 09 Sep 2024 · 13:00–14:00 | 4 | 0 | RJS tally and 9 Sep observer sheet agree. |
| 12 | 6 | 15 | 09 Sep 2024 · 14:00–15:00 | 4 | 0 | RJS tally and observer sheet. Last start 14:45. |
| 13 | 7 | 10 | 11 Sep 2024 · 09:00–10:00 | 6 | 0 | RJS tally and 11 Sep observer sheet. Clear opening queue. |
| 14 | 8 | 10 | 11 Sep 2024 · 10:00–11:00 | 6 | 0 | RJS tally and 11 Sep observer sheet agree. |
| 15 | 9 | 10 | 11 Sep 2024 · 11:00–12:00 | 6 | 0 | RJS tally and 11 Sep observer sheet agree. |
| 16 | 10 | 10 | 11 Sep 2024 · 12:00–13:00 | 6 | 2 | RJS-WS-20240912-001 and overlap pairs extract. At 12:20, W-3184 was open until 12:22:10. At 12:40, W-3186 was open until 12:41:35. Both finishes checked by write reference. |
| 17 | 11 | 10 | 11 Sep 2024 · 13:00–14:00 | 6 | 0 | RJS tally and 11 Sep observer sheet agree. |
| 18 | 12 | 10 | 11 Sep 2024 · 14:00–15:00 | 6 | 0 | RJS tally and observer sheet. Last start 14:50. |

| Row | A:C total label | D: Observed starts | E: Open predecessor at start | F: Use in trial review |
|---:|---|---|---|---|
| 19 | 15-minute total | =SUMIF(B7:B18,15,D7:D18) | =IF(COUNTIFS(B7:B18,15,E7:E18,"")>0,"",SUMIF(B7:B18,15,E7:E18)) | Comparator. Witness record RJS-WS-20240912-001. |
| 20 | 10-minute total | =SUMIF(B7:B18,10,D7:D18) | =IF(COUNTIFS(B7:B18,10,E7:E18,"")>0,"",SUMIF(B7:B18,10,E7:E18)) | Candidate for a controlled load trial. Both flagged finishes are in Ticket archive overlap pairs.csv. |

A21:F21 note: Each hour includes the opening boundary and excludes the closing boundary. Count an overlap against the current start if the immediate predecessor is still open. A finish at the same UTC second counts as clear. Unknown overlap counts stay blank and prevent an overlap total. Eligible-set coverage needs a fixed-reference readback. Existing operator recovery terms remain in force.

## Check totals

Expected formula results for the data above (outside the sheet): 15 minutes gives 24 starts and 0 overlaps. 10 minutes gives 36 starts and 2 overlaps.
