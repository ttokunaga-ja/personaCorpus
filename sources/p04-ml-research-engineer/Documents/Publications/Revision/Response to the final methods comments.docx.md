---
path: Documents/Publications/Revision/Response to the final methods comments.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# Response to the final methods comments

Document ID: MDR-STI-20260917-RM-01-R01  
Issued: September 17, 2026  
Author: Vanessa Nguyen  
For: Eric Sandoval

Eric,

The numbered responses below refer to Nicole Foster's final methods comments, HEI-20260909-MR-01-R01, on the April 29 editor copy, MDR-STI-20260420-RM-01-R02. The revised manuscript is MDR-STI-20260420-RM-01-R03, issued September 17. I have retained Nicole's numbering so that you can move between her comments, this response and the manuscript. N1–N7 are reflected in the manuscript; I have not added the materials proposed in N8 and N9 for the reasons below.

## N1 — Case allocation

**R03: Methods / Case Allocation.** The revised text names the case as the allocation unit and gives the three partition sizes: 2,804 training, 935 development and 935 evaluation documents. It also reports the pairwise boundary check: training–development, training–evaluation and development–evaluation each shared zero cases. The counts describe documents; the boundary check concerns cases.

## N2 — Scoring set

**R03: Methods / Scoring Set.** One received intake body is the scoring unit. The 126 Hold documents are outside allocation and scoring. Both methods are scored against the same twelve-class reference labels on the same fixed 935 evaluation documents. This separates the documents scored from the cases used to allocate them.

## N3 — Comparator

**R03: Methods / Comparator.** The majority-class baseline predicts New work request for every evaluation document. That class is the most frequent in the training partition; it is not selected from the evaluation labels. The linear classifier uses word frequencies from the received body only, and its feature and setting choices were completed on development data.

## N4 — Macro-F1

**R03: Methods / Macro-F1.** For each of the twelve classes, F1 is calculated as 2TP/(2TP+FP+FN). The twelve unrounded class F1 values are averaged with equal weight, and the resulting macro-F1 is rounded only after the average is calculated. This order applies to both methods in the comparison.

## N5 — Figure and denominator

**R03: Results / Figure 1.** The figure now labels the metric as twelve-class macro-F1 and uses a common vertical axis from 0 to 1. The bars are labelled 0.021 for the training-majority baseline and 0.832 for the word-frequency linear classifier; the figure also states that both use the same 935 evaluation documents. The values and denominator can therefore be read with the figure rather than inferred from the adjacent paragraph.

## N6 — Diagnostic reading

**R03: Abstract and Methods / Diagnostic Reading.** At first mention, the 160 are identified as selected misclassifications from saved predictions for the development partition. They were selected for a separate diagnostic reading, not for scoring the comparison; they do not estimate the prevalence of errors in the fixed 935-document evaluation set. The distinction matters because the development and evaluation partitions happen to have the same document count.

## N7 — Scope examples

**R03: Discussion / Scope Examples.** The revision presents 41 of the selected 160 as readings in which negation scope mattered: 18 reject an action, 14 negate an attribute and 9 request an alternative action. These are observations from reading the text, not evidence of what caused a model prediction. The other 119 are not attributed to the same scope account; their relevant wording must be considered individually.

## N8 — July result

**R03: Results / Figure 1; not added.** I have kept the April comparison in the figure. July's R184 score of 0.873 comes from a different evaluation set of 1,200 documents and a different comparator. Placing it next to the April 0.832 could imply a same-set comparison or a measurable effect of the later features that those results cannot establish. The July comparison is recorded separately under its own conditions.

## N9 — Case-key correspondence

**R03: Methods / Case Allocation; correspondence table not added.** The cross-channel case-key table records internal correspondences between sources and remains an internal record. Instead, the manuscript reports the aggregate boundary result: zero shared cases for each of the three partition pairs, alongside the partition document counts. If inspection of individual correspondences is required, we should discuss their handling separately before any release.
