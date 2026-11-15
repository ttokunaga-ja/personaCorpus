# Model Beta B07 Evaluation Slices Assurance Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-007037  
**Scope:** p04-primary-05 / research/programs/model-beta/experiments/configs  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded evaluation slices decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 201 | 0.863 | 0.860 | 0.861 |
| slice-02 | 217 | 0.880 | 0.873 | 0.876 |
| slice-03 | 244 | 0.862 | 0.849 | 0.855 |
| slice-04 | 228 | 0.879 | 0.862 | 0.870 |
| slice-05 | 191 | 0.896 | 0.875 | 0.885 |
| slice-06 | 239 | 0.878 | 0.851 | 0.864 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
