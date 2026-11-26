---
path: Documents/Confidence-abstention/Queue balance checks.tsv
kind: text
author: Emily Chen
derived_from:
---

## Check note (outside the TSV)

For each replay date I counted review arrivals and lunch-condition completions in Review arrivals and completions.csv through each listed PST minute, including events in that minute. Reset the cumulative counts for each date. The difference is outstanding. On December 5 the review started at 13:00 is still part of the balance. Use the completion column for the subtraction even when a start is recorded at the check time.

```tsv
date	time	arrivals	completed	outstanding
2024-12-04	11:59	8	8	0
2024-12-04	13:00	18	8	10
2024-12-04	13:15	18	13	5
2024-12-04	13:30	18	18	0
2024-12-04	14:00	18	18	0
2024-12-04	17:00	28	28	0
2024-12-05	11:59	7	7	0
2024-12-05	13:00	25	7	18
2024-12-05	13:15	25	12	13
2024-12-05	13:30	25	17	8
2024-12-05	14:00	25	25	0
2024-12-05	17:00	32	32	0
2024-12-06	11:59	8	8	0
2024-12-06	13:00	22	8	14
2024-12-06	13:15	22	13	9
2024-12-06	13:30	22	18	4
2024-12-06	14:00	22	22	0
2024-12-06	17:00	30	30	0
```
