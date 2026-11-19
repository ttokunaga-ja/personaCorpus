---
path: Documents/短文受付意図分類/Research-questions/Leave the shorter character window out.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Leave the shorter character window out

Document ID: MDR-STI-20260708-RP-01-R01  
Issued: July 8, 2026  
Author: Vanessa Nguyen  
For: Emily Chen and Fatima Rahman

## The extra window did not earn a place in the pair

Emily and I reviewed the three runs on the same 1,200 July development bodies. R187 extends the character window from 3–5 to 2–5 while retaining the word features. I wanted to see whether two-character fragments could help on very short misspellings. Fragments that short also recur in routine greetings, such as “th” and “an” in “thanks”. They can appear with many different requests. The aggregate development result does not tell us which fragments drove individual predictions.

| Run | Body features | Development macro-F1 | July 8 disposition |
|---|---|---:|---|
| R173 | Word frequencies | 0.848 | Keep as the comparator |
| R184 | Word frequencies and character 3–5 grams | 0.871 | Keep as the freeze candidate |
| R187 | Word frequencies and character 2–5 grams | 0.870 | End at development |

R187 did not exceed R184. The scores differ by only 0.001 at the displayed precision, so I would not use this run to claim a general disadvantage for two-character features. For this comparison, leave R187 out of the evaluation pair.

## Settings closed on July 8

Emily, retain R173 and R184 with the settings used for the development readout. Both use the same July training partition, a regularized linear classifier and the received body alone. The feature window is the planned difference between the two runs. Fatima, pair their top predictions on the same July evaluation IDs and the same twelve reference categories. Keep this pair and its feature settings fixed when the evaluation becomes available. Any follow-up prompted by the evaluation belongs in a new comparison.
