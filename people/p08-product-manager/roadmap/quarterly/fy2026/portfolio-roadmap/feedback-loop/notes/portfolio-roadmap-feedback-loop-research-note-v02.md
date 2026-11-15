# Portfolio Roadmap Feedback Loop Research Note

**Lab:** Juniper Works  
**Record:** p08-full-006565  
**Scope:** p08-primary-05 / roadmap/quarterly  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded feedback loop decision for run M-27. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Product Alpha reached F1 **0.873**, precision **0.881**, and recall **0.865** on Product Alpha Q3 / Product Beta Q4. Model Product Beta reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 18.4% adoption synthetic-only.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 234 | 0.891 | 0.849 | 0.869 |
| slice-02 | 186 | 0.873 | 0.862 | 0.867 |
| slice-03 | 228 | 0.890 | 0.875 | 0.882 |
| slice-04 | 173 | 0.872 | 0.851 | 0.861 |
| slice-05 | 211 | 0.889 | 0.864 | 0.876 |
| slice-06 | 249 | 0.871 | 0.877 | 0.874 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run M-27 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
