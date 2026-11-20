---
path: Documents/Data-descriptions/Case key provenance guide.pdf
kind: tex:mercer-research-brief.cls
author: Joshua Ramirez
derived_from:
---

# Where the case key came from

Document ID: MDR-STI-20250220-DD-01-R01  
Issued: February 20, 2025  
Author: Joshua Ramirez  
For: Vanessa Nguyen, Fatima Rahman and researchers using the intake ledger

## Purpose and scope

A case key brings received documents about the same case together before partitioning; it is not an intent label. A document is one received body, and several bodies may belong to one case. The ledger covers Web form, Shared inbox and Service desk relay. Preserve each document ID and its receipt reference even when a cross-channel key joins it to another entry.

## Evidence for a link

| Situation | Evidence to retain | Disposition |
|---|---|---|
| Direct receipt link | The original receipt record explicitly links the document ID to the same case reference as another receipt. Record both receipt references and the shared case reference. | Assign the same case key. |
| Confirmed alias | A receipt uses an alternate case reference, and the confirmed alias table records its correspondence to the other reference. Record the receipt and the specific confirmed alias entry. | Assign the shared case key, retaining the original reference as well. |
| Similar subject only | Two messages have similar subjects but neither receipt record nor a confirmed alias establishes a shared case. | Do not join on subject similarity. Hold the proposed link for evidence review. |

For example, a web receipt and an inbox receipt that both explicitly point to the same case reference can be grouped directly. If the inbox record instead carries an alternate reference, the corresponding confirmed alias entry must connect the two references. Similar subjects such as “visit next week” are not enough: different cases can use the same words. The channel of receipt alone does not establish identity across channels.

## Recording uncertainty

Put the document IDs, channel, receipt reference, any confirmed alias entry, proposed key and disposition in the reconciliation ledger. Put an unsupported link in the ledger's pending disposition with the missing evidence identified; keep it out of a confirmed bundle until the receipt or alias evidence is checked. Do not guess from a subject or assign a link merely to fill a key. Bring unresolved links to me for investigation.

## Use at allocation

The February check of 24 known crossing cases reunited 17 with channel-specific keys and all 24 with evidence-backed cross-channel keys. That check is about case matching, not a classifier score. The key and channel are allocation and audit information: neither goes into classifier input. The received body alone is the model text; retain the ledger separately so the assignment can be traced.
