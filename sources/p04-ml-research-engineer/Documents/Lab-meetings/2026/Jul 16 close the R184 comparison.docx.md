---
path: Documents/Lab-meetings/2026/Jul 16 close the R184 comparison.docx
kind: mercer-lab-record.docx
author: Vanessa Nguyen
derived_from:
---

# Close the R184 comparison

Document ID: MDR-STI-20260716-LR-01-R01  
Issued: July 16, 2026  
Author: Vanessa Nguyen

Meeting: July 16, 2026, 1:30 p.m. PDT  
Present: Vanessa Nguyen and Fatima Rahman  
Circulation: Fatima Rahman and the research team

## What we checked

We noted Joshua Ramirez's July 7 report on the cross-channel case keys: he found zero shared cases in each of the training–development, training–evaluation and development–evaluation pairs. The case-level allocation detail was unavailable to us. We checked the document totals from the channel counts ourselves: 3,600 training, 1,200 development and 1,200 evaluation bodies. Those totals establish the sizes of the partitions.

We confirmed that R173 and R184 use the same 1,200 evaluation bodies and the same twelve-intent reference. Each run contributes one top intent per body. The per-intent counts reconcile with Fatima's reference support, and the total false positives equal the total false negatives in each run. We recalculated F1 for every intent from the counts, gave each intent equal weight and checked the difference before rounding the means for display.

## Decision at 2:20 p.m. PDT

Freeze the July comparison with R173 as the word-frequency reference run and R184 as the run that adds character 3–5 grams. On this evaluation R173 is 0.852 and R184 is 0.873. Record the paired difference as +0.021 macro-F1. Keep the scoring set and settings fixed. Threshold selection and any decision on using the model for intake routing were outside this meeting.

## Follow-through

| Owner | Action | Due |
|---|---|---|
| Fatima Rahman | Retain the shared support and both runs' per-intent counts. Confirm the unrounded means and the difference for the brief. | Before the July 16 brief is issued |
| Vanessa Nguyen | Issue the condition worksheet and fixed-results brief with the evidence for partition sizes and the provenance of the case check. | July 16, before requesting talk permission |
| Vanessa Nguyen | Send Michael Serrano the proposed slide order and figure together with the internal result records for his decision on the oral talk. | July 16, after issuing the brief |
