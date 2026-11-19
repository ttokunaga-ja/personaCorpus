---
path: Documents/短文受付意図分類/Measurements/July development runs.csv
kind: text
author: Emily Chen
derived_from:
---

```csv
run,classifier,features,scoring_partition,scored_documents,macro_f1,disposition_july_8
R173,Regularized linear,Word frequencies only,July development,1200,0.848,Comparator
R184,Regularized linear,Word frequencies + character 3–5 grams,July development,1200,0.871,Freeze candidate
R187,Regularized linear,Word frequencies + character 2–5 grams,July development,1200,0.870,Stop after development
```

Handoff to Vanessa: all three rows use the same 1,200 development bodies and reference labels. The feature column gives the full feature set. Each score gives equal weight to the 12 intents.
