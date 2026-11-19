---
path: Documents/Incidents/2026/2026-07-13/Faregate evidence readback.docx
kind: brt-reliability-meeting.docx
author: Daniel Jennings
derived_from:
---
# Faregate evidence readback

Reference  BRT-MT-20260713-001 | Issued  13 Jul 2026

Prepared by  Daniel Jennings

## Agenda

INC-260713 — evidence check-in, 13 July 2026 at 10:20 UTC. Present: Daniel Jennings, Matthew Pickering and Claire Hutchinson. Reconcile the incident milestones and impact before calculating the July balance and preparing customer wording.

## Evidence reviewed

| Milestone | UTC | Evidence and interpretation |
|---|---|---|
| First confirmed affected attempt | 08:07 | Dashboard observation read 09:07 BST. Matthew confirmed the clock's time zone; the equivalent UTC time replaces the provisional start. |
| On-call page | 08:12 | Alert receipt, previously used as the provisional start. It is detection, not the first confirmed affected attempt. |
| Recovery observed | 08:41 | Payment path recovery signal; the subsequent watch was still needed. |
| Verification | 08:56 | Three completed five-minute windows from 08:41 contained 740, 820 and 900 eligible attempts, with no repeat incident failures. |

Matthew's disposition reconciliation separates 312 unique eligible failed attempts from 48 additional retry indications and 24 diagnostic indications in the initial 384. The confirmed 312 is out of 4,800 eligible attempts in [08:07, 08:41) UTC. The earlier 08:18 update said about 380 indications and remains an estimate of signals, not a customer count. Neither the number of people nor a Westbury-only count follows from the aggregate.

Matthew paused the staged rollout and returned traffic to the last stable route at 08:29 UTC. Recovery followed, but the evidence reviewed does not establish that action as the cause of recovery or identify the cause of the failure.

## Decisions and owners

| Owner | Decision / next action |
|---|---|
| Matthew Pickering | Own the 312-of-4,800 impact reconciliation and its scope. |
| Daniel Jennings | Calculate the July Fare Payments error budget against the monthly eligible population and provide the cutoff to Anjali. |
| Claire Hutchinson | Prepare Westbury wording after Operations decides the freeze; distinguish failed attempts from people and address uncertain receipts. |

Daniel Jennings
