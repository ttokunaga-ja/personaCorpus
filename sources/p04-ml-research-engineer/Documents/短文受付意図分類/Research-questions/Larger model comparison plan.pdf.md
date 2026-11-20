---
path: Documents/短文受付意図分類/Research-questions/Larger model comparison plan.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Before we compare the heavier model

Document ID: MDR-STI-20240912-RP-01-R01  
Issued: September 12, 2024  
Author: Vanessa Nguyen  
To: Fatima Rahman and Emily Chen

## Question and common material

Can a larger contextual model improve the short-intake intent comparison over our word-frequency linear model? Fatima and Emily, please use the same July-revised twelve-class reference labels for both. Train each on the 2,800 training documents. The input is the received body only; do not pass route, subject, or later handling information to either model.

Score all 934 development documents once per model, with one top-label prediction per body. For each class calculate F1 as 2TP/(2TP+FP+FN); average the twelve unrounded class values with equal weight and round the resulting macro-F1 for reporting. Keep the class counts as well as the aggregate so that an improvement in the mean does not hide a loss in one intent.

## Boundaries

The development set includes the 800 bodies used in August's routing replay. This comparison and that replay are therefore not independent replications. The August exercise examined a saved model's confidence and a hypothetical review queue; here every development prediction is scored, with no review-queue rule applied. Do not infer a queue burden from the macro-F1 comparison.

The 940-document evaluation partition stays closed. We can use development results to investigate the models, but cannot present them as a fixed evaluation result or an approval for intake use. The 126 July holds without a supported gold label remain outside the partitions.

Please retain the predictions and class-level counts against these conditions so we can examine reversals before proposing a use decision. See Gold labels before the first score and Conditions for the August routing replay for the eligibility and replay definitions.
