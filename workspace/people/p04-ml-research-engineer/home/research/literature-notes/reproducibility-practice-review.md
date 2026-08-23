# Reproducibility Practice Review

## Reproducibility target

For this fictional study, reproducibility means an internal reader can reconstruct the stated synthetic comparison from a run card, dataset card, and analysis script without hidden retrieval or live services.

## Evidence chain

1. Record the versioned dataset statement: HF-Eval-S1/S2 v3.2, 2,400 records.
2. Record the evaluation snapshot and metric definitions.
3. Preserve a deterministic synthetic example for every reporting transformation.

## Failure modes

*Rebuildability Checklist* [synthetic/internal] calls undocumented preprocessing a break in the chain. *Metric Contract Note* [synthetic/internal] calls an unlabeled aggregation convention a second break.

## Review decision

The R184 materials are useful only if a reviewer can distinguish an observed value from the code and assumptions that produced it.
