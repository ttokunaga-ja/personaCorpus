---
path: Pictures/Analysis/2026-08-10 A known reason can go stale.png
kind: diagram
author: Jeremy Sutton
derived_from:
---
# A known reason can go stale

Diagram for a prototype discussion with Jennifer. Title: “Can the reason be carried forward?” Subtitle: “Information states to consider — not an agreed update rule or screen behavior”.

Three starting boxes on the left, each with an arrow to the central question:

1. “Reason confirmed for this route” — “A reason is tied to the route under discussion.”
2. “Information tied to an earlier day” — “Same route number alone does not establish today's reason.”
3. “Reason not confirmed” — “A contact may be known even when the reason is not.”

Central decision box: “Is the reason confirmed for this route and relevant time, with Source and Checked at available?”

- Yes arrow to “Candidate for display: Hold reason + Source + Checked at”.
- No arrow to “Unknown — do not present the old or unconfirmed reason as current”. From this box an arrow goes to “If a contact is known, show whom to ask in route detail”.

A note beneath both outcomes: “A source or contact is not release authority.”

Margin comparison:

- “UX-181: a message from the previous day was ruled out; today's reason remained unknown.”
- “UX-182: an older record already contained a reason; the receiving desk confirmed the current situation against route and window. This was not permission to receive.”

The arrows organize questions for a design discussion. They do not specify how the system acquires, updates, or expires data.
