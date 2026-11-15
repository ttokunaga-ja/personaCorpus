# HF-Eval Usage Constraints

## Permitted use

HF-Eval-S1/S2 v3.2 may be used for internal, synthetic evaluation, error
analysis, and reproducibility checks. Use the complete frozen population for
headline comparisons and cite both version and split when discussing a slice.

## Prohibited use

- Do not treat these records as customer data, user research, or a demographic sample.
- Do not use benchmark text as production input, customer outreach, or account action evidence.
- Do not claim legal, safety, fairness, or service-quality certification from its scores.
- Do not modify rows, labels, or split membership while comparing model runs.
- Do not train on held-out examples and then report the same version as untouched evaluation.

## Reporting requirements

| report element | requirement |
| --- | --- |
| dataset identifier | state HF-Eval-S1/S2 v3.2 |
| population | report 2400 total, S1 1600, S2 800 |
| metric | name aggregation and include counts for narrow slices |
| caveat | state that all records are synthetic |
| changes | identify any successor version or excluded records |

## Escalation

Pause use if a record appears to contain real personal, operational, or
credential-like content. Preserve the audit evidence, restrict access to the
candidate material, and seek a versioned remediation decision before any
further release or comparison.
