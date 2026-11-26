---
path: Documents/Recovery-evidence/Archive load trial entry criteria.md
kind: text
author: Sanjay Mehta
derived_from:
---

# Entry criteria for the ticket archive load trial

17 September 2024 · Proposal for Daniel to review

Daniel,

I propose taking both schedules into the isolated archive trial. BRT-IC-20240916-001 gives us the comparison to carry forward: 0 open predecessors in 24 starts at 15 minutes and 2 in 36 at 10 minutes. Both flags were in the busier hour. I'd like Patrick to agree how we hold the foreground load steady across the two runs before we book either window.

## Before booking

- Keep 15 minutes as the comparator and 10 minutes as the candidate. Use the same isolated starting conditions and confirm an empty archive queue at the opening of each window.
- Agree one sustained foreground request profile with Patrick, including the request mix, rate and duration. Apply that same profile in both windows and record the profile actually delivered alongside the starts. The dates and load size are open for agreement with Patrick.
- Agree the fixed UTC reference for each readback in advance, at the same point in the load profile for both runs. Keep the coverage and completion marks required by the 20 August review (BRT-MT-20240820-001).

## Evidence to collect

Count actual starts in each agreed window, including the opening boundary and excluding the closing boundary. Record the current start and the immediate predecessor's reference to the UTC second. Follow every open predecessor through to its matching finish, including past the end of the window if required. Keep the overlap against the original start. If the finish cannot be located, mark the follow-up unresolved and leave its duration blank.

For the readback, retain the selected set's reference, evidence that it completed by the fixed reference, and the latest restorable coverage confirmed from its records. Subtract that coverage time from the reference. The existing target is 15 minutes inclusive. Exactly 15 minutes meets it. If eligible coverage remains unconfirmed, leave the target assessment open. Record restore duration separately.

Please check the evidence fields before I arrange the windows with Patrick. This is the proposed method for an isolated trial. The operator recovery terms still apply.

Sanjay
