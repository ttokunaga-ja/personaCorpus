---
path: Documents/Review-notes/Mar 19 - Case split size tradeoffs.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# Equal sizes are not the gate — March 19

Emily and I looked at the proposed case allocation today. The target was about 2,805 training, 934 development and 935 evaluation documents. One way to get closer would be to split a large case across partitions. We rejected that: a neater count would reintroduce the very dependence the new allocation is meant to avoid.

The targets are guides, not acceptance limits. First keep every document in a case in one partition and check all three partition pairs for shared cases. Also check that each of the twelve reference classes has support in the proposed evaluation partition before it is fixed. If the bundle constraints change the counts, record the actual counts and keep the boundaries intact. Do not use evaluation scores to choose a more attractive allocation.
