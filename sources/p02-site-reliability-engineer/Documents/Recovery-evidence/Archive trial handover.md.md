---
path: Documents/Recovery-evidence/Archive trial handover.md
kind: text
author: Daniel Jennings
derived_from:
---

# Archive trial handover

Friday 20 September 2024, 10:55 UTC

Sanjay,

Your follow-up this morning asks Patrick for two proposed windows by Tuesday 24 September. Neither is booked. Once he replies, please agree the request profile and load size with him before booking. We need the same sustained foreground load in isolation for the 15-minute comparator and the 10-minute candidate.

I'll set the evidence requirements for the two runs. Please check with Patrick that we can keep the start and finish records even if a write is still open when the window ends. For each overlap, keep the new start and the preceding write's reference and matching finish, all to the UTC second. If we can't confirm the finish, leave the pair open on the sheet. A finish after the window still belongs to that pair.

Let's also arrange a coverage readback for each run. Agree the UTC reference in advance, then check the latest confirmed record coverage among restorable sets that were complete by that reference. Take the age from the coverage timestamp. The target for the check is 15 minutes or less. If we can't establish the coverage, leave the result unresolved. We still need that evidence to close the August exception.

For the September numbers, use `Downloads/RJS archive hourly observations.csv` for the counts. It has six hourly rows per run, 0 overlaps in 24 starts for the comparator and 2 in 36 for the candidate, both in the 12:00–13:00 UTC hour on 11 September. The separate `Ticket archive overlap pairs.csv` in this folder is the two-row finish extract you sent Patrick. I checked both against the earlier write references with him. W-3184 was open at 12:20:00 and finished at 12:22:10 UTC. W-3186 was open at 12:40:00 and finished at 12:41:35 UTC. Those finishes close the two overlap checks. The September runs left coverage at a fixed reference unmeasured.

Keep `Desktop/Ticket archive trial - draft.docx` with the working papers. Anjali's v0.1 still makes the early claim that a shorter interval improves the recovery point. For the Operations decision, use `Documents/Shared-drive/Ticket archive trial scope.pdf` (BRT-CD-20240919-001 v1.0, issued 19 September). It carries the decision to take both intervals into the isolated load trial. The live schedule and Claire's existing wording for operators stay in force.

Daniel
