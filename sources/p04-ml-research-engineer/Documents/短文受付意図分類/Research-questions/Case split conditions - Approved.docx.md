---
path: Documents/短文受付意図分類/Research-questions/Case split conditions - Approved.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# What the new split must hold fixed

Document ID: MDR-STI-20250212-RM-01-R02  
Issued: March 12, 2025  
Author: Vanessa Nguyen  
For: Short intake research team

## Decision and scope

We will replace the document-level allocation with an evidence-backed case allocation before repeating the short intake comparison. The January macro-F1 of 0.851 is withdrawn: documents from the same cases crossed partition boundaries, so the old evaluation cannot support an independent comparison. Removing only the 84 identified documents does not establish that the remaining old allocation is independent, and does not undo their use in training.

This revision incorporates Fatima's February 14 review of the case-key evidence and the timing of the class-support check. It specifies the conditions for the new evaluation; it does not report a new score or a frozen allocation.

## Population and question held fixed

The question remains which of the twelve intake intents is supported by a received short message. Use the July 18 revised annotation guide and its requirement for a supporting phrase in the received body. The 4,674 documents with reference labels are candidates for allocation, including documents in the identified crossing cases. The 126 Hold documents remain outside allocation and scoring, with reference labels blank. A case is the allocation unit; one received body is the scoring unit.

The classifier receives the received body only. Subject, channel, case key, routing information, handling outcome and later forwarding comments are not classifier features. The ledger retains the identifiers and evidence needed to audit the allocation separately from the model input.

## Establish case bundles

Joshua's case-key guidance, MDR-STI-20250220-DD-01-R01, describes the evidence to retain. Use a cross-channel case key supported by the original receipt record or a verified alias-table entry. Record the document and receipt references and, where applicable, the confirmed alias entry. Similar subjects or a shared channel alone do not establish a link. Leave an unsupported link unresolved and identify the missing evidence for investigation; do not guess a key to complete an allocation.

The known-case check reunited 17 of 24 crossing cases with channel-specific keys and all 24 with the evidence-backed cross-channel keys. This supports the choice of key; it is not a model score or proof that every other link is complete. Resolve the allocation population's case links with the recorded evidence before fixing the split.

Assign every document belonging to a case to one partition: training, development or evaluation. Check each pair of partitions for shared case keys; all three pairwise counts must be zero. Do not split a case simply to make partition sizes closer to a target. Account for each labeled candidate once, and keep Hold outside all three partitions.

## Gates before scoring

Before opening the evaluation partition for scoring, check that its reference labels have support for each of the twelve intents under the July guide. Record the support counts and the case-overlap checks with the proposed allocation. If any class lacks support or a case crosses a boundary, revise the proposed allocation and repeat the checks before freezing it. Record the fixed assignments and reference labels once the gates are met. Do not redraw after scoring to improve class support or a result; any necessary change after that point requires a documented new evaluation condition.

## Fitting and comparison

Fit the word-frequency linear classifier on the training partition. After the partitions are fixed, use only the development partition for feature and setting choices; do not use evaluation outcomes to adjust them. Determine the majority class from the training partition and use it as a baseline prediction for every evaluation document.

Score both the baseline and the linear classifier on the same fixed evaluation documents against the same reference labels. Report the evaluation document count, reference support and F1 for each intent, and macro-F1 as the equal-weight mean of the twelve unrounded per-intent F1 values, rounded at the end. For each intent, F1 is 2TP / (2TP + FP + FN). A baseline that never predicts a supported intent has F1 zero for that intent. Keep the allocation record and scoring conditions with the report so the two results can be read within the same comparison.

## References

- Short intake intent research plan, MDR-STI-20240416-RP-01-R01.
- Where the case key came from, MDR-STI-20250220-DD-01-R01.
- Short intake annotation guide, MDR-STI-20240513-GD-01-R02, issued July 18, 2024.
