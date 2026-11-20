---
path: Documents/Lab-meetings/2026/Jul 15 - R184 talk draft.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1 — Cover

**Short intake intent | July comparison**

Document ID: MDR-STI-20260715-LM-01-R01 · Issued: July 15, 2026 · Author: Vanessa Nguyen

Speaker notes: Emily, I'd like to read this aloud once this afternoon and see what assumptions the order invites. The numbers can wait for the scoring check.

## Slide 2 — Heading and Body

**Does the candidate improve on the linear model?**

- R184 adds short character sequences to the word features.
- A spelling variation in a short request can leave very few useful whole words. Character sequences give the classifier another way to match part of a word.
- The July evaluation asks whether adding those features improves the average across the twelve intents.

Speaker notes: I'm trying the improvement question as the opening. On which slide do you first hear what the difference is against? I haven't settled where to show the prediction results.

## Slide 3 — Two Columns

**What goes in, what changes**

Left column — **Received body**

- Use the incoming text as received.
- Keep the subject line, channel, case key and later staff notes out of the features.
- Score one top-ranked intent per body against its reference label.

Right column — **Feature sets**

- R173: word frequencies.
- R184: word frequencies plus character 3–5 grams.
- Both are regularized linear classifiers fitted on the same July training partition. We set the pair on July 8 using development data.

Speaker notes: Make clear that the extra character features come from the same text. The models don't get case context from the split.

## Slide 4 — Table

**The July split and the score**

| Partition | Labelled bodies | Role |
|---|---:|---|
| Training | 3,600 | Fit both runs |
| Development | 1,200 | Set the comparison pair |
| Evaluation | 1,200 | Score both runs against the same references |

Table note: Assign whole cases to partitions. Score one received body per document. Macro-F1 gives each of the 12 intents equal weight.

Speaker notes: Compute F1 separately for each intent as 2TP/(2TP+FP+FN). Give each of the twelve intents the same weight and round the mean at the end. A class with fewer reference bodies still contributes one twelfth. Ask Emily whether the audience can hold the opening question this long before hearing which documents support the answer.
