---
path: Documents/Change-decisions/Faregate work we can carry on with.md
kind: text
author: Matthew Pickering
derived_from:
---
# Faregate work we can carry on with

Anjali — for the Fare Payments freeze decision, 13 July

The two production items waiting in the queue are the next Faregate rollout tranche and the routine payment reconciliation release. Both are normal production changes; leave them held under the existing freeze unless Operations makes a further decision. The recovery watch passing does not by itself release the queue.

We can continue the failure-count check outside production: compare the eligible-attempt aggregate and the excluded retry and diagnostic indications against the 312 confirmed failures. That work does not require either release to go ahead.

If an urgent production mitigation is needed, bring it to Operations for a separately recorded approval. This list is not that approval.

Matthew
