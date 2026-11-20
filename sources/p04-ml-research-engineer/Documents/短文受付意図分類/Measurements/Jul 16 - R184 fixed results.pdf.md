---
path: Documents/短文受付意図分類/Measurements/Jul 16 - R184 fixed results.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# M-14 | R184 fixed evaluation results

Document ID: MDR-STI-20260716-RM-01-R01  
Issued: July 16, 2026  
Author: Vanessa Nguyen  
For: Michael Serrano and the research team

Fatima Rahman and I reviewed the evaluation conditions and scoring at 1:30 p.m. PDT and froze the comparison at 2:20 p.m. R184 scored 0.873 macro-F1 on the July evaluation. Its difference against R173 on those same documents is +0.021 macro-F1.

## Partition and reference

M-14 uses a July collection of 6,000 reference-labelled received bodies. The allocation unit is a case, using a cross-channel key supported by receipt records and confirmed aliases. One received body counts as one document for scoring. We reconciled the document totals from the channel counts: 3,600 training, 1,200 development and 1,200 evaluation.

Joshua Ramirez reported on July 7 that he found zero shared cases in each of the training–development, training–evaluation and development–evaluation pairs. The research team did not have the case-level allocation detail to reproduce that check. The channel totals were available to us and establish the sizes by document. The scoring checks below used the evaluation references and the two runs' predictions.

Both runs use the same reference for each of the 1,200 evaluation documents. The reference labels follow the July 2024 annotation guide (MDR-STI-20240513-GD-01-R02). Each of its twelve intents has support. The 126 old Hold bodies without a reference label are outside the July evaluation.

## Models and scoring

R173 and R184 are regularized linear classifiers trained on the same July training partition. R173 uses word frequencies and R184 adds character 3–5 grams from the same received body. The subject, channel, case key and later staff notes are excluded from the features. The pair was set on July 8 using the development partition before looking at the evaluation results. Each run gives one top-ranked intent for every evaluation body, with no confidence threshold.

For each class, F1 is 2TP/(2TP+FP+FN). Fatima reconciled TP + FN to the common reference support for each intent and checked that total FP and FN matched within each run. The 12 unrounded class scores receive equal weight. The means from the counts are approximately 0.852005305 for R173 and 0.872784697 for R184. We subtract the unrounded means (R184 minus R173) before rounding the difference. Each displayed value is rounded to three decimal places.

| Run and features | Evaluation documents | Macro-F1 | Difference vs. R173 |
|---|---:|---:|---:|
| R173 · word frequencies | 1,200 | 0.852 | 0.000 |
| R184 · word frequencies + character 3–5 grams | 1,200 | 0.873 | +0.021 |

The average rose while several individual intents fell. Reschedule had 100 reference documents. Its F1 went from 0.888 with R173 to 0.829 with R184. The number of missed Reschedule references rose from 9 to 20. That change in the totals does not identify which individual documents changed predictions.

## Reporting scope

The April 2025 word-frequency score of 0.832 belongs to a separate evaluation of 935 documents. The figure for this comparison should use the two July runs only. This freeze records a research comparison. We did not make a decision on operational use or measure recovery time after a wrong route.

Use *R184 evaluation conditions.xlsx* for the conditions, *R184 results by intent.csv* to reproduce the means from the counts, and *R184 run comparison.tsv* for the two displayed result rows.
