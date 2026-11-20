---
path: Documents/配車の見通し調査/Findings/2026-08-04 Dispatch scenes and people.xlsx
kind: carver-dispatch-findings.xlsx
author: Jennifer Salazar
derived_from:
---

# Dispatch scenes and people

管理欄：DR · DEC · CWR-DR-20260804-DEC-01 · v1.0 · August 4, 2026 · Jennifer Salazar / Brian Chen

Jennifer and Brian reconciled the July index and transcripts on August 4, 10:30–11:35 CDT. A scene is identified by route and elapsed interview time. Count one row per person; preserve all three UX-184 scenes. An account of a prior shift is not an observed workplace event. The three UX-184 actions were observed in the interview.

## Sheet: Dispatch findings

6行目が見出し、7〜32行目が明細。A列 Scene、B列 Participant、C列 Person flag、D列 Decision、E列 What was established、F列 Evidence and limit。

| Scene | Participant | Person flag | Decision | What was established | Evidence and limit |
|---|---|---:|---|---|---|
| R-352 / 12:50 | UX-166 | 1 | Existing material | Today's slip identified pending loading confirmation. | Participant account; yesterday's slip was set aside. The current slip, not a new answer, supplied the reason. |
| R-373 / 24:15 | UX-167 | 1 | Unconfirmed | Vehicle coordinator confirmed van type. | Participant account; replacement decision and hold reason remained unconfirmed. |
| R-362 / 16:10 | UX-168 | 1 | Existing material | Separate handoff notebook line identified pending loading confirmation. | Participant account; matched the route after reading a stop-order correction. |
| R-367 / 19:40 | UX-169 | 1 | Supplement or currency | Outgoing dispatcher identified receiving contact as the source of the awaited reply. | Participant account: callback wait was already in the message; asked outgoing dispatcher whose reply, not receiving directly. No reply or release established. |
| R-381 / 15:35 | UX-170 | 1 | Unconfirmed | A message concerned adjacent R-382. | Participant account; rejecting another route's reason does not establish R-381's reason. |
| R-392 / 14:20 | UX-171 | 1 | Unconfirmed | Stop 02 Note gave the west door instruction. | Participant account; instruction is not the hold reason. |
| R-398 / 20:30 | UX-172 | 1 | Additional answer | Yard contact said a loading bay had not been assigned after reading back the route. | Participant account; the additional exchange supplied what was outstanding. |
| R-405 / 22:05 | UX-173 | 1 | Additional answer | Outgoing dispatcher said loading was pending after the participant asked what was unfinished. | Walkthrough and participant account; Held alone was known at handoff. Stop instruction is separate. |
| R-408 / 29:15 | UX-174 | 1 | After handoff | Inventory reconciliation was pending, learned after Held was passed on. | Participant account; the answer was not available at handoff. |
| R-416 / 17:10 | UX-175 | 1 | Existing material | Full notification linked route to pending answer about receiving time. | Participant account; preview had cut off the route number. |
| R-421 / 18:25 | UX-176 | 1 | Additional answer | After the route number was repeated, outgoing dispatcher identified the loading supervisor's pending reply. | Participant account; earlier answer concerned another route. |
| R-430 / 13:45 | UX-177 | 1 | Unconfirmed | Ready changed to Held as Window changed; participant suggested a possible delay. | Interview walkthrough and explanation; delay was a guess, not an established reason. |
| R-434 / 21:30 | UX-178 | 1 | Existing material | Outgoing dispatcher's numbered note identified vehicle desk inspection pending. | Walkthrough and explanation; note did not identify release authority. |
| R-439 / 23:40 | UX-179 | 1 | Additional answer | Outgoing dispatcher identified a pending callback about the receiving slot. | Participant account; Window returning to its old value did not clear Held. |
| R-445 / 12:25 | UX-180 | 1 | Unconfirmed | Stop 01 Note identified the entry gate. | Walkthrough and explanation; no evidence links gate instruction to the hold. |
| R-451 / 26:05 | UX-181 | 1 | Unconfirmed | Participant rejected a loading-wait message dated the previous day. | Walkthrough and account; same route number does not establish today's reason. |
| R-458 / 18:45 | UX-182 | 1 | Supplement or currency | Participant read route and Window to receiving desk; desk said permission to receive was still pending. | Participant account of call; old copy had a reason but its specific wording is not established. Paper could not establish currency; no permission or release was obtained. |
| R-463 / 16:30 | UX-183 | 1 | No hold | Route stayed Ready during the described handoff. | Participant account; no hold was encountered in that period. |
| R-472 / 12:40 | UX-184 | 1 | Unconfirmed | Selected Held row and opened Stops; reason remained unknown. | Observed in interview; first of this participant's three scenes and the only counted row. |
| R-476 / 21:15 | UX-184 | 0 |  | Read west loading door instruction at stop 02. | Observed in interview; stop instruction did not explain why the route was held. Same person as R-472. |
| R-481 / 32:50 | UX-184 | 0 |  | Returned from detail to the same Held row without a reason to pass on. | Observed in interview; same person as R-472, not another count. |
| R-484 / 11:55 | UX-185 | 1 | Before list | Outgoing dispatcher told Mara the route and pending receiving response before she opened the list. | Participant account; row later showed Held. She did not receive the response or call receiving in this account. |
| R-489 / 22:30 | UX-186 | 1 | Additional answer | Caller confirmed route and said the loading list still needed checking. | Participant account; participant read back the route on the same call before the outstanding item was identified. |
| R-493 / 31:10 | UX-187 | 1 | Unconfirmed | Passed Held and a request to check with the outgoing dispatcher. | Participant account; a contact for follow-up is not the reason. |
| R-501 / 25:35 | UX-188 | 1 | Additional answer | Departure coordinator said a departure bay was not yet free on the second call. | Participant account; first answer only said route could not yet go. |
| R-509 / 20:10 | UX-189 | 1 | No hold | Route remained Ready while the scheduled time was corrected. | Participant account; no hold was encountered in the period described. |

集計はA列に式、B〜D列の結合セルに以下のラベルを置く。

| 行 | A列の式 | B〜D列ラベル | 期待値 |
|---:|---|---|---:|
| 33 | `=SUMIF(D7:D32,"Unconfirmed",C7:C32)` | Unconfirmed | 8 |
| 34 | `=SUMIF(D7:D32,"After handoff",C7:C32)` | After handoff | 1 |
| 35 | `=SUMIF(D7:D32,"Additional answer",C7:C32)` | Additional answer | 6 |
| 36 | `=SUM(A33:A35)` | Recommendation evidence | 15 |
| 37 | `=SUMIF(D7:D32,"Existing material",C7:C32)` | Existing material | 4 |
| 38 | `=SUMIF(D7:D32,"Supplement or currency",C7:C32)` | Supplement or currency | 2 |
| 39 | `=SUMIF(D7:D32,"Before list",C7:C32)` | Before list | 1 |
| 40 | `=SUMIF(D7:D32,"No hold",C7:C32)` | No hold | 2 |
| 41 | `=SUM(A37:A40)` | Comparison | 9 |
| 42 | `=SUM(C7:C32)` | People | 24 |
| 43 | `=COUNTA(A7:A32)` | Scenes | 26 |
| 44 | `=A36+A41-A42` | Group totals less people; zero indicates agreement | 0 |

45行目注記：These are selected interviews, not an estimate of prevalence. The comparison group is not a single “no problem” category. A reason, its current source and release authority are different facts. Each decision describes only the available account or interview observation.
