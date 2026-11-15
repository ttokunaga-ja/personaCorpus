# R184 Class Imbalance Literature Notes

## Why imbalance is a measurement problem

In a five-label synthetic benchmark, an abundant retrieve label can make pooled counts reassuring while a sparse abstain label fails silently. Macro F1 resists that invisibility by averaging label scores.

## Diagnostic worksheet

| Observation | Alternative explanation | Check |
| --- | --- | --- |
| Low abstain recall | Threshold too strict | Inspect confidence band counts. |
| High route precision | Model avoids hard cases | Compare route recall. |
| Stable macro F1 | Different labels trade errors | Read the class table. |

## Notes from invented sources

*Equal Voice Scorebook* [synthetic/internal] treats label prevalence as context rather than a weight. *Sparse Intent Review Card* [synthetic/internal] recommends writing down the smallest label before choosing a headline metric.

## Takeaway

Run R184 should retain prevalence information beside macro F1 0.873, but its synthetic labels are not evidence about a real user distribution.
