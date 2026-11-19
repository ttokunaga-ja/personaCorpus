---
path: Downloads/Short intake final methods comments.pdf
kind: office:hartwell-methods-review.docx
author: Nicole Foster
derived_from:
---

# Final methods comments: short intake intent classification

Document ID: HEI-20260909-MR-01-R01  
Issued: September 9, 2026  
Author: Nicole Foster  
For: Vanessa Nguyen

Vanessa, these points refer to the April 29 editor copy, MDR-STI-20260420-RM-01-R02. Please retain the numbers in your response so Eric can trace each point to the revised text.

## N1 — Methods / Partitions and reference

The paragraph says that cases do not cross partitions. Please identify the allocation unit and make the three partition sizes and the pairwise case-boundary check explicit. The reader needs to know what was kept apart, not just the number of documents scored.

## N2 — Methods / Partitions and reference

Please distinguish the unit allocated from the unit scored. State the treatment of the 126 Hold documents and whether both methods use the same evaluation documents and reference labels.

## N3 — Methods / Comparison and scoring

Please identify the class the majority baseline predicts, where its selection comes from, and whether it gives that prediction to every evaluation document. “Majority class” alone can be read as using the evaluation labels to select it.

## N4 — Methods / Comparison and scoring

The macro average needs an explicit order of operations: how each class F1 is calculated, how the twelve classes are weighted, and when rounding occurs. Please state this alongside the comparison.

## N5 — Results / comparison paragraph

Please make the scale, metric, both values and evaluation denominator visible in the figure itself. A figure read separately from the prose should make clear that the two scores refer to the same evaluation documents.

## N6 — Abstract and Methods / Reading selected errors

At first mention, please identify the 160 as selected misclassifications from saved development predictions. The equal-sized development and evaluation partitions make the distinction easy to miss; the reading is not the scoring set or an estimate of its error prevalence.

## N7 — Discussion

The scope examples are useful, but please separate what the text permits a reader to observe from a claim about the cause of a model prediction. Show how the 41 readings relate to the selected 160 without extending the same explanation to the remaining 119.

## N8 — Results / comparison paragraph

Could the July R184 score also be shown in the same results figure, so the reader can see the later result beside the April comparison? Please consider whether the differing evaluation and comparison conditions can be made sufficiently clear there.

## N9 — Methods / Partitions and reference

Please consider supplying the cross-channel case-key correspondence table as a supplement so readers can inspect the allocation boundary. If individual correspondences cannot be shared, explain what aggregate evidence could be reported instead.
