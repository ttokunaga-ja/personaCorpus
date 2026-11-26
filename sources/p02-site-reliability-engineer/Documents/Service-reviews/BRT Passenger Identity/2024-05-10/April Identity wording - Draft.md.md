---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-10/April Identity wording - Draft.md
kind: text
author: Daniel Jennings
derived_from:
---

# April Passenger Identity — cover wording for Emma

3 May 2024 · Daniel Jennings · Draft for discussion

## Proposed headline

**April availability: 99.90509% against a 99.9% target.**¹

The April window contains 43,200 scheduled sign-in minutes. We have 41 observed failures. The primary collector was blank for the final 18 minutes of the month; Emma's separately retained verifier confirms 14 successful scheduled minutes within that gap. Four minutes, 23:51–23:55 UTC on 30 April, have no matched outcome.

The 14 recovered outcomes are nine from 23:42 to 23:51 and five from 23:55 to 00:00 UTC. They are completed sign-ins matched to their scheduled minutes, not a carry-forward across the gap. The 1 May flash charged all 18 missing minutes as downtime, producing 59 charged minutes and 99.86343%; that provisional estimate needs to be replaced when the report closes.

¹ If all four unobserved minutes failed, availability would be 99.89583% (45 of 43,200 minutes charged). The headline above instead assumes those four succeeded, giving 41 charged minutes. Emma: please discuss whether the lower bound belongs in the cover rather than this footnote before we issue.
