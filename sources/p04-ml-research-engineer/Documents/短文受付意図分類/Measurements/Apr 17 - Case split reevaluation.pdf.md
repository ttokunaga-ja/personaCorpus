---
path: Documents/短文受付意図分類/Measurements/Apr 17 - Case split reevaluation.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---
# Case-split reevaluation of short intake intent

Document ID: MDR-STI-20250417-RM-01-R01  
Issued: April 17, 2025  
Author: Vanessa Nguyen  
For: Michael Serrano; reviewed with Fatima Rahman

## Decision for reporting

The January document-partition score of 0.851 is withdrawn. Nicole Foster identified documents from the same cases in different partitions; Fatima and Joshua's reconciliation found 84 documents belonging to 24 cases crossing partition boundaries. Consequently the old evaluation was not independent of the other partitions. The January value is not a basis for model comparison or a use decision. Withdrawal notice MDR-STI-20250128-RM-01-R01 identifies the affected January readout.

## Conditions and results

| Condition | Training documents | Development documents | Evaluation documents | macro-F1 | Status |
|---|---:|---:|---:|---:|---|
| January document split, word-frequency linear classifier | 2,800 | 934 | 940 | 0.851 | Withdrawn |
| April case split, word-frequency linear classifier | 2,804 | 935 | 935 | 0.832 | Fixed evaluation |
| April case split, training-majority baseline | 2,804 | 935 | 935 | 0.021 | Fixed evaluation |

Both allocations draw on the same 4,674 documents with reference labels. The 126 Hold documents are outside allocation and scoring in both. The case split was fixed on April 3: all three partition pairs share zero cases, and all twelve classes have evaluation support. The 84 documents were not simply removed; their cases were kept together in the new allocation.

The two April results use the same 935 evaluation documents, reference labels, twelve classes and July-revised reading criteria. The unit of scoring is a received message body; the unit of allocation is a case. The linear classifier uses word frequencies from the received body, with choices made on development data. Case keys and channel information are not input features. The baseline predicts the training partition's most frequent class, New work request, for every evaluation document.

For each class F1 is 2TP/(2TP+FP+FN); macro-F1 is the equally weighted mean of the twelve unrounded class values, rounded at the end. The 0.019 difference between the displayed January and April linear scores spans different allocations and evaluation sets. It is not an effect of leakage estimated on the same evaluation set. The April score describes this fixed evaluation, not authorization for operational use.

## Records

The withdrawal notice records the superseded result. April intake comparison.csv contains the three condition rows; the April 3 partition record and April scoring check describe the freeze and scoring procedure. The accompanying chart shows the two linear scores as separate points, not a paired before-and-after test.
