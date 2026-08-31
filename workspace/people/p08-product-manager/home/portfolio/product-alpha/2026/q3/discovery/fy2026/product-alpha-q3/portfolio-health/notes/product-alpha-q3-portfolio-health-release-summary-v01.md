# Product Alpha Q3 Portfolio Health Release Summary

**Lab:** Juniper Works  
**Record:** p08-full-001039  
**Scope:** p08-primary-02 / portfolio/product-alpha/2026/q3/discovery  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded portfolio health decision for run M-27. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Product Alpha reached F1 **0.873**, precision **0.881**, and recall **0.865** on Product Alpha Q3 / Product Beta Q4. Model Product Beta reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 18.4% adoption synthetic-only.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 173 | 0.890 | 0.876 | 0.883 |
| slice-02 | 251 | 0.872 | 0.852 | 0.862 |
| slice-03 | 166 | 0.889 | 0.865 | 0.877 |
| slice-04 | 189 | 0.871 | 0.878 | 0.874 |
| slice-05 | 172 | 0.888 | 0.854 | 0.871 |
| slice-06 | 229 | 0.870 | 0.867 | 0.868 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run M-27 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
