---
path: Documents/Conferences-and-funding/Abstract wording check.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Sheet: Adjudication

A2: Abstract wording check

B3: MDR-STI-20250213-AW-01-R01  
E3: February 13, 2025  
B4: Vanessa Nguyen  
E4: Short intake methods abstract — wording for Michael and Fatima

| Ref. | Comparison condition | Evidence source | Status | Draft location | What to verify |
|---|---|---|---|---|---|
| 1 | Received body | R01 abstract, paragraph 2 (reader view) and paragraph 6 (classifier input) | Retain both limits | R01 paragraphs 2 and 6 | Avoid “the message and its routing context are used to classify intent.” Readers see the body and matching identifier without subject, channel, routing tag, outcome or later forwarding comments. The word-frequency classifier takes the received body as input, not the case key or channel. Keep the reader view and classifier input distinct. |
| 2 | Evidence-based Hold | R01 abstract, paragraph 3 and Hold footnote; February 12 read-through record, Discussion | Clarify in prose | R01 paragraph 3 and footnote | Avoid “agreement between readers establishes the gold label” or “Hold means low model confidence.” Even matching candidates need supporting phrases checked against the body. When the text does not establish one intent, record Hold and leave the gold-label field empty; this is a reference-label disposition, not a model-confidence decision. Move the definition out of the footnote. |
| 3 | Case allocation | R01 abstract, paragraph 5; February 6 case split options | Retain evidence and unit | R01 paragraph 5 | Avoid “allocate similar messages independently.” Use receipt records or confirmed alias mappings to link bodies belonging to a case, and assign the linked group to one partition. Similar wording alone does not establish a case link. Check partition pairs for shared cases before scoring. |
| 4 | Scoring unit | R01 abstract, paragraph 4 (Hold excluded from allocation and scoring) and paragraph 6 (each evaluation body scored separately); February 13 claims note, Comparison still to be done | Retain both exclusions and unit | R01 paragraphs 4 and 6 | Avoid “score each case” or “allocate held bodies and omit them only at scoring.” Hold documents remain in the accounting but are outside both the population allocated and the population scored. Among labeled documents in the evaluation partition, count each received body separately for scoring, even when several belong to one case. |
| 5 | Comparison on a common partition | R01 abstract, paragraph 6; February 13 claims note, Comparison still to be done | Make common references explicit | R01 paragraph 6 | Avoid “compare two methods on their respective evaluation sets.” Compare the training-partition majority-class baseline and word-frequency linear classifier on the same labeled evaluation bodies and against the same reference labels. Use macro-F1 across the twelve intents; do not imply that the case key is a classifier feature. |
| 6 | Status of results | R01 abstract, paragraphs 4 and 6; February 13 claims note, Observed and Comparison still to be done | Keep observation separate from plan | R01 paragraphs 4 and 6 | Avoid “126 held means the classifier achieved this result” or “the new comparison showed an improvement.” The 126 of 4,800 is an observation from reference-label adjudication. The case-based comparison has not yet been scored; describe it as planned and give no performance result. |

A12: Michael and Fatima — please check whether the shortened wording preserves these distinctions in the plain-text field. The schedule pair and the Hold definition need to stand in prose without the draft table or footnote.
