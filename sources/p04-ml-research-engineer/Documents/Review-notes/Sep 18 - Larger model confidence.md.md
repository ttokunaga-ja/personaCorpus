---
path: Documents/Review-notes/Sep 18 - Larger model confidence.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# September 18 — higher scores may only sound surer

My first thought was that the larger model might send fewer bodies to review at the same 0.60 cutoff. I cannot make that claim from a larger model's scores: 0.60 from one model need not represent the same degree of confidence as 0.60 from another. A higher top score alone says nothing about whether that score is calibrated against correctness.

For the September comparison, score the top label for each of the 934 development bodies. No confidence routing is applied. The 800 August bodies are included in those 934, not a second independent sample. The August queue showed 144 waiting, including 122 correct predictions; it does not tell us what the larger model's queue would contain. Leave review burden unquantified here. If we later want to compare queues, first agree on comparable confidence conditions and count both caught errors and correct cases sent for review.
