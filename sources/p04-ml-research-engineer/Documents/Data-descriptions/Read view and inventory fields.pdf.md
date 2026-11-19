---
path: Documents/Data-descriptions/Read view and inventory fields.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---
# Read view and inventory fields

Document ID: MDR-STI-20240507-DD-01-R01  
Issued: May 7, 2024  
Author: Vanessa Nguyen  
For: Joshua Ramirez and Fatima Rahman

## Purpose

For the May reading extract, give each reader the same received body and a reconciliation ID. Keep the information used to route and handle a message in the inventory so that Joshua can reconcile records without making those fields available to the readers. The route describes a handling path; it is not a substitute for the sender's words.

## Field handoff

| Field | Reader extract | Inventory or handling record |
|---|---|---|
| Reconciliation ID | Include | Retain for matching |
| Received body | Include, with its start and end checked | Retain source for comparison |
| Subject | Do not include | Retain |
| Queue or destination name | Do not include | Retain |
| Routing tag | Do not include | Retain |
| Handling outcome | Do not include | Retain |
| Forwarding staff comment | Do not include | Retain separately from received body |

## Boundary check

Suppose a forwarded record contains the staff note “Please take this one,” followed by the received body “Could you send the invoice again? Thank you.” The extract for the reader consists of the ID and “Could you send the invoice again? Thank you.” The staff note is outside the received body, even if it appears above the forwarded text. The subject, queue and any outcome remain in the inventory. Do not infer the body boundary merely from the first visible sentence of the forwarded record.

Joshua should check the start and end against the received record before releasing the extract. If the boundary cannot be established, flag it for investigation rather than adding the forwarding comment to the reading view. Fatima can then compare the readers' candidate actions and their supporting text using the ID without exposing the withheld fields.
