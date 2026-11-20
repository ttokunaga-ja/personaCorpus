---
path: Documents/Clients/Pioneer/Buying/02 Sep - First dispatch evidence guide.docx
kind: holt-handoffs.docx
author: Connor Atkinson
derived_from:
---

# Trace the order to its first send

Evidence note: HRA-PIO-BUY-EV-2026-001 v1.0  
Date: 02 Sep 2026  
Prepared by: Connor Atkinson  
For: Neha Patel and Nicholas Hargreaves

## What to compare

Use the first event that sent the PO to the supplier, not the creation or last-change field. Compare it in EDT with the last deadline approved before that send. A send later than the deadline is late; a send at the deadline is on time. Check the approval history as well as the displayed deadline: a later amendment must not be applied retrospectively.

## Where to look

| No. | Case | Evidence and reading |
|---|---|---|
| 1 | PO-84217 | Buying dispatch history EV-84217 records first supplier send on 18 Aug at 15:08 EDT. Deadline approval history for PO-84217, entry DH-84217-01, records approval on 18 Aug at 14:30 EDT of the 18 Aug 15:00 deadline; the history shows no later approval before 15:08. Eight minutes late. The export trial's Created value of 14:42 is only a draft time and would falsely make this on time. |
| 2 | PO-84241 | Buying dispatch history EV-84241 records first send on 19 Aug at 15:00 EDT. Deadline history DH-84241-01 records approval at 14:20 EDT that day of the 15:00 deadline, with no subsequent approval before send. Equality is on time, not late. |
| 3 | Same date, no time | A date alone cannot establish whether a same-day send was before or after 15:00. Leave the outcome pending and ask Buying for the first supplier-send event and deadline approval history; do not fill in a time from Created or Last changed. |

## Follow-through

Neha: use these two cases when explaining the event and deadline to Buying. Nicholas: keep the date-only cases outside the judged denominator until Buying supplies enough precision. The 28 Aug export trial is a field-selection check; it is not a rate calculation.
