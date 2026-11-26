---
path: Documents/Reading-notes/Reading availability with missing data.md
kind: text
author: Daniel Jennings
derived_from:
---

# Reading availability with missing data

8 May 2024 · Daniel · for the next monthly reviewer

Use the full scheduled UTC window as the denominator. April Passenger Identity has 43,200 eligible minutes: 43,155 confirmed successes, 41 observed failures and four unobserved. Start inclusive, end exclusive, from 1 April 00:00 to 1 May 00:00 UTC.

## Two possible extremes

If all four unknown minutes failed, there are 45 failed minutes and 43,155 successes: 43,155 / 43,200 = 99.89583%. If all four succeeded, there are 41 failures and 43,159 successes: 43,159 / 43,200 = 99.90509%. Neither extreme asserts an outcome for the four minutes; together they bound the answer. Compare unrounded values with 99.9%: the target is inside the interval, so its status is undetermined.

Dropping the four would give 43,155 / 43,196 = 99.90508% approximately. That asks how often sign-in succeeded **among observed minutes**, not availability over the whole eligible month. It cannot establish the four missing outcomes.

## Three different quantities

- Observed failures: 41 scheduled minutes with failed sign-ins, in the 11 April 10:12–10:24 and 24 April 17:21–17:50 UTC windows.
- Primary collector coverage: 43,182 / 43,200 minutes, including the 41 observed failures. Its 18-minute gap is a collection gap, not 18 proven failures. A separate verifier recovers 14 successes.
- Service availability interval: 99.89583%–99.90509% after reconciliation, with four minutes still unobserved. A collector coverage rate is not the service's success rate.

Do not fill the four interior minutes from the successes on either side. Keep them visible beside the monthly figure.
