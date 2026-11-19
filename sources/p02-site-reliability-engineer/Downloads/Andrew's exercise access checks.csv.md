---
path: Downloads/Andrew's exercise access checks.csv
kind: text
author: Daniel Jennings
derived_from:
---

| Check | Expected action | Observed result | Result | Follow-up owner |
|---|---|---|---|---|
| Read request | Andrew opens and reads the exercise decision request | Request readable on the exercise copy | Pass | Daniel: retain result for Emma |
| Record approval | Andrew records an approval in the exercise decision record | Decision record is readable only; approval cannot be written | Fail | Emma: confirm entitlement and exercise-only access change |
| Review verification | Andrew views both exercise sign-in verification results | Both results visible | Pass | Daniel: retain result for Emma |

Reconciliation outside the CSV: 2 of 3 checks passed on 17 October 2024. The failed write needs a witnessed recheck before Andrew can be relied on to record the decision.
