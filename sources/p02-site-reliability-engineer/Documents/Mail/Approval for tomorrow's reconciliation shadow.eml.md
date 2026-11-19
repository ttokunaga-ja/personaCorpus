---
path: Documents/Mail/Approval for tomorrow's reconciliation shadow.eml
kind: text
author: Anjali Shah
derived_from:
---
From: Anjali Shah <anjali@brt-booking.example>
To: Matthew Pickering <matthew@brt-booking.example>, Daniel Jennings <daniel@brt-booking.example>
Cc: Claire Hutchinson <claire@brt-booking.example>
Date: Tue, 18 Aug 2026 13:32:00 +0100
Subject: Approval for 19 August reconciliation shadow window
Message-ID: <a4f7b0e1-8d3c-4f58-92a1-6e048b17d25c@brt-booking.example>

Matthew and Daniel,

I've attached the issued decision BRT-CD-20260818-001 for tomorrow's reconciliation shadow window, 09:00–09:30 UTC. This permits read-only comparison for up to 10% of eligible jobs; it does not authorise writes or postings.

Matthew, please confirm the preflight before starting: the normal worker pool and matching workload mix, the no-write/no-post check and at least 25% headroom. You own the start and stop decision. Stop immediately if headroom falls below 25%, if there is any output discrepancy, or if there is a customer-facing regression. Daniel will watch the trailing 15-minute p95 against the verified 2,800/min ceiling.

Claire, the existing customer path will stay unchanged. The normal reconciliation release and the next Faregate rollout tranche remain on hold.

Daniel, please bring the observations, or the reason for standing down, to the 11:00 UTC Operations review on 19 August. Any expansion needs my further decision at or after that review.

Thanks,
Anjali

Attachment: August reconciliation shadow approval.pdf (application/pdf; Documents/Shared-drive/August reconciliation shadow approval.pdf)
