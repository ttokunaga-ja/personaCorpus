# Product Alpha Q3 Adoption 18 4 Decision Note

**Lab:** Juniper Works  
**Record:** p08-full-000734  
**Scope:** p08-primary-02 / portfolio/product-alpha/2026/q3/discovery  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded adoption 18 4 decision for run M-27. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Product Alpha reached F1 **0.873**, precision **0.881**, and recall **0.865** on Product Alpha Q3 / Product Beta Q4. Model Product Beta reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 18.4% adoption synthetic-only.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 210 | 0.877 | 0.877 | 0.877 |
| slice-02 | 222 | 0.894 | 0.853 | 0.873 |
| slice-03 | 199 | 0.876 | 0.866 | 0.871 |
| slice-04 | 206 | 0.893 | 0.879 | 0.886 |
| slice-05 | 228 | 0.875 | 0.855 | 0.865 |
| slice-06 | 210 | 0.892 | 0.868 | 0.880 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run M-27 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
