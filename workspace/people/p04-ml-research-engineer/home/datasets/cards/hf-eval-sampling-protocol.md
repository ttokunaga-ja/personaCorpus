# HF-Eval Sampling Protocol

## Frozen evaluation population

HF-Eval-S1/S2 v3.2 is a fixed 2,400-record evaluation population. Sampling is
only used for manual audit or error inspection; it does not replace the full-set
metric. Draw reviews within split and label so S2's stress cases remain visible
instead of being submerged by S1.

## Audit draw procedure

1. Select a review question and name the target split(s).
2. Stratify by the five primary labels.
3. Draw records without replacement using an internal deterministic index.
4. Preserve record order, selection criteria, and reviewer decision in the audit log.
5. Escalate label disagreements; do not alter benchmark rows during review.

## Minimum coverage

| review target | minimum draw | required coverage |
| --- | ---: | --- |
| routine label check | 25 | all five labels in S1 |
| stress wording check | 25 | all five labels in S2 |
| error triage | 10 per affected label | predicted and reference label noted |

When a stratum contains fewer requested examples, include all available records
and report the denominator. Samples are diagnostic artifacts and cannot be
combined into a replacement macro F1.

## Reproducibility note

The selection index is derived from dataset version, split, label, and review
purpose. That preserves repeatable coverage without exposing any real-world
source material, because all records are synthetic.
