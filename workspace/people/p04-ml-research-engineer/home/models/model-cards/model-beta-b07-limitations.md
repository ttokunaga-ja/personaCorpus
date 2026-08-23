# Model Beta B-07 limitations

Model Beta B-07 is a comparator, not a deployment candidate. Its macro F1 of 0.846 comes from the same fixed synthetic frame used for run R184. It does not establish real-world accuracy, user benefit, or a safe fallback path.

- It may share dataset-specific shortcuts with Model Alpha M-14.
- The macro result hides label-level errors and disagreement severity.
- The evaluation omits robustness, privacy, fairness, and operating-cost measures.

Use Beta B-07 only to preserve a transparent baseline for the Alpha selection rationale.
