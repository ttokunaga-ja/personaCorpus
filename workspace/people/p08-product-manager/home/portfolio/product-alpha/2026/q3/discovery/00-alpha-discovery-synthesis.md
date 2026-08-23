# Product Alpha Q3 discovery synthesis

**Juniper Works | 2026-07-13 | milestone M-27**  
**Decision context:** guided rollout gate on 2026-09-18; 1,240 synthetic workspaces; adoption is 18.4% against a Q3 target of 26.0%.

## What we learned

Across 12 synthetic discovery sessions and a prototype walkthrough, participants could describe the intended outcome but often could not reach their first useful result without help. Evidence clusters around four themes:

| Theme | Evidence | Implication |
|---|---|---|
| First-value arrival | 8/12 paused before completing the first guided task; 6 asked what a “good first result” looked like. | The opening should name one job and show a visible success state. |
| Permission understanding | 7/12 interpreted access choices as permanent or risky; 5 deferred an action to an administrator. | Explain scope, reversibility, and who can see the outcome before asking for access. |
| Notification fatigue | 9/12 wanted fewer default alerts; 4 muted an illustrative alert stream immediately. | Start with one digestible signal and make the control obvious. |
| Export confidence | 6/12 hesitated to share an output because provenance and included fields were unclear. | Put a preview, fields list, and confidence note in the export moment. |

## Decision

Proceed to a constrained M-27 guided-rollout experiment, not a broad default change. The experience will lead with a single “first outcome” path, permission explainer, one weekly digest default, and an export preview. The gate is conditional on a 7.6-point adoption lift trajectory (18.4% to 26.0%) without a material increase in opt-outs or access-related support contacts.

## Next experiment

Run a moderated prototype test with 16 synthetic workspaces from 2026-07-20 through 2026-07-31. Compare the current sequence with the guided sequence. Primary signal: completed first outcome within ten minutes. Guardrails: permission comprehension at least 80%, notification opt-out no higher than 12%, and export-confidence rating at least 4/5. See `experiment-brief-m27.md` and `prototype-test-plan.md`.

## Open questions

- Does the permission explainer reduce hesitation for administrators and contributors equally?
- Which single weekly digest signal best earns attention without creating notification fatigue?
- Does showing export fields before generation improve confidence or merely add friction?

