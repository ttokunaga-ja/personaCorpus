---
path: Documents/短文受付意図分類/Error-readings/R184 lower-F1 intents.csv
kind: text
author: Fatima Rahman
derived_from:
---

```csv
intent,reference_support,r173_f1,r184_f1,f1_change_r184_minus_r173,r173_tp,r184_tp,tp_change_r184_minus_r173
Reschedule,100,0.888,0.829,-0.058789334,91,80,-11
Correct record,75,0.893,0.874,-0.019161148,67,66,-1
Send document,90,0.870,0.865,-0.004396678,80,77,-3
```

Reconciliation outside the CSV: Both runs scored the same 1,200 July evaluation documents. Keep the three rows ordered by the drop calculated from the unrounded class scores in `R184 results by intent.csv`. The run scores are shown to three decimals and the differences to nine. TP changes are net changes in correctly classified documents for each reference intent. They do not give the number of individual documents whose predictions differed between runs.
