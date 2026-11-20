---
path: Pictures/Charts/Cutoff routing flows.png
kind: diagram
author: Vanessa Nguyen
derived_from:
---

# What came off the review shelf

A flow diagram starts at “Held at cutoff 0.60: 144 documents” and splits into two equally wide branches, each proportional to its 72 documents. Each branch is divided into segments proportional to correct and incorrect saved predictions.

| Destination at cutoff 0.45 | Correct | Incorrect | Total |
|---|---:|---:|---:|
| Still held | 57 | 15 | 72 |
| Newly automatic | 65 | 7 | 72 |

Show each count in its segment and label each branch with its destination and total. Legend: “Correct saved prediction” and “Incorrect saved prediction” use distinct fills. Note: “This diagram follows the 144 documents held at 0.60, a subset of the 800 scored documents. The other 656 were already automatic and are not shown.” Correctness compares the saved prediction with the reference label before hold.
