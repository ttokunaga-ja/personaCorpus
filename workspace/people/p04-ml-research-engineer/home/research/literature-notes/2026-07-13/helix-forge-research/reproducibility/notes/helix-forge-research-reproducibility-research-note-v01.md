# Helix Forge Research Reproducibility Research Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-007664  
**Scope:** p04-primary-02 / research/literature-notes  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded reproducibility decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 245 | 0.862 | 0.854 | 0.858 |
| slice-02 | 192 | 0.879 | 0.867 | 0.873 |
| slice-03 | 184 | 0.896 | 0.880 | 0.888 |
| slice-04 | 198 | 0.878 | 0.856 | 0.867 |
| slice-05 | 176 | 0.895 | 0.869 | 0.882 |
| slice-06 | 184 | 0.877 | 0.882 | 0.879 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
