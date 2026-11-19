# Why ninety-two is still a queue

May 28, 2025

Joshua's ledger check gives us a place to start, not a comparison result. Of the 96 additional intake documents, 68 trace directly to a receipt record and 24 through a confirmed alias. Those 92 have passed the provenance check and remain candidates. The other four stay out: two lack a reference to a receipt record and two lack evidence for the alias correspondence.

I keep wanting to call the 92 a test set. That skips two questions:

1. **Are the partitions independent by case?** Trace the candidates using receipt records or confirmed aliases, group documents belonging to the same case, and check for shared cases across every partition pair. A traceable document can still share a case with another partition. If a link cannot be established, leave the affected item out until the evidence is resolved; do not infer a link from a similar subject alone.
2. **Does the evaluation partition support all 12 intents?** Check the reference-label support before opening the evaluation partition for scoring. If a class has no support, the proposed partition is not ready under our conditions. Do not score and then redraw because the result is inconvenient.

Once the case grouping and support are established, freeze the partitions and the comparison conditions. Fatima can then score both models against the same fixed evaluation documents and reference labels, with the same 12-class macro-F1 calculation: calculate each class's F1 from TP, FP and FN, average the unrounded values equally, and round at the end. Selection or adjustment belongs to the development partition, not the evaluation results. Until then, no ranking.

The April 3 fixed evaluation and its 935 documents are a separate record. The 92 are additional candidates, not a replacement denominator for the April 0.832, and the four unresolved records do not explain the difference from the withdrawn January score. See the May 23 ledger decisions and Fatima's *Scoring the frozen intake set* for the decisions and scoring procedure respectively.
