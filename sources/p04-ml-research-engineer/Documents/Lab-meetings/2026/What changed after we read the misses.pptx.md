---
path: Documents/Lab-meetings/2026/What changed after we read the misses.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1

Layout: Cover

# What changed after we read the misses

MDR-STI-20260122-LM-01-R01 | Issued: January 22, 2026 | Author: Vanessa Nguyen

Presenter notes: For Fatima and the research team. I want to separate a change in our explanation from a change in the evaluation.

## Slide 2

Layout: Two Columns

# Two partitions, two questions

Left — Fixed evaluation
- 935 documents, assigned by case.
- Used for the fixed comparison and its reported scores.

Right — Development
- 935 documents, assigned by case, with no cases shared with evaluation.
- 160 misclassified documents selected from saved development predictions for close reading.

Presenter notes: The two 935-document partitions contain different cases. The 160 are selected examples from development, not evaluation errors and not a denominator for evaluation performance. We selected them to examine reference and prediction combinations, not to estimate their frequency.

## Slide 3

Layout: Table

# What we separated on January 12

| Agreed reading | Documents | What needed to be distinguished |
|---|---:|---|
| Action rejection | 18 | An action refused versus the action requested |
| Attribute negation | 14 | A rejected attribute, such as Friday, versus the action itself |
| Alternative request | 9 | An explicitly requested alternative action |
| Other readings | 119 | Evidence specific to each body |

Caption: Vanessa and Nicole read 160 selected development errors; each of the 41 scope cases has one category.

Presenter notes: We read the body before comparing the reference and prediction. The 41 share a reading question about what the negation applies to. This does not show that the classifier used a particular feature or that negation caused its prediction.

## Slide 4

Layout: Figure

# Where did our initial flags differ?

Figure: Pictures/Charts/Where the readers differed before discussion.png

Caption: Initial scope flags, grouped by the reading agreed after discussion. Scope group: 29 agreed, 12 differed (n=41). Other readings: 102 agreed, 17 differed (n=119).

Presenter notes: Agreement here is between Nicole and me on the initial scope flag. It is not whether the classifier was correct. The groups reflect the later discussion; the bars describe the initial flags. Agreement alone does not establish that a reading is correct.

## Slide 5

Layout: Two Columns

# A counterexample to the single explanation

Left — Scope matters
“Don't cancel; move it to Tuesday.” Cancellation is rejected; a change of date is requested.

Right — A different reading problem
“Has the crew been yet?” There is no negation. The body may require distinguishing progress from visit timing.

Presenter notes: The second sentence is a counterexample to explaining every miss by misplaced negation. Its reference label depends on the rest of the body. The 119 should be examined on their own evidence; this example does not classify all of them.

## Slide 6

Layout: Heading and Body

# Change the discussion, keep the comparison fixed

- Withdraw the single explanation for all 160 selected errors.
- Give the 41 scope readings their own discussion; retain evidence for the other 119 individually.
- Keep the original scoring and reference labels frozen.
- Treat the reading as an observation, not a demonstrated model mechanism.

Presenter notes: I will rewrite the discussion for Fatima. We have not changed the fixed evaluation or relabelled these documents. A causal account of the model would require a separate controlled comparison.
