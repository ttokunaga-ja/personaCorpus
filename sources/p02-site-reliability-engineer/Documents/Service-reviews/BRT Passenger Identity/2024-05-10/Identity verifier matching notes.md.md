---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-10/Identity verifier matching notes.md
kind: text
author: Emma Wilkinson
derived_from:
---

# April Identity — verifier matching

Daniel, the separately retained verifier aggregate records completed sign-in outcomes against the scheduled UTC minute used by the primary collector. It is not simply a count of requests sent. Matching is to the scheduled slot, not the time a retry happened; retries within one scheduled minute are deduplicated so that the aggregate has at most one matched outcome for that minute.

For the collector tail, the export has one scheduled check per minute. The first nine slots, 30 April 23:42 up to 23:51 UTC, each have one matched completed success. The next four slots, starting 23:51, 23:52, 23:53 and 23:54, each have zero matched outcomes and a blank outcome field. A blank tells us that this aggregate has no matched outcome for the slot; it does not establish either success or failure.

The slot starting 23:55 has a matched completed success, as do the slots starting 23:56 through 23:59. That is a second run of five successes, 23:55 up to 1 May 00:00 UTC. The two runs give 14 distinct successful minutes, not a continuous 18-minute run. The 00:00 endpoint is excluded from April; a check scheduled at 00:00 would belong to May.

Emma
