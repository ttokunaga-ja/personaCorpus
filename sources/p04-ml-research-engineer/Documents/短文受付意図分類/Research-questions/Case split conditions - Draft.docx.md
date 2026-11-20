---
path: Documents/短文受付意図分類/Research-questions/Case split conditions - Draft.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# What the new split must hold fixed

Document ID: MDR-STI-20250212-RM-01-R01  
Issued: February 12, 2025  
Author: Vanessa Nguyen  
For: Fatima Rahman

## Question

Fatima, I suggest we retain the original question—what intent is supported by a received short message—while replacing the document-level allocation. The January score is withdrawn: documents from the same case appeared in different partitions. We cannot establish independence by simply removing the 84 identified documents from evaluation.

## What stays

Keep the twelve intent categories and the July 18 revised annotation guide. A reference label needs a supporting phrase in the received body; the 126 Hold documents remain outside scoring with their gold labels blank. The 4,674 labeled candidates remain the allocation population, including documents involved in the crossing. One received body remains one scoring unit. The classifier sees the received body only, not the subject, route, handling outcome or later forwarding comment.

## Proposed allocation

Use a case bundle across the three intake channels as the allocation unit. The February 6 comparison showed that a channel-only key reunited only 17 of 24 known cases, while the cross-channel key reunited all 24. Recover missing case keys from related documents at the same channel. Keep all documents assigned to one case together in training, development or evaluation. Check for shared cases in each pair and for reference support for all twelve categories in the proposed evaluation partition. Fix the allocation before opening evaluation for scoring.

Use training for fitting and development for choices. Compare the word-frequency linear classifier with the training-partition majority-class baseline on the same evaluation documents and reference labels. Report support and per-class F1 along with macro-F1. The evaluation results should not drive further model choices.

## For review

Please check whether the key recovery description gives Joshua enough direction and whether the support check is placed early enough. I have not yet specified the allocation counts or opened a new evaluation result.
