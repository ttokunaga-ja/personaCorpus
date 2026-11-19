---
path: Documents/Review-notes/Jul 9 the old line is a method reference.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# Jul 9 — The old line is a method reference

July 9, 2026 · Note to self before laying out the July figure

Reopened *Case-split reevaluation of short intake intent* (MDR-STI-20250417-RM-01-R01). In the April 2025 table, the word-frequency linear classifier scored **0.832 macro-F1** on the **935** case-split evaluation documents. The **0.021** in the row beneath it is the macro-F1 of the training-majority rule on those same 935 documents. That rule assigned New work request to every body. The number in that row is the baseline’s score.

For July, the label next to R184 needs to say R173. We are fitting the word-only model again on the July training partition, then scoring both runs on the same July evaluation bodies. A listener who sees “word-frequency linear classifier” beside the old number could easily assume it is this run.

Reuse the explanation from April: allocate by case, score one body at a time and average the twelve class F1 values with equal weight. Leave April’s numbers out of the July figure. Put the shared July denominator and the full description of R173 next to the eventual scores so the reader can tell what the comparison measures.
