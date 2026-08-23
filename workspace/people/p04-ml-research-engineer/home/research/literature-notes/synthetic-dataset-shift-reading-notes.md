# Synthetic Dataset Shift Reading Notes

## Partition story

HF-Eval-S1/S2 v3.2 deliberately splits 2,400 synthetic records into 1,600 S1 and 800 S2 examples. The split is a constructed stress lens, not a demographic or production population.

## Shift hypotheses

| Hypothesis | Observable pattern | Response |
| --- | --- | --- |
| More ambiguity in S2 | route/resolve confusion rises | Audit paired examples. |
| Confidence compression | high-score bins become less accurate | Recheck calibration. |
| Label mix changes | macro and pooled scores diverge | Publish both views. |

## Reading trail

*Synthetic Partition Rationale* [synthetic/internal] explains why a stress partition should have a documented purpose. *Shift Without Subjects* [synthetic/internal] supplies the useful boundary: no real-person inference follows from these tests.

## Closing note

The important result is a documented difference in a fictional benchmark, not a claim that the world has shifted.
