# R184 Error Taxonomy Reading Notes

## Taxonomy purpose

An error taxonomy makes aggregate synthetic metrics inspectable. It groups mistakes by decision mechanism rather than by a generic “incorrect” bucket.

## Categories

| Category | Example review prompt |
| --- | --- |
| Near-neighbor confusion | Did route and resolve share a cue? |
| Abstention miss | Was evidence weak but forced? |
| Underspecified prompt | Was more than one label defensible? |
| Annotation edge | Did the synthetic rule conflict with the phrasing? |

## How to use it

*R184 Case Coding Guide* [synthetic/internal] recommends assigning one primary category before discussing remedies. *Error Slice Sampling Note* [synthetic/internal] asks reviewers to sample from each category rather than only the largest bucket.

## Final observation

The taxonomy should generate targeted experiments; it should not be used to imply a real-world incident pattern.
