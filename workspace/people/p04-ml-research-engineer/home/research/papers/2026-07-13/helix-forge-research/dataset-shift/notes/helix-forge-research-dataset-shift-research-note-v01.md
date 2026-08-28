# Helix Forge Research Dataset Shift Research Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-000151  
**Scope:** p04-primary-01 / research/papers  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded dataset shift decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 172 | 0.889 | 0.857 | 0.873 |
| slice-02 | 250 | 0.871 | 0.870 | 0.870 |
| slice-03 | 165 | 0.888 | 0.883 | 0.885 |
| slice-04 | 236 | 0.870 | 0.859 | 0.864 |
| slice-05 | 243 | 0.887 | 0.872 | 0.879 |
| slice-06 | 217 | 0.869 | 0.848 | 0.858 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
