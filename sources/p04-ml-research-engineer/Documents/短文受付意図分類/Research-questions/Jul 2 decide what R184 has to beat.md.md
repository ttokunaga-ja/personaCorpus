---
path: Documents/短文受付意図分類/Research-questions/Jul 2 decide what R184 has to beat.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# Jul 2 — Decide what R184 has to beat

July 2, 2026 · M-14 · For Fatima and Emily

Can a few shared letters carry the request when a short misspelling leaves the word model with very little to go on? I’d like to try adding character 3–5 grams to the word-frequency features for R184. Start with the 1,200 July development bodies. A development miss like “can you rescedule?” is where I’d look first: check whether the added features recover the requested action, then check what happens to the equally short messages that are already right.

Emily, please fit the word-only R173 and the augmented R184 on the same July training partition. Keep the regularized linear model and the body-only input policy common to both. On development, put the two macro-F1 values alongside one another and save the IDs where the predictions differ. Fatima, can you check those IDs against the same development references so we can tell a recovered spelling variant from a change that just happens to land on the right label?

For the July head-to-head I want the word-only R173 scored on the very same evaluation partition as R184, against the same reference labels. The April case-split report describes the procedure I want to reuse. Its score comes from a separate set. Let’s agree on the comparator and the added feature window before looking at any July evaluation output. Until we have closed those choices, keep the discussion to the development results.
