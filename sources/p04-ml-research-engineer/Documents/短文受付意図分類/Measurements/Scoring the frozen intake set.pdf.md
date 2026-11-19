---
path: Documents/短文受付意図分類/Measurements/Scoring the frozen intake set.pdf
kind: tex:mercer-research-brief.cls
author: Fatima Rahman
derived_from:
---

# Scoring the frozen intake set

Document ID: MDR-STI-20250408-RM-01-R01  
Issued: April 8, 2025  
Author: Fatima Rahman  
For: Vanessa Nguyen and Emily Chen

## Before scoring

- [ ] Use the April 3 frozen case allocation: 2,804 training, 935 development and 935 evaluation documents. The 126 Hold documents remain outside scoring.
- [ ] Match both sets of predictions to the same 935 evaluation document IDs and the same July-revised reference labels. A document is the scoring unit.
- [ ] Confirm all twelve reference classes have positive support. Use received bodies only as model input; case keys and channel are allocation information, not features.
- [ ] Record the training majority class and complete feature and model choices using development data before opening evaluation results.

## Per-class counts

For each of the twelve classes, count support, TP, FP and FN against the fixed reference. Support is TP + FN. Calculate F1 as 2TP / (2TP + FP + FN). Keep the unrounded fraction for the macro average; add all twelve unrounded F1 values, divide by twelve, then round the final reported value. Include the counts with the result so that small supports remain visible.

The majority baseline predicts the training partition's most frequent class for every evaluation body. For the eleven classes it never predicts, TP and FP are zero and precision is defined as zero. Their positive reference support makes FN positive and F1 zero. The predicted class receives its own TP, FP and FN from the same reference labels. A class with zero reference support would have required partition review before the freeze; it is not resolved by the zero-prediction rule.

## After scoring

- [ ] Check that each model has 935 predictions and that class supports sum to 935.
- [ ] Keep the frozen membership and labels unchanged; do not adjust features, settings or partitions in response to evaluation scores. Record any later proposed change as a separate evaluation condition, not a revision of this result.
- [ ] Report both scores with the common denominator, allocation and metric. The withdrawn January document-split result is not a same-set comparator.
