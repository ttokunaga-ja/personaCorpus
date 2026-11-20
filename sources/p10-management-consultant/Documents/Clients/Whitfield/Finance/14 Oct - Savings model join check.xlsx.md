---
path: Documents/Clients/Whitfield/Finance/14 Oct - Savings model join check.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Sheet: Name match

Department and item names used as the initial join. Annual values are those carried in the original estimate.

| Original row | Department and item string | Name-match result | Candidate using same quantity | Evidence needed next |
|---|---|---|---|---|
| AP model!D12 | AP / Routine checks | No matching Finance string | None identified | Confirm AP-CHECK period and count |
| AP model!D13 | AP / Independent follow-up contacts | No matching Finance string | None identified | Confirm AP-CALL is independent of routine checks |
| AP model!D14 | AP / Exception document packs | No matching Finance string | Finance model!D11: 2,000 packs | Trace work key, source count and preparation records |
| Overtime!D8 | Overtime / Avoidable premium | No matching Finance string | None identified | Check approved hours and premium-only rate |
| Finance model!D11 | Finance / Handover preparation | No matching AP string | AP model!D14: 2,000 packs | Trace receipt identifiers back to AP preparation |

The two candidate names do not match the join string, so the initial match does not link them. Original amounts total JPY 26m; agreement to that total does not establish independent benefits. Tomorrow morning match the work key and source of the counts, then compare what each team actually does. Do not remove either line on a name or amount alone.
