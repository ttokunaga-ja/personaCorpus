---
path: Documents/短文受付意図分類/Measurements/July reference support check.csv
kind: text
author: Fatima Rahman
derived_from:
---

```csv
intent,unique_reference_documents
New work request,160
Quote request,120
Schedule check,110
Reschedule,100
Cancel request,80
Job status,120
Send document,90
Request document,85
Correct record,75
Access issue,70
Report defect,95
Invoice question,95
Total with reference label,1200
```

Reconciliation for Vanessa: all twelve intents have positive support before the prediction join. The final record in the CSV is the sum of the twelve intent rows. Exclude that record from per-intent scoring and from the macro-F1 average. This note sits outside the CSV.
