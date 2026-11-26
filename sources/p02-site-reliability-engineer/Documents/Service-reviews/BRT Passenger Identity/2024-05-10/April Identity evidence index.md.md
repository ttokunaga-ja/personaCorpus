---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-10/April Identity evidence index.md
kind: text
author: Daniel Jennings
derived_from:
---

# April Identity availability — evidence index

10 May 2024 · Daniel Jennings

For the next monthly reviewer: the April close covers 1 April 00:00 to 1 May 00:00 UTC, start inclusive and end exclusive. Keep the original flash with the close; it records what was known at dispatch, not the final result.

- [1 May flash, BRT-AV-20240501-001](../2024-05-01/April%20Identity%20availability%20-%20Flash.pdf) — the initial 59-minute charge included all 18 collector blanks as provisional downtime. It has been superseded, not silently corrected.
- [10 May issued report, BRT-AV-20240510-001](April%20Identity%20availability.pdf) — the closed reconciliation and decision: 41 known failed minutes, four unobserved, and an availability range of 99.89583%–99.90509%. Target status is undetermined.

## Exports

| File | Question it answers |
|---|---|
| [Primary collector, 30 April hourly](../../../../Downloads/passenger_identity_hourly_apr30.csv) | What did the primary collector record on the last UTC day? Its 23:00 hour has 42 successes and 18 missing outcomes. It does not resolve the missing tail. |
| [Independent verifier, 30 April](../../../../Downloads/identity_verifier_apr30.csv) | Which scheduled minutes in that tail have a separately retained completed sign-in outcome? Fourteen match successful outcomes; the four slots starting 23:51, 23:52, 23:53 and 23:54 UTC have no matched outcome. |
| [Reconciled April hourly figures](../../../../Downloads/April%20Identity%20hourly%20figures.csv) | How do primary successes, known failures, recovered successes and unobserved minutes add up across the full month? The last hour separates 42, 0, 14 and 4 respectively. |

The residual is **30 April 23:51–23:55 UTC**, end exclusive. The verifier confirms success from 23:42–23:51 and from 23:55–1 May 00:00 UTC; the residual between those runs remains unobserved. Please start there if looking for further evidence, rather than reopening all 18 minutes as unresolved. Neither endpoint supplies an outcome for the four intervening slots.
