---
path: Documents/Change-decisions/August reconciliation shadow decision.docx
kind: brt-change-decision.docx
author: Anjali Shah
derived_from:
---
# August reconciliation shadow decision

Reference  BRT-CD-20260818-001 | Issued  18 Aug 2026

Prepared by  Anjali Shah

For Matthew Pickering and Daniel Jennings

## Decision

Following our 11:00 UTC review on 18 August, I approve one production shadow-read window on 19 August from 09:00 to 09:30 UTC, for up to 10% of eligible reconciliation jobs. The comparison must not write or post payments. The existing production path remains in place. Matthew owns the decision to start and to stop the window.

## Basis and restrictions

Louise Turner's capacity check PCS-DF-20260818-001 reports 28% remaining against a verified 2,800 eligible attempts/min ceiling in the busiest observed band on 17 August: 2,016/min at p95, leaving 784/min. The ceiling was verified with the normal pool and representative mix. This evidence supports a small, controlled read-only exception; it does not establish failover capacity or production change safety. The 25% job share proposed for discussion is not approved.

Before starting, verify the normal worker pool and matching workload mix, verify that comparison results cannot write or post, and establish headroom of at least 25%. Daniel will monitor the p95 of one-minute eligible attempt rates over the trailing 15 minutes against the verified 2,800/min ceiling before and during the window. Matthew must stop immediately for headroom below 25%, any output discrepancy or any customer-facing regression.

The normal reconciliation release and the next Faregate rollout tranche remain held under BRT-CD-20260713-001. This exception does not lift either hold.

## Review conditions

Bring the observations, or the reason for standing down, to the 19 August 11:00 UTC Operations review. Any expansion requires my further decision at or after that review; neither the window nor the review itself authorises expansion.

Anjali Shah  
Operations Director
