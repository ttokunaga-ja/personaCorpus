---
path: Documents/短文受付意図分類/Measurements/April macro-F1 reconciliation.csv
kind: text
author: Vanessa Nguyen
derived_from:
---

```csv
intent,support,tp,fp,fn,f1,macro_f1_contribution
New work request,132,114,23,18,0.847583643,0.070631970
Quote request,90,75,13,15,0.842696629,0.070224719
Schedule check,110,95,16,15,0.859728507,0.071644042
Reschedule,85,68,14,17,0.814371257,0.067864271
Cancel request,55,43,12,12,0.781818182,0.065151515
Job status,98,86,14,12,0.868686869,0.072390572
Send document,70,57,14,13,0.808510638,0.067375887
Request document,65,55,8,10,0.859375000,0.071614583
Correct record,52,40,14,12,0.754716981,0.062893082
Access issue,58,47,10,11,0.817391304,0.068115942
Report defect,67,59,5,8,0.900763359,0.075063613
Invoice question,53,44,9,9,0.830188679,0.069182390
```

The twelve rows reproduce the accepted April 2025 linear-classifier counts. Support totals 935. Each contribution is the unrounded class F1 divided by 12, displayed to nine decimal places; summing the unrounded fractions gives 0.832152587410, rounded to 0.832. This checks the fixed evaluation, not a rescoring of the 160 January development errors.
