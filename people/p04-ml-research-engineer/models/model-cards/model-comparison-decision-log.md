# Model comparison decision log

## Decision

On 2026-07-13, Helix Forge Lab selected Model Alpha M-14 for bounded offline synthetic-intent research.

| Candidate | Evaluation frame | Macro F1 | Decision |
|---|---|---:|---|
| Model Alpha M-14 | HF-Eval-S1/S2 v3.2; 2,400 synthetic records | 0.873 | Selected with controls |
| Model Beta B-07 | Same fixed frame | 0.846 | Retained as comparator |

## Rationale

The 0.027 macro F1 difference favors Alpha under run R184. Alpha also recorded precision 0.881 and recall 0.865. This is a controlled comparison, not a release authorization.

## Follow-up gate

Any altered dataset version, taxonomy, prompt policy, or model revision requires a new decision record.
