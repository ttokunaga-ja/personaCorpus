---
path: Documents/短文受付意図分類/Measurements/July scoring handoff checklist.pdf
kind: tex:mercer-research-brief.cls
author: Fatima Rahman
derived_from:
---

# Check the July scoring handoff

Document ID: MDR-STI-20260710-RM-01-R01  
Issued: July 10, 2026  
Author: Fatima Rahman  
For: Vanessa Nguyen

## July 16, before opening the score table

Start with the reference extract for July evaluation. Count the document IDs once in that extract. There should be 1,200 distinct IDs and one valid reference intent per ID, using the twelve categories in the July 2024 revision of the guide. Save the per-intent support from this pass. If the extract repeats an ID or lacks a reference, resolve it against the source before joining predictions. A row count after a join can hide a duplicate that offsets a missing prediction.

Check the R173 and R184 prediction files separately. For each run, check for more than one row per ID, reference IDs missing from the predictions, prediction IDs outside the reference extract and predictions outside the twelve-category list. Join each file back to the same 1,200 references. Each joined document must have exactly one top predicted class per run. Keep the discrepancy lists until we have reconciled them. Hold the comparison if either run fails this check.

## Check the counts before averaging

For each intent, compare TP + FN to the support from the reference-only pass. Check that TP + FP also gives the number of predictions for that intent. Within each run, the sum of TP + FN across intents and the sum of TP + FP must both equal 1,200. The sum of FP must equal the sum of FN and the count of mismatched documents. Each top-label error adds one FP to the predicted class and one FN to the reference class.

After these checks pass, compute 2TP/(2TP + FP + FN) from the counts for each intent. Average the twelve unrounded values with equal weight. For the difference, subtract the unrounded R173 mean from the unrounded R184 mean. Round each mean and the difference separately to three decimal places for reporting.

## Two uses of hold

The annotation guide’s Hold disposition records a body with no resolved reference intent. Its reference label is blank, and it is outside the July labelled collection. In the June routing comparison, every body had a reference and a saved prediction. A confidence cutoff determined which documents would be sent to hold. For July, score the top label for every one of the 1,200 evaluation bodies, regardless of confidence. The June cutoffs and routing counts do not enter this comparison.
