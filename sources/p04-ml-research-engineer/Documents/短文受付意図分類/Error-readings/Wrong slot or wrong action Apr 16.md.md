---
path: Documents/短文受付意図分類/Error-readings/Wrong slot or wrong action Apr 16.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# Wrong slot or wrong action — April 16

Fatima and I read these after scoring the fixed 935-document evaluation. They illustrate distinctions in the July guide; they are not a new sample from which to estimate a separate error rate.

## Asking about a time is not moving it

Received body: “What time is the visit on Thursday?” Reference: Schedule check. The linear classifier predicted Reschedule. “What time” asks for the appointment time; there is no request to move the visit. The shared “visit” and day are not themselves evidence of a change.

## Asking for the invoice again is not a question about its amount

Received body: “Could you send the invoice again?” Reference: Request document. The classifier predicted Invoice question. “Send the invoice again” identifies the requested action as sending a document; it asks nothing about a charge or amount.

These are explanations of fixed-set errors. We will not revise the current score, features or model settings in response to this reading. If we try a change later, it needs its own comparison conditions.
