# Juniper Works Portfolio Health Release Summary

**Lab:** Juniper Works  
**Record:** p08-full-004731  
**Scope:** p08-secondary-01 / desktop/working  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded portfolio health decision for run M-27. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Product Alpha reached F1 **0.873**, precision **0.881**, and recall **0.865** on Product Alpha Q3 / Product Beta Q4. Model Product Beta reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 18.4% adoption synthetic-only.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 220 | 0.872 | 0.872 | 0.872 |
| slice-02 | 227 | 0.889 | 0.848 | 0.868 |
| slice-03 | 249 | 0.871 | 0.861 | 0.866 |
| slice-04 | 183 | 0.888 | 0.874 | 0.881 |
| slice-05 | 169 | 0.870 | 0.850 | 0.860 |
| slice-06 | 228 | 0.887 | 0.863 | 0.875 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run M-27 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
