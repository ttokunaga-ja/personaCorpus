---
path: Desktop/Aldridge walkthrough - product first_travel.pptx
kind: halstead-workshop.pptx
author: Michelle Whitaker
derived_from:
---

# Slide 1 — Cover

Title: Aldridge walkthrough — service first

Subtitle: HBC-ALD-20241101-BRF-01 v0.2 | November 1, 2024 | Michelle Whitaker | Aldridge Service Group

Speaker notes: Working order for Ahmed's November 5, 10:00 Denver call. Thirty minutes: 15 on the service, 10 on the two operating boundaries, 5 on the remaining exceptions and questions. Ahmed asked to start with #24 and #34; check whether this opening leaves enough time for them.

# Slide 2 — Two Columns

Title: Halstead Branch Connect: from a branch message to a record

Left — Branch communication:
- Administrators maintain the people and branch information used for communications.
- A branch sends an after-hours handoff to the next duty person.
- The delivery log records the sending of the message.

Right — Administration and rollout:
- An administrator can export activity records as CSV.
- Aldridge manages the source roster; a CSV can be imported manually.
- Divya's proposed rollout checks one site before proceeding to the next.

Speaker notes: Allow 15 minutes for this service introduction and the steps on this slide. Distinguish sending from an individual acknowledgment and export from retention in the service. These steps help explain the response, but they do not remove the six mandatory exceptions.

# Slide 3 — Figure

Title: #24 — where the after-hours handoff leaves the service

Diagram, left to right:
1. Branch sends handoff → delivery log records that the message was sent.
2. Separate box: next duty person receives the handoff and calls back. Label: possible Aldridge operating step; receipt and callback are not established by the delivery log.
3. Separate box: branch lead records or follows up on the callback. Label: Aldridge would need to decide how to manage this step.

Between the delivery log and the callback box, show a break labeled: no individual acknowledgment recorded by the service. Do not draw an arrow from the log to receipt.

Caption: A sending record is not an individual acknowledgment.

Speaker notes: I had planned to show the handoff on screen. Replace that with this diagram: the only screen evidence I can show is the delivery log, and that confirms sending only. Ask Ahmed how Aldridge might handle a callback; do not present that operating step as a product feature or as satisfying #24.

# Slide 4 — Table

Title: Exceptions to walk through

| Requirement | Current boundary | Question for the conversation |
|---|---|---|
| #34 — seven-year in-service retention | Administrator exports CSV; Aldridge would manage storage, access and retention outside the service. | Who could take custody of the export? |
| #5 — two-way directory sync | Manual CSV import; no automatic two-way sync. | How would the source roster be maintained? |
| #14 — site-level delegated administration | Central administrator handles site requests; no site-limited delegation. | How would sites route requests? |
| #43 — single-night cutover | One site first, then sequential rollout; no all-site single-night cutover offered. | What constraints would sequencing create? |
| #51 — 30-minute overnight initial response | Overnight requests receive contact the next business day. | What overnight coverage does Aldridge need? |

Caption: #24 and these five remain mandatory exceptions in the submitted response.

Speaker notes: Spend 10 minutes across #24 and #34, then 5 minutes on the other four and questions. Ahmed asked for #24 and #34 first, so I should reconsider this product-first sequence before sending. The October 23 response addresses 50 requirements and lists these six as exceptions; Aldridge's scoring remains its decision. No promise of full compliance or a delivery date.
