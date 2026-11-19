---
path: Documents/長文点検記録分類/Cost-discussions/Oct 14 keep the comparison open.docx
kind: mercer-lab-record.docx
author: Vanessa Nguyen
derived_from:
---

# October 14: Keep the comparison open

Document ID: MDR-LDI-20251014-LR-01-R01  
Issued: October 14, 2025  
Author: Vanessa Nguyen

## Discussion

October 14, 2025, 2:00 p.m. PDT. Participants: Vanessa Nguyen and Brandon Romero.

The same fixed 480-record evaluation produced 37 misses with the first 2,048 body tokens and 11 with the first 1,536 plus last 1,024. A miss means an action-required reference was predicted as one of the four remaining classes. The paired counts include 28 recovered and two newly missed records; nine were missed by both. One of the new misses loses an instruction in the middle; another loses a clause governing the reinspection deadline.

Brandon's comparison covers one pass over the same 480 records at the same equipment, parallelism and unit prices, including preparation and host processing. With the first-only pass indexed at 100, the ends pass is 118: 18% higher. The measured conditions differ in both body-token budget and location, so neither the improvement nor the cost increment isolates the effect of location.

## Decisions and follow-up

- Vanessa will compare first 2,048 against first 1,536 plus last 512, and first 2,560 against first 1,536 plus last 1,024, using the same evaluation and references. She will inspect the two middle-evidence cases individually with Fatima.
- Brandon will measure inference cost for the same scope and measurement boundary, using the same unit prices, for the comparisons.
- Keep application adoption on hold. Review the matched-budget results and both directions of movement before deciding whether an ends window is preferable.
