---
path: Documents/短文受付意図分類/Measurements/R184 evaluation conditions.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# R184 evaluation conditions

## Sheet: Adjudication

A2: R184 evaluation conditions  
B3: MDR-STI-20260716-AW-01-R01  
E3: July 16, 2026  
B4: Vanessa Nguyen  
E4: M-14 July comparison for Fatima Rahman and the research team

| Row | A Ref. | B Comparison condition | C Evidence source | D Status | E Draft location | F What to verify |
|---:|---:|---|---|---|---|---|
| 6 | 1 | Partition by cross-channel case key | Joshua Ramirez's July 7 report | Reported by Joshua. Allocation detail unavailable to the research team for an independent recount. | Frozen results — Partition and reference | Joshua reported zero shared cases in each pair: training–development, training–evaluation and development–evaluation. Count shared cases by case key. Count partition sizes by received body. |
| 7 | 2 | Document totals by partition | July intake coverage by channel.csv | Reconciled on July 16 | Frozen results — Partition and reference | Sum Web form, Shared inbox and Service desk relay within each partition: 1,642 + 1,125 + 833 = 3,600 training, 534 + 382 + 284 = 1,200 development, and 515 + 401 + 284 = 1,200 evaluation. Channel totals establish document counts only. |
| 8 | 3 | One common labelled evaluation set | July reference support check.csv and July 16 scoring reconciliation | Same references confirmed for both runs | Frozen results — Partition and reference | Both runs use the same 1,200 reference-labelled bodies. All 12 intents in the July 2024 guide (MDR-STI-20240513-GD-01-R02) have support. Each run's TP + FN matches that support per intent. The 126 unlabelled legacy Hold bodies are outside the set. |
| 9 | 4 | Model input and comparison pair | July development runs.csv and July 16 comparison | Run conditions confirmed | Frozen results — Models and scoring | Both regularized linear classifiers use the same July training set and only received text. R173 uses word frequencies. R184 adds character 3–5 grams. Exclude case key, channel, subject and later staff notes. |
| 10 | 5 | Choice closed on development | July development runs.csv and Leave the shorter character window out.pdf | Pair set July 8 before evaluation | Frozen results — Models and scoring | On the same 1,200 development bodies, R173 scored 0.848, R184 0.871 and R187 0.870. Keep R173 as comparator and R184 as candidate. R187 ended in development. |
| 11 | 6 | Top-one scoring and macro-F1 | R184 results by intent.csv and R184 run comparison.tsv | Recalculated on July 16 | Frozen results — Models and scoring | Score one top intent per body at every confidence. For each run calculate 2TP/(2TP+FP+FN) for each of the 12 classes and average the unrounded values equally. Subtract the unrounded R173 mean from the unrounded R184 mean. Round each reporting value, including the difference, to three decimal places at the end. |

A12:F12: Scope: fixed July research comparison. Misrouting recovery time was not measured. Any decision on operational use requires separate evidence.
