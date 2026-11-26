# Claims in the abstract — for Michael

February 13, 2025

## Observed in the reference-label work

We read 4,800 received bodies independently and compared the readers' candidates and supporting phrases. Even matching candidates required an evidence check against the body. In the July adjudication, 126 of the 4,800 were held because the text did not establish one intent; the gold-label field stays empty for those documents. The remaining 4,674 have text-supported labels. Hold is a disposition, not a thirteenth intent.

The 126 is an observation about the labeling work. It is neither accuracy on an evaluation set nor a measure of automatic routing performance. Nor is an evidence-based Hold the same decision as sending a model's low-confidence prediction for review.

## Comparison still to be done

Keep the allocation and scoring units separate. Bodies linked by evidence to the same case belong in one partition; score one received body at a time. The planned comparison is the training-partition majority-class baseline against a word-frequency linear classifier. Both must be scored on the same evaluation documents against the same reference labels, using macro-F1 across the twelve classes. The partition and scores are not results to report here.

## Leave out

The application paragraph suggests prioritizing review work and extending the approach across intake desks. Those are questions for later investigation, not conclusions supported by this comparison. Removing the paragraph gives us room to explain what a reader can infer from the text and what happens when the evidence is insufficient.

For the reader's likely question “compared against what?”, name the common reference labels and the same evaluation documents explicitly, without inserting a result.
