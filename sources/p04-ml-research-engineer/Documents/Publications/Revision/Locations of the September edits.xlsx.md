---
path: Documents/Publications/Revision/Locations of the September edits.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Sheet: Adjudication

A2: Locations of the September edits  
B3: MDR-STI-20260917-AW-01-R01  
E3: September 17, 2026  
B4: Vanessa Nguyen  
E4: Nicole N1–N9; manuscript R02 to R03

| Row | A: Ref. | B: R02 location | C: R03 heading | D: Disposition | E: Check marker | F: Note |
|---:|---|---|---|---|---|---|
| 6 | N1 | Methods / Partitions and reference | Case Allocation | Reflected | 2,804 / 935 / 935; three pairwise shared-case counts | Case is allocation unit |
| 7 | N2 | Methods / Partitions and reference | Scoring Set | Reflected | One body; Hold 126 excluded; same 935 and 12 labels | Scoring unit and set |
| 8 | N3 | Methods / Comparison and scoring | Comparator | Reflected | New work request for every evaluation document | Training majority class |
| 9 | N4 | Methods / Comparison and scoring | Macro-F1 | Reflected | Average 12 unrounded class F1 values, then round | Equal class weights |
| 10 | N5 | Results / comparison paragraph | Results / Figure 1 | Reflected | 0–1 axis; 0.021 and 0.832; same 935 | Direct labels and denominator |
| 11 | N6 | Abstract; Methods / Reading selected errors | Abstract; Diagnostic Reading | Reflected | 160 selected from saved development predictions | Diagnostic reading |
| 12 | N7 | Discussion | Scope Examples | Reflected | 41 = 18 + 14 + 9; remaining 119 | Observation, not prediction cause |
| 13 | N8 | Results / comparison paragraph | Results / Figure 1 | Out of scope | April comparison retained | July has a different evaluation set and comparators |
| 14 | N9 | Methods / Partitions and reference | Case Allocation | Out of scope | Three shared-case counts are zero | Cross-channel key correspondence is internal |

A16: Reflected; D16: `=COUNTIF(D6:D14,"Reflected")`  
A17: Out of scope; D17: `=COUNTIF(D6:D14,"Out of scope")`  
A18: Total; D18: `=COUNTA(A6:A14)`

A20:F20: Comment text: HEI-20260909-MR-01-R01. Full author explanations: MDR-STI-20260917-RM-01-R01. Check the headings in manuscript MDR-STI-20260420-RM-01-R03.
