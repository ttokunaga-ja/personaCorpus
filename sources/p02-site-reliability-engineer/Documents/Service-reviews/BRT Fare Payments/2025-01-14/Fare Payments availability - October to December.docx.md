---
path: Documents/Service-reviews/BRT Fare Payments/2025-01-14/Fare Payments availability - October to December.docx
kind: brt-reliability-meeting.docx
author: Daniel Jennings
derived_from:
---

# Fare Payments availability — October to December

Reference  BRT-MT-20250114-001 | Issued  14 Jan 2025

Prepared by  Daniel Jennings

## Agenda

Quarterly availability readout for Anjali Shah, Operations Director. Review held 14 January 2025 at 11:00 UTC with Daniel Jennings and Matthew Pickering. We compared the elapsed-time availability figures and agreed the order of follow-up work. The service target is 99.9% monthly; the quarter below is a weighted elapsed-time summary, not a separate monthly target.

## Reconciliation

The bounds below are UTC, inclusive at the start and exclusive at the end. Availability is one minus counted unavailable seconds divided by eligible seconds. Monthly percentages are rounded to five decimal places; the quarter is calculated from summed seconds, not an unweighted average of the monthly rates.

| Period (UTC) | Eligible seconds | Unavailable seconds | Availability |
|---|---:|---:|---:|
| 1 Oct 2024 00:00 – 1 Nov 2024 00:00 | 2,678,400 | 552 | 99.97939% |
| 1 Nov 2024 00:00 – 1 Dec 2024 00:00 | 2,592,000 | 2,520 | 99.90278% |
| 1 Dec 2024 00:00 – 1 Jan 2025 00:00 | 2,678,400 | 1,728 | 99.93548% |
| Q4 total | 7,948,800 | 4,800 | 99.94% |

Matthew confirmed the two Faregate windows as distinct customer-visible intervals counted once each. Their share uses downtime, not all eligible time, as its denominator.

| Counted interruption | Seconds | Elapsed | Share of Q4 downtime |
|---|---:|---:|---:|
| November Faregate | 2,520 | 42:00 | 52.5% |
| December Faregate | 1,368 | 22:48 | 28.5% |
| Faregate combined | 3,888 | 64:48 | 81.0% |
| Other interruptions | 912 | 15:12 | 19.0% |
| Total | 4,800 | 80:00 | 100.0% |

## Decision and actions

Anjali asked us to address the two longer Faregate windows first. The checks are intended to improve readiness and observation; they do not assert a cause for either interruption.

| Priority | Owner | Action and evidence to bring back | Due |
|---|---|---|---|
| 1 | Matthew Pickering | Rehearse a safe return to the previous checkout route; bring elapsed time and observed outcome. | 24 Jan 2025 |
| 2 | Daniel Jennings | Check that the post-change watch catches a failed checkout completion; bring the watched outcome and reviewer acknowledgement. | 31 Jan 2025 |
| Later | Daniel Jennings and Matthew Pickering | Scope broader work on the other interruptions after reading those two results. | After the two results |

Both actions are open at issue. The measured Q4 figures remain the same as the action status changes.
