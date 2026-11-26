---
path: Documents/Recovery-evidence/Ticket archive candidate review.pptx
kind: brt-reliability-briefing.pptx
author: Daniel Jennings
derived_from:
---

## Slide 1 — Cover

Title: Ticket archive candidate review

BRT-RB-20240918-001  
18 September 2024 · Daniel Jennings

Document reference: BRT-RB-20240918-001

Speaker notes: For Anjali Shah, Sanjay Mehta and Claire Hutchinson at the 19 September review, 10:00 UTC. Ask which intervals we should carry forward under load. The September comparison tells us whether the last archive write was still open when the next one started. A decision about operator recovery wording needs a separate readback.

## Slide 2 — Heading and body

Title: How we observed the starts

- Isolated archive, clear queue at opening on both days.
- Six hours each, 09:00–15:00 UTC. Same request profile, with the busier hour at 12:00–13:00 UTC.
- Count actual starts from 09:00 inclusive to 15:00 exclusive.
- Check the immediate predecessor by write reference and full UTC seconds.

Document reference: BRT-RB-20240918-001

Speaker notes: Sanjay and Patrick observed the 15-minute schedule on 9 September and the 10-minute schedule on 11 September. The actual start marks determine the denominator and the hour. The first start counts with no predecessor. A matching completion at or before the next start second counts as clear. A later completion belongs to the earlier write, even if we find it after the window closes. A missing completion check stays unresolved.

## Slide 3 — Table

Title: Two intervals for the load trial

| Schedule and window | Open predecessor / starts | What we observed | Proposed use |
|---|---:|---|---|
| 15 minutes · 9 September, 09:00–15:00 UTC | 0/24 | Four starts in each hour. No overlaps observed. | Keep as comparator. |
| 10 minutes · 11 September, 09:00–15:00 UTC | 2/36 | Six starts in each hour. Both overlaps fell in the 12:00–13:00 UTC block. Both earlier finishes verified. | Keep as trial candidate. |

Document reference: BRT-RB-20240918-001

Speaker notes: Use BRT-IC-20240916-001 for the twelve hourly rows and RJS-WS-20240912-001 for the witness findings. The short interval's busy hour accounts for both flags, 2 of 6 starts, with zero flagged starts in the other five hours. The two cases are included in the hourly tally. The September runs establish the overlap count under these observation conditions.

## Slide 4 — Figure

Title: The two predecessor finishes

Figure asset: `Pictures/Reliability-charts/Ticket archive starts and finishes.png`

Caption: 11 September 2024. The lower tier shows the two checked finishes, paired with the earlier starts by write reference.

Figure contents from the linked chart: “Ticket archive starts and finishes · 11 September 2024”. Shared Time (UTC) axis from 12:05 to 12:45, marked every five minutes. Upper tier shows scheduled starts on the 10-minute interval. Lower tier shows confirmed finishes (exception extract). Legend: Scheduled start, Confirmed predecessor finish, Overlap after next start.

| Earlier write | Earlier start (UTC) | Next start (UTC) | Confirmed earlier finish (UTC) | Open after next start |
|---|---|---|---|---|
| W-3184 | 12:10:00 | 12:20:00 | 12:22:10 | 2m 10s |
| W-3186 | 12:30:00 | 12:40:00 | 12:41:35 | 1m 35s |

Document reference: BRT-RB-20240918-001

Speaker notes: Read across the upper tier to the next start, then follow the earlier write's reference to its finish below. At 12:20, W-3184 had been open since 12:10 and remained open for another 130 seconds. At 12:40, W-3186 had been open since 12:30 and remained open for another 95 seconds. Sanjay, Patrick and I checked both finish references. The finish second closes that write and gives the end of its overlap. To find the newest restorable record, we need a coverage check on an eligible set.

## Slide 5 — Two columns

Title: Scope to agree on 19 September

Left column — Trial scope

- Keep 15 minutes as comparator and 10 minutes as candidate.
- Run both in isolation under matched sustained foreground load.
- Keep the live schedule and current operator recovery statement in force.

Right column — Evidence still due

- Agree windows and one sustained request profile with Patrick.
- Follow open predecessors to their matching finish. Record unresolved checks.
- Read back the latest eligible set at a fixed UTC reference against the inclusive 15-minute target.

Document reference: BRT-RB-20240918-001

Speaker notes: Ask Anjali to approve the trial scope. Sanjay can agree the windows and common load profile with Patrick before booking. I'll specify the paired evidence and where the UTC reference falls in each run. A set is eligible if it has completed by that reference. Confirm how far its restorable records reach, then subtract that coverage time from the fixed reference. Exactly 15 minutes meets the existing test target. Record time spent restoring separately. Leave the August exception open pending the readback. Ask Claire to continue using the current operator statement.
