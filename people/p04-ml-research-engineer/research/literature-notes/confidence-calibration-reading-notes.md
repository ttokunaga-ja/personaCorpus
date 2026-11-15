# Confidence Calibration Reading Notes

## Calibration claim

A confidence score is well calibrated when an 0.8 band is correct about eight times in ten within the stated synthetic slice. This relation concerns probability quality, not whether the chosen label is useful.

## Reliability-bin interpretation

| Bin | What to record | What not to infer |
| --- | --- | --- |
| 0.9-1.0 | Count and observed accuracy | Universal trustworthiness |
| 0.7-0.8 | Label composition | A threshold recommendation by itself |
| below 0.5 | Abstention behavior | A model failure without context |

## References in this note

*Reliability Diagram Field Guide* [synthetic/internal] proposes plotting count with accuracy. *ECE Caveat Memo* [synthetic/internal] notes that a small aggregate calibration error can coexist with a serious single-label defect.

## Practical conclusion

Review confidence separately for S1 and S2 before treating the R184 score panel as a policy input.
