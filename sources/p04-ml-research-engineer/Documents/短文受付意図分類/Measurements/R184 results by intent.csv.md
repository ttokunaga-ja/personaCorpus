---
path: Documents/短文受付意図分類/Measurements/R184 results by intent.csv
kind: text
author: Fatima Rahman
derived_from:
---

```csv
intent,reference_support,r184_tp,r184_fp,r184_fn,r184_f1,r173_tp,r173_fp,r173_fn,r173_f1
New work request,160,139,13,21,0.891025641026,143,26,17,0.869300911854
Quote request,120,109,12,11,0.904564315353,99,9,21,0.868421052632
Schedule check,110,96,15,14,0.868778280543,88,10,22,0.846153846154
Reschedule,100,80,13,20,0.829015544041,91,14,9,0.887804878049
Cancel request,80,70,14,10,0.853658536585,63,10,17,0.823529411765
Job status,120,111,11,9,0.917355371901,109,25,11,0.858267716535
Send document,90,77,11,13,0.865168539326,80,14,10,0.869565217391
Request document,85,72,11,13,0.857142857143,74,16,11,0.845714285714
Correct record,75,66,10,9,0.874172185430,67,8,8,0.893333333333
Access issue,70,65,10,5,0.896551724138,58,8,12,0.852941176471
Report defect,95,83,9,12,0.887700534759,75,14,20,0.815217391304
Invoice question,95,82,21,13,0.828282828283,77,22,18,0.793814432990
```

Reconciliation, outside the CSV: The shared reference column totals 1,200. For each run, FN is reference support minus TP, and F1 is 2TP/(2TP+FP+FN). Across the twelve rows, R184 has 150 FP and 150 FN, and R173 has 176 FP and 176 FN. Calculate the macro means from the counts before rounding.
