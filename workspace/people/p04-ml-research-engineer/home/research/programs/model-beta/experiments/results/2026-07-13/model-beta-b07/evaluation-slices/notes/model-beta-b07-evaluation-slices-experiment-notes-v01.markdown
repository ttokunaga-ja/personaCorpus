# Model Beta B07 Evaluation Slices Experiment Notes

**Lab:** Helix Forge Lab  
**Record:** p04-full-003679  
**Scope:** p04-primary-06 / research/programs/model-beta/experiments/results  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded evaluation slices decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 189 | 0.866 | 0.854 | 0.860 |
| slice-02 | 211 | 0.883 | 0.867 | 0.875 |
| slice-03 | 241 | 0.865 | 0.880 | 0.872 |
| slice-04 | 179 | 0.882 | 0.856 | 0.869 |
| slice-05 | 167 | 0.864 | 0.869 | 0.866 |
| slice-06 | 179 | 0.881 | 0.882 | 0.881 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
