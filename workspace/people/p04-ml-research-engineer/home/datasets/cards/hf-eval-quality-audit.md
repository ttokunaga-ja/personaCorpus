# HF-Eval Quality Audit

## Audit objective

Quality review checks whether the benchmark remains a coherent synthetic
measurement tool: records must be readable, labels must follow the taxonomy,
and split membership must still match the intended baseline-versus-stress
contrast. It does not assess model quality or human behavior.

## Control checks

| control | pass condition | response to failure |
| --- | --- | --- |
| schema | one label, one split, and required audit state present | quarantine record for versioned repair |
| count ledger | split and label totals reconcile to 2400 | halt reporting until ledger is corrected |
| label rationale | ambiguous phrasing has adjudication support | request independent taxonomy review |
| synthetic origin | no copied personal or operational data | remove from candidate material and investigate |
| duplication | no identical prompt-outcome pair within a scenario family | revise wording before release |

## Review cadence

Run these controls on each candidate version and before publishing comparison
results. The quality owner samples both S1 and S2 under the sampling protocol,
while a second reviewer checks disputed label boundaries. Findings are recorded
as correction candidates; v3.2 remains immutable for run comparisons.

## Known measurement limits

Clean synthetic text cannot capture all production variation. The audit therefore
supports internal consistency, not external validity, demographic coverage, or
assurances about live-system safety.
