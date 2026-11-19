---
path: Documents/Publications/Revision/Comparison conditions for the editor copy.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Comparison conditions for the editor copy

## Sheet: Adjudication

A2: Comparison conditions for the editor copy  
B3: MDR-STI-20260427-AW-01-R01  
E3: April 27, 2026  
B4: Vanessa Nguyen  
E4: Short intake paper; Emily Chen and Michael Serrano

| Row | A Ref. | B Comparison condition | C Evidence source | D Status | E Draft location | F What to verify |
|---:|---:|---|---|---|---|---|
| 6 | 1 | Case-level allocation and separation | Case split freeze checks, April 3, 2025, Adjudication rows 6–8 and 11–13 | Evidence checked; text check pending | Methods — partitions | Training 2,804; development 935; evaluation 935 documents. Shared cases: zero for each of the three partition pairs. |
| 7 | 2 | Common evaluation set and reference | Scoring the frozen intake set, April 8, 2025, Before scoring; Case split freeze checks, rows 9–10 and Evaluation support | Evidence checked; text check pending | Methods — evaluation and reference labels | Both comparisons use the same 935 evaluation documents and July-revised reference labels. The 126 Hold documents are outside allocation and scoring. |
| 8 | 3 | Model input | Scoring the frozen intake set, Before scoring; Case split freeze checks, row 15 | Evidence checked; text check pending | Methods — input | Received body only; case keys and channel information are not features. |
| 9 | 4 | Feature and setting selection | Scoring the frozen intake set, Before and After scoring | Evidence checked; text check pending | Methods — model selection | Feature and model choices are completed using development data before evaluation results; no changes in response to evaluation scores. |
| 10 | 5 | Training-majority baseline | Scoring the frozen intake set, Per-class counts; Case split reevaluation Apr 17 | Evidence checked; text check pending | Methods — comparison; Results | Training's most frequent class is New work request; predict it for all 935 evaluation documents. Report baseline macro-F1 0.021 alongside the linear classifier's 0.832 on that same set. |
| 11 | 6 | Macro-F1 calculation | Scoring the frozen intake set, Per-class counts | Evidence checked; text check pending | Methods — scoring | Compute 2TP/(2TP+FP+FN) for each of 12 classes, average the unrounded values with equal weight, then round the final result. |

A12:F12: The 160 selected development errors and the 41 scope-related readings are diagnostic observations, not the denominator of the fixed evaluation. Evidence has been checked; Emily's reading of the revised text is still pending.
