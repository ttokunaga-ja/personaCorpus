# Run R184 Threshold Selection Method Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-008009  
**Scope:** p04-secondary-01 / desktop/working  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded threshold selection decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 169 | 0.876 | 0.880 | 0.878 |
| slice-02 | 249 | 0.893 | 0.856 | 0.874 |
| slice-03 | 165 | 0.875 | 0.869 | 0.872 |
| slice-04 | 189 | 0.892 | 0.882 | 0.887 |
| slice-05 | 172 | 0.874 | 0.858 | 0.866 |
| slice-06 | 229 | 0.891 | 0.871 | 0.881 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
