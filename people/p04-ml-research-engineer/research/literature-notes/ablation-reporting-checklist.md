# Ablation Reporting Checklist

## Before the run

- Name the single component to remove.
- Freeze the synthetic evaluation slice and the decision threshold.
- State the hypothesized direction and the metric panel.

## During comparison

| Field | Required record |
| --- | --- |
| Baseline | Full configuration identifier |
| Intervention | One explicit removal or replacement |
| Controls | Threshold, seed policy, and split |
| Result | Macro F1 plus class-level deltas |

## After the run

Explain whether the intervention changes representation, calibration, or abstention behavior. *One-Factor Experiment Sheet* [synthetic/internal] is the reporting model; *Interaction Warning Log* [synthetic/internal] lists confounds that invalidate a simple attribution.

## Checklist result

An ablation earns a conclusion only when a reader can locate the changed factor and the protected controls.
