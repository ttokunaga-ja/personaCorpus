---
path: Documents/Review-notes/Jul 14 one answer for every scored body.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# One answer for every scored body

July 14, 2026 · For Fatima

I almost carried the two cutoffs from the June routing sheet into Thursday's comparison. In June the question was where to send a saved prediction. At an unrounded confidence of exactly 0.450, it went to hold under 0.60 and to automatic routing under 0.45. Equality went to automatic routing, and the predicted label stayed the same.

For M-14, if either run gives its top intent a confidence of 0.450, score that intent against the reference for the body. Apply the same rule to every other confidence. Both runs owe us one label for each of the same 1,200 reference-labelled bodies. There is no cutoff or routing decision in this comparison, so the June equality rule never enters the join.

The 126 old Hold documents were left without reference labels under the July 2024 reading rules. Leave them out of both the July denominator and any count of low-confidence predictions. If a July reference ID has no matching prediction, flag the missing row in the handoff check and resolve it before averaging. Dropping the row would change the set we agreed to score.
