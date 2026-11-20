---
path: Documents/短文受付意図分類/Research-questions/Intent support requirements.md
kind: text
author: Vanessa Nguyen
derived_from:
---
# How many support examples do we need?

April 19, 2024

Support here means the number of reference documents belonging to a class in the evaluation partition, not the number the model predicts for it. Keep the twelve supports next to the twelve F1 values. The average is one twelfth of their sum: a common class does not get extra weight just because it has more documents. A model can correctly predict many common requests and still have a poor macro-F1 if it misses rare intents.

A handful of examples in a rare class makes its score sensitive to one or two errors. Before fixing the evaluation partition, inspect each class count and the available evidence for its labels. This is a reason to describe uncertainty and class-level results, not to choose a convenient score.

If a class has zero reference examples in the proposed evaluation partition, do not fill in an arbitrary class score to complete the twelve-class average. Revisit the partition before freezing it; then preserve the same fixed evaluation documents for both the majority baseline and the word-frequency linear model. The 0.82 in the plan remains a first-year target.
