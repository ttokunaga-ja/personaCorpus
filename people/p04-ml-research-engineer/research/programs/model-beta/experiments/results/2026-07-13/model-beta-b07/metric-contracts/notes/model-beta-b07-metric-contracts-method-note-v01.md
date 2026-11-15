# Model Beta B07 Metric Contracts Method Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-001500  
**Scope:** p04-primary-06 / research/programs/model-beta/experiments/results  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded metric contracts decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 237 | 0.869 | 0.849 | 0.859 |
| slice-02 | 188 | 0.886 | 0.862 | 0.874 |
| slice-03 | 182 | 0.868 | 0.875 | 0.871 |
| slice-04 | 197 | 0.885 | 0.851 | 0.868 |
| slice-05 | 223 | 0.867 | 0.864 | 0.865 |
| slice-06 | 207 | 0.884 | 0.877 | 0.880 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
