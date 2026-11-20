---
path: Documents/Recovery-evidence/Archive timestamp selection.md
kind: text
author: Sanjay Mehta
derived_from:
---

# Which archive clock to compare

Daniel,

The set we read back finished writing at 09:44 UTC on 19 August. That stamp tells us it was complete before the 10:00 UTC reference and is eligible for this exercise. The coverage marker on that set says it contains records through 09:38 UTC. It is the latest restorable coverage point among the complete eligible sets we checked.

Please retain both entries, but use 09:38 when subtracting from 10:00. Moving the point forward to 09:44 would claim six more minutes of coverage than the marker supports; completion of the write does not establish that those later records are present. The result is 22 minutes old, seven over the inclusive 15-minute limit.

The first calculation, 10:00 minus the completion stamp, gives 16 minutes and also misses the limit. That does not make it a valid measure of the point. Neither difference is a measure of how long the restore took. The clock readings alone do not tell us the cause of the miss.

Sanjay
