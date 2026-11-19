---
path: Documents/Capacity/August capacity check - what to line up.md
kind: text
author: Daniel Jennings
derived_from:
---

# August capacity check — what to line up

Matthew,

For Louise's check on 18 August, please bring the 17 August Fare Payments aggregate for each hour from 15:00 to 19:00 UTC. We need the p95 of the one-minute eligible attempt rates in each half-open hour, with the count of valid minutes. Exclude duplicate retry indications and diagnostic traffic on the same basis throughout.

Alongside it, bring an independently verified sustainable ceiling for the same workload mix and normal worker pool. Please write down exactly which pool and mix both figures cover, the hold period and the observed sustained rate. A planning ceiling on its own is not a verification of sustainable capacity.

Flag any missing minutes and their bands. Don't fill them from a daily average; tell Louise what cannot be compared. The result needs to identify the busiest hourly p95 and the remaining capacity against the comparable verified ceiling. Keep failover and production change safety outside the claim unless separately assessed.

Daniel
