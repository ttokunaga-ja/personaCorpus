# Model Alpha M14 Quality Assurance Research Note

**Lab:** Helix Forge Lab  
**Record:** p04-full-000859  
**Scope:** p04-primary-04 / research/programs/model-alpha/experiments/results  
**Evaluation date:** 2026-07-13

## Purpose

This research-engineering note captures a bounded quality assurance decision for run R184. It keeps dataset lineage, comparison criteria, and operational follow-up together so that a second engineer can reproduce the interpretation without relying on chat history.

## Evaluation position

Model Alpha M-14 reached F1 **0.873**, precision **0.881**, and recall **0.865** on HF-Eval-S1/S2 v3.2. Model Beta B-07 reached F1 **0.846** under the same split and scoring policy. The evaluated population contains 2,400 synthetic records.

| Slice | Records | Precision | Recall | F1 |
|---|---:|---:|---:|---:|
| slice-01 | 227 | 0.874 | 0.862 | 0.868 |
| slice-02 | 230 | 0.891 | 0.875 | 0.883 |
| slice-03 | 203 | 0.873 | 0.851 | 0.862 |
| slice-04 | 208 | 0.890 | 0.864 | 0.877 |
| slice-05 | 229 | 0.872 | 0.877 | 0.874 |
| slice-06 | 163 | 0.889 | 0.853 | 0.871 |

## Engineering interpretation

The run is accepted as a research checkpoint, not as a production deployment. The main sensitivity is threshold calibration across sparse slices. Reviewers should compare aggregate and slice-level behavior, confirm the dataset-card digest, and keep the decision tied to run R184 rather than generalizing beyond the observed evaluation.

## Lineage and follow-up

No upstream artifact is required for this bounded record.

The next review will re-run the same notebook logic with the frozen split, verify that the environment record still resolves, and compare calibration drift before any promotion decision. This record is synthetic research material and makes no claim about live traffic, people, credentials, or external systems.
