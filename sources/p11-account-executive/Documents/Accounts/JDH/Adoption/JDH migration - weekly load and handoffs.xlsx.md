---
path: Documents/Accounts/JDH/Adoption/JDH migration - weekly load and handoffs.xlsx
kind: halstead-adoption.xlsx
author: Divya Rao
derived_from:
---

# Implementation Load

A3: JDH migration — weekly load and handoffs  
C4: HBC-JDH-20260115-IMP-01 v1.0  
C5: January 15, 2026  
C6: Divya Rao

| Row | A: Week | B: Workstream | C: Demand (h) | D: Available (h) | E: Headroom (h) | F: Dependency or handoff |
|---|---:|---|---:|---:|---|---|
| 9 | 1 | Roster and contacts | 24 | 28 | =D9-C9 | Start only after JDH's initial roster and site contacts are confirmed; Janet coordinates the inputs. |
| 10 | 2 | Recipient reconciliation | 28 | 28 | =D10-C10 | Compare department destinations against the roster; JDH resolves conflicting entries. |
| 11 | 3 | Cutover procedure | 24 | 26 | =D11-C11 | Use reconciled destinations to walk the handoff and prepare the single-site rehearsal. |
| 12 | 4 | Single-site rehearsal | 28 | 28 | =D12-C12 | Day and night shifts practice separately; capture outcomes and issue list. |
| 13 | 5 | Resolve issues and approve | 28 | 30 | =D13-C13 | Rehearsal results and issue list pass to owners; corrections and evidence must be reviewed and the responsible JDH contact approve before live cutover. |
| 14 | 6 | First group cutover | 26 | 28 | =D14-C14 | Proceed only after W5 corrections and approval. |
| 15 | 7 | Second group cutover | 24 | 26 | =D15-C15 | Use first-group handoff and confirm second-group readiness. |
| 16 | 8 | Stabilization | 22 | 24 | =D16-C16 | Review the two live groups' outstanding operational issues. |
| 17 | 9 | Support handoff | 16 | 20 | =D17-C17 | Transfer status, open issues and owners to ongoing support. |
| 18 | | Total | =SUM(C9:C17) | =SUM(D9:D17) | =D18-C18 | Nine-week estimate, conditional on starting inputs. |
| 19 | | W1–W4 | =SUM(C9:C12) | =SUM(D9:D12) | =D19-C19 | Rehearsal checkpoint: 104 demand / 110 available hours. |
| 20 | | W5–W9 | =SUM(C13:C17) | =SUM(D13:D17) | =D20-C20 | 116 demand hours remain after W4; 128 hours available over these weeks. |

At the W4/W5 gate, Divya reviews the day and night rehearsal results and the issue list with the responsible JDH contact. JDH resolves recipient or roster issues; Divya records evidence of the corrections. The JDH contact must approve proceeding before the first live group. A rehearsal alone is not a release approval. W2 and W4 have no spare hours; moving later work into them also skips dependencies. The estimate is not a booked schedule.
