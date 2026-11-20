---
path: Documents/Lab-meetings/2026/Jul 16 - R184 talk slides.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1 — Cover

**Short intake intent | The July comparison**

Document ID: MDR-STI-20260715-LM-01-R02 · Issued: July 16, 2026 · Author: Vanessa Nguyen

Speaker notes: Michael, I'd like your approval to use these slides and the accompanying figure in the Applied Document Research Forum oral talk. After yesterday's read-through with Emily, I've moved the evaluation set and the name of the comparison run ahead of the figure. Open with the question we set for M-14: what changes when we add short character sequences to the same intake text?

## Slide 2 — Heading and Body

**One July evaluation set for both runs**

- July collection: 6,000 labelled received bodies. Whole cases were assigned to training (3,600 documents), development (1,200) or evaluation (1,200).
- Both runs are scored on the same 1,200 evaluation documents. One received body counts as one document.
- Joshua Ramirez reported zero shared cases across each pair of partitions on July 7.

Speaker notes: The cross-channel case key keeps requests from the same case together at allocation. Joshua checked for shared cases across training and development, training and evaluation, and development and evaluation. We had his report and the document totals by channel. We checked the totals ourselves. We did not have the case-level allocation detail to repeat his overlap check. Fatima and I used the common evaluation references and the two runs' predictions to check the scoring.

## Slide 3 — Table

**What R173 and R184 see and how we score them**

| Condition | R173 · comparison run | R184 · candidate |
|---|---|---|
| Fit | Regularized linear classifier, July training set | Same classifier family and training set |
| Input | Word frequencies from the received body | Word frequencies and character 3–5 grams from the received body |
| Scoring | One top intent per body, no confidence cutoff | Same rule and same 1,200 reference labels |

Table note: Compute F1 per intent, then average all 12 intents with equal weight.

Speaker notes: We chose the pair on July 8 using the development partition, before looking at the evaluation results. Both feature sets use only the received body. Subjects, channels, case keys and later staff notes stay out. Fatima checked that true positives plus false negatives matched the common reference support for each intent. We recomputed each class score as 2TP/(2TP+FP+FN), averaged the twelve unrounded scores and took the difference of the unrounded means. Round the reported means and their difference separately at the end.

## Slide 4 — Figure

**Macro-F1 on the same July documents**

Figure: Use the complete image `Desktop/R184 talk figure.png`. The chart shows R173 at 0.852 and R184 at 0.873. The horizontal axis shows each run and its feature set. The vertical axis is unitless macro-F1, from 0 to 1. The figure note gives the common evaluation size of 1,200, equal weighting across 12 intents, the July 16 freeze and the +0.021 difference. The figure shows no error bars.

Caption: Both runs use the same July evaluation documents and reference labels.

Speaker notes: “R184 scored 0.873 macro-F1, a difference of +0.021 against R173 on these same 1,200 documents.”

## Slide 5 — Heading and Body

**The average rose and some intents fell**

- The July comparison measures the change from adding character 3–5 grams under the same training and scoring conditions.
- Reschedule F1 fell from 0.888 to 0.829. The evaluation includes 100 documents with that reference label.
- Read the overall difference as an average across the twelve intents on this evaluation.

Speaker notes: Use Reschedule to make the limit concrete: its F1 fell from 0.888 to 0.829, so we can't say all intents improved. If asked about last year's result, say it came from a different evaluation set. Leave operational use out of the talk. If asked whether to use the model for intake routing, say, “That's a separate decision. This comparison measured classification on the July evaluation.”
