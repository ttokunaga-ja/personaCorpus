---
path: Documents/Lab-meetings/2025/Short intake January readout.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

# Slide 1 — Cover

Short intake intent classification: January readout

MDR-STI-20250116-LM-01-R01 | Issued: January 16, 2025 | Author: Vanessa Nguyen

Speaker notes: Research meeting with Michael Serrano and the research team. This readout concerns the 2024 short intake collection.

# Slide 2 — Table: What entered the comparison

| Population or partition | Documents | Purpose |
|---|---:|---|
| Labeled candidates | 4,674 | Documents with a supported reference label |
| Training | 2,800 | Fit the classifier |
| Development | 934 | Choose settings |
| Evaluation | 940 | Reserved for scoring |
| Hold | 126 | Outside the scored population; gold label blank |

The partitions were assigned by document. The twelve intents and the July revised reading guide define the reference task.

Speaker notes: The 4,674 and 126 account for the 4,800 received bodies. The hold disposition is not a thirteenth intent.

# Slide 3 — Heading and Body: Word-frequency linear classifier

- Evaluation: 940 documents, 12 reference intents
- Macro-F1: **0.851**
- Input: received message body; word frequencies
- Document-level training, development and evaluation partitions

Speaker notes: This is a research measure on the held-out evaluation documents, not a decision to deploy the classifier. The first-year research target was 0.82; suitability for use requires separate assessment.

# Slide 4 — Heading and Body: How the score is calculated

For each intent, F1 = 2TP / (2TP + FP + FN). Reference support is TP + FN. Macro-F1 is the equal-weight mean of the twelve unrounded class F1 values, rounded for reporting. A class with no predictions receives F1 zero when it has positive reference support.

Speaker notes: Training fits the model, development supports setting choices, and evaluation is reserved for scoring. Each scored unit is one received body.

# Slide 5 — Heading and Body: Interpretation and next discussion

- The result describes the labeled evaluation population, not the 126 held bodies.
- Read errors by intent and supporting phrase before drawing conclusions about where the model fails.
- A separate use assessment must consider workflow and costs.

Speaker notes: Ask the team which error distinctions should be read together next. Keep the denominator and partition method with any citation of this result.
