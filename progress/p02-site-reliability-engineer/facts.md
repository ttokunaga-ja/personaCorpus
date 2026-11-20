# Daniel Jennings — shared facts

## Faregate payment interruption (INC-260713), 13 July 2026

- BRT Fare Payments, Faregate checkout path. File times are BST (UTC+01:00); incident evidence is UTC. The dashboard observation at 09:07 BST converts to 08:07 UTC, the first confirmed affected attempt. The on-call page arrived at 08:12 UTC and supplied the provisional onset.
- Matthew paused the staged rollout and returned traffic to the last stable route at 08:29 UTC. Recovery was observed at 08:41 UTC (34 minutes after onset); the follow-up watch passed at 08:56 UTC. Cause and causal attribution to the intervention remain open.
- The 08:41–08:56 UTC watch counted 740, 820 and 900 eligible attempts in successive five-minute windows, with zero repeat incident failures (2,460 attempts total).
- Initial internal update at 08:18 UTC gave about 380 failure indications, rounded from 384. Reconciled at 10:20 UTC: 312 eligible failed attempts + 48 duplicate indications from retries within those same attempts + 24 ineligible diagnostic indications = 384. The 312 were among 4,800 eligible Faregate checkout attempts in [08:07, 08:41) UTC.
- The 312 is the service-wide incident checkout-attempt count, not a people count or a Westbury-only count. Four hourly Faregate buckets starting 07:00 through 10:00 UTC have eligible totals 7,960 / 8,640 / 9,100 / 8,800 and confirmed incident failures 0 / 312 / 0 / 0.
- Evidence readback on July 13 at 10:20 UTC: Daniel Jennings, Matthew Pickering and Claire Hutchinson. Matthew owns the impact reconciliation, Daniel the budget calculation, Claire the customer wording. Matthew's cause-evidence comparison is due July 17 at 12:00 UTC.

## July Fare Payments budget and change freeze

- BRT-EB-20260713-001 (Daniel, July 13) applies the 99.9% target to eligible Fare Payments attempts, July 1 00:00 through July 13 11:00 UTC, end exclusive. July 1–12: 1,200,000 eligible and 908 failed. July 13 through cutoff: 50,000 eligible and 312 failed.
- At the cutoff 1,250,000 attempts allow 1,250 failures. Of those, 1,220 were used and 30 remained, so the remaining allowance was 2.4%.
- BRT-CD-20260713-001, issued July 13 at 12:10 UTC by Anjali Shah, continues the existing normal production change freeze for BRT Fare Payments on the strength of BRT-EB-20260713-001. Hold the next Faregate rollout tranche and the routine payment reconciliation release.
- Non-production investigation may continue. Urgent production mitigation requires a separately recorded Operations approval. July 14 10:00 UTC queue checkpoint (Anjali, Daniel and Matthew): both items remain held, with no newly reconciled budget balance or automatic release.

## Faregate internal update and incident review

- “Faregate checkout: initial estimate” thread, July 13: (1) Daniel → Matthew, Claire and Anjali at 08:18 UTC (09:18 BST), with another internal update promised by 09:15 UTC; (2) Matthew → Daniel, Claire and Anjali at 09:10 UTC (10:10 BST), “Re: Faregate checkout: initial estimate”.
- BRT-IR-20260714-001 is Daniel's July 14 incident review reference, distinct from INC-260713. Working v0.1 saved July 13 15:42 UTC, Matthew's marked v0.1 returned July 14 08:35 UTC, and issued v1.0 released July 14 13:30 UTC. The first two copies are unissued.

## Westbury desk guidance after Faregate recovery

- “Westbury desk wording for pending receipts” thread, July 13: (1) Stephen Cartledge → Claire Hutchinson, Cc Daniel, at 13:02 UTC (14:02 BST); (2) Claire → Stephen, Cc Daniel, at 14:15 UTC (15:15 BST), “Re: Westbury desk wording for pending receipts”.
- Claire's BRT-CU-20260713-001, issued July 13 at 14:05 UTC, is sent as Faregate recovery and change restriction.pdf with her reply. Desk guidance is to check the booking and payment outcome or contact Support before asking for a second payment.

## Fare Payments peak capacity review, 28 April 2025

- Common reference is 1,000 eligible Fare Payments attempts/min, excluding duplicate retry indications and diagnostics. Palmer forecasts the hourly p95 of one-minute rates; half-open hourly bands starting at 15:00, 16:00, 17:00 and 18:00 UTC are 1.70×, 2.05×, 2.30× and 1.90×.
- Old cap 2.00× leaves −0.05× at 16:00–17:00 UTC and −0.30× at 17:00–18:00 UTC. Proposed 2.80× covers the peak with +0.50× of the reference rate: 0.25× for short bursts and 0.25× for retaining capacity during worker replacement. These are sizing allowances, not observations.
- PCS-DF-20250425-001 is Louise Turner's issued forecast; BRT-CP-20250428-001 is Daniel Jennings's options workbook; BRT-RB-20250428-001 is his approval briefing.
- Review on 28 April at 11:00 UTC (12:00 BST): Anjali Shah, Daniel and Matthew Pickering. In BRT-MT-20250428-001 Daniel records Anjali's approval of the 2.80× planning ceiling, Matthew's separate burst/replacement verification and Daniel's August reserve-cost readback.
- “Fare Payments peak forecast for 28 April” thread, BST: (1) 25 April 14:18 Louise → Daniel, Cc Matthew, attaches Palmer Fare Payments peak forecast.pdf; (2) 25 April 15:06 Daniel → Louise, Cc Matthew, “Re: Fare Payments peak forecast for 28 April”, quotes (1); (3) 28 April 09:12 Louise → Daniel, Cc Matthew, same Re: subject, answers directly without quoting.
- April thread Message-IDs, in order: `<7b28c5e4-0b63-4b59-93ce-d0f641a8de72@palmer-capacity.example>`, `<9c61c43b-5913-4d75-8f9e-2df3cb77ea06@brt-booking.example>`, `<f4385ae7-ec96-4475-b5b0-c37ac5d9e142@palmer-capacity.example>`. Palmer dispatches use `louise@palmer-capacity.example`.

## Fare Payments reserve cost readback, 18 August 2025

- Like-for-like recurring monthly allocation: GBP 14,600 standard payment worker pool + GBP 2,600 baseline supporting capacity = GBP 17,200 approved budget; GBP 700 warm reserve workers + GBP 500 reserved overflow capacity = GBP 1,200 above budget; total GBP 18,400.
- Daniel Jennings and Richard Fenner meet at 10:00 UTC (11:00 BST). The GBP 500 reservation-only cut would leave GBP 17,900 and remove the overflow reservation. The cut is deferred and neither reserve line is removed.
- “August Fare Payments reserve treatment”: 18 August 12:14 BST, Richard → Daniel, Cc Anjali Shah; standalone follow-up confirms the reserve classification and retained allocation.

## Fare Payments observed headroom, 18 August 2026

- Matthew Pickering's representative normal-pool rehearsal on 17 August verified 2,800 eligible attempts/min as the highest sustained rate during 13:00–13:30 UTC. Failover and production change safety were outside the capacity check.
- For 17 August live traffic, the four half-open hourly bands starting at 15:00, 16:00, 17:00 and 18:00 UTC have 60 valid one-minute observations each and p95 rates of 1,680, 1,848, 2,016 and 1,792 eligible attempts/min. Relative to 2,800/min, unused capacity is 40%, 34%, 28% and 36%.
- Louise Turner's PCS-DF-20260818-001 (18 August) reports 784/min remaining at the busiest observed band, so headroom is (2,800 − 2,016) / 2,800 = 28%. This percentage uses a verified ceiling, whereas the 2025 approved 2.80× multiplier uses the normal reference rate.
- “August Fare Payments capacity check”: 18 August 10:15 BST, Louise (louise@palmer-capacity.example) → Daniel Jennings, Cc Matthew, standalone dispatch with Palmer August capacity check.pdf attached.

## Fare Payments limited reconciliation restart, 18 August 2026

- Anjali Shah, Daniel Jennings and Matthew Pickering review at 11:00 UTC (12:00 BST). The 25% job-share proposal stays unapproved. Anjali issues BRT-CD-20260818-001 citing PCS-DF-20260818-001; the shared PDF is her issued Word decision exported unchanged.
- One 19 August window, 09:00–09:30 UTC (10:00–10:30 BST): shadow reads for up to 10% of eligible reconciliation jobs with the existing production path kept in place. Preflight requires a no-write/no-post check, a verified normal pool and matching workload mix.
- Daniel monitors headroom against 2,800/min using p95 one-minute rates over a trailing 15-minute window. Preflight requires at least 25%; Matthew owns start/stop and stops below that floor or on any output discrepancy or customer-facing regression.
- The normal reconciliation release and next Faregate rollout tranche stay held under BRT-CD-20260713-001; the shadow approval does not extend to them. Operations reviews observations or the stand-down reason at 11:00 UTC (12:00 BST) on 19 August; expansion requires a further decision by Anjali.
- “Approval for 19 August reconciliation shadow window”: 18 August 13:32 BST, Anjali → Matthew and Daniel, Cc Claire Hutchinson, standalone dispatch with August reconciliation shadow approval.pdf attached.

## March night pages and April handover review

- March 2024 observation period, UTC: eight rota engineers received 46 night-call deliveries, comprising 28 first calls, 10 reminder repeats and 8 parallel-route repeats. A repeat points to the same still-open item; a first call counts a delivery, rather than establishing a unique incident.
- Borderline examples: 17 March 23:58 → 18 March 00:03 UTC is one item still open, so the second delivery is a repeat. The same label at 01:12 and 02:06 UTC on 22 March refers to two openings separated by a recorded clearance.
- Andrew's BRT-NH-20240408-001 covers Daniel's 8–9 April watch, 22:00–07:00 BST. The old sheet groups the Fare Payments repeat (last seen 15:40 UTC) and pending Support receipt query (15:52 UTC) under a shared instruction and a single untimed tick; Anjali comments on the same issue on 9 April without reissuing it.
- Daniel and Anjali meet on 9 April at 13:00 UTC (14:00 BST). BRT-MT-20240409-001 keeps reminders in place while Daniel and Andrew trial a named owner, next check and item-level acknowledgement, with unacknowledged items left visible.
- “April night review: handover first” is standalone, 9 April 15:48 BST, Daniel → Andrew, Cc Anjali, with April night handover review decisions.pdf attached. Message-ID: `<209eb2c7-a466-4336-bf25-598e98e0c067@brt-booking.example>`.

## May rota review and June Friday exchanges

- Daniel's 7 April–4 May 2025 readback counts 25 distinct answered night calls for eight rota engineers: Daniel 9, Matthew 8, other six 8 combined. Attribute by actual responder; Matthew answered the 25 April 01:42 BST call on Daniel's planned watch.
- Daniel and Andrew meet 9 May at 11:00 BST. BRT-MT-20250509-001 records the change and readback requirement. The board order is incoming primary → named cover if no answer → Anjali if still unclaimed, with owner and next check read back.
- BRT-OX-20250509-001, issued 9 May by Daniel: 6 and 20 June Daniel → Andrew; 13 and 27 June Matthew → Sanjay. All four watches start at 22:00 Friday and end at 07:00 Saturday, Europe/London (BST); all other June assignments stay in place.
- “Please check the four June Friday exchanges” thread, 9 May BST: (1) 13:26 Daniel → Andrew, Sanjay and Matthew, attaches June Friday night exchanges.xlsx and asks for readback by 16:00; (2) 14:02 Andrew → Daniel, Cc Sanjay and Matthew; (3) 14:37 Sanjay → Daniel, Cc Andrew and Matthew. Both replies use “Re: Please check the four June Friday exchanges”, quote (1) and carry no new attachments.
- Thread Message-IDs in order: `<bdeb11e6-e978-4a21-853f-0057160997d1@brt-booking.example>`, `<73f9aa7b-fd84-4525-895a-186eb5ab4a2e@brt-booking.example>`, `<b63ddd66-2b10-4e05-81de-fe5994d301c1@brt-booking.example>`.
- Matthew gives Daniel a spoken readback at 15:48 BST on 9 May. Daniel records the four changes as entered and checked at 16:19 BST after the other two confirmations.

## November mock handover and supervised duty entry

- Daniel teaches and Andrew observes on 4 November 2025 at 10:00 GMT. Carys Morgan, Owen Price and Megan Davies make two routing decisions each. BRT-RB-20251103-001 is Daniel's training deck, issued on 3 November before the exercise.
- First exercise: 4 of 6 correct. Carys sends a single missing receipt after a completed payment to Matthew instead of Claire Hutchinson. Megan sends a cluster of failed Fare Payments to Claire instead of Matthew Pickering. Owen routes both items correctly.
- All three repeat the exercise on 6 November at 10:00 GMT; the entry gate is two correct destinations and a complete owner/next-check readback for both prompts per trainee, without coaching. Andrew records six correct destinations and six complete readbacks; Daniel records supervised entry in BRT-TR-20251106-001.
- Carys, Owen and Megan start their first supervised watches on 10, 12 and 14 November respectively, each 22:00–07:00 GMT with Andrew as named cover. Carys records her first morning readback on 11 November for a single receipt-status query owned by Claire, next check 09:00 GMT.
- “November trainees cleared for supervised duty” is standalone, 6 November 13:24 GMT, Daniel → Anjali and Andrew, with November handover refresher record.pdf attached. Message-ID: `<7d6a3fe0-4065-4ea5-9409-3958954856d3@brt-booking.example>`.
- Internal addresses for the rota and training correspondence use `@brt-booking.example` with local parts `daniel`, `anjali`, `andrew`, `matthew` and `sanjay`.

## Passenger Identity recovery exercise, 16 October 2024

- Exercise only. Daniel Jennings operates, Emma Wilkinson owns the decision, and Carys Morgan observes and keeps time. Both runs use a 20-minute target and the same clock: Carys issues the simulated sign-in failure cue and stops at the second of two consecutive successful exercise sign-ins one minute apart.
- 16 October UTC boundaries for scope/request, approval wait, restoration and verification: 09:00 cue → 09:04 logged request → 09:16 Emma records approval → 09:26 ready to verify → 09:34 second good sign-in (first at 09:33). The intervals are 4, 12, 10 and 8 minutes, 34 total and 14 over target.
- Daniel, Emma and Carys read the timings back at 13:00 UTC on 16 October. Acknowledging receipt did not release restoration; BRT-MT-20241016-001 names Andrew Cartwright to cover future Identity exercises, subject to checking the handover and authority.
- BRT-EX-20241018-001 is Daniel's 18 October baseline report, with the four original intervals intact and the 17–18 October actions dated separately. Retain the report and October CSV as issued; keep both exercises outside actual incident and availability totals.
- Standalone mail, BST: 15 October 15:06 “16 October exercise - who has the clock”, Emma → Daniel and Carys; 18 October 15:03 “October Identity exercise baseline for the review file”, Daniel → Emma and Anjali, with Passenger Identity recovery exercise - October baseline.pdf attached.

## Passenger Identity exercise decision cover and access, 17–18 October 2024

- Daniel's 17 October first pass proposed five minutes and parallel decision routes. Emma's revision replaces these with transfer after two minutes without a decision on a logged request, or immediately on her explicit handover.
- Andrew is the named, prechecked delegate. During transfer Daniel holds restoration; Andrew confirms the scope and acknowledges sole decision ownership in the exercise record before recording the decision, with no parallel approval by Emma. If no checked delegate accepts, hold and contact Anjali.
- 17 October check: Andrew can read the request and see both verification results but cannot write the decision, so 2 of 3. Emma adjusts the exercise-only access and witnesses the 18 October 09:15 UTC repeat with Daniel and Andrew: all three pass. Emma publishes the rehearsal role page on 18 October after the check.
- “Can Andrew record the Identity exercise decision” thread, BST: (1) 17 October 12:14 Daniel → Emma with Andrew's exercise access checks.csv attached; (2) 18 October 11:04 “Re: Can Andrew record the Identity exercise decision”, Emma → Daniel, Cc Andrew, quoting (1), with no new attachment.
- For internal mail, Emma and Carys use `emma@brt-booking.example` and `carys@brt-booking.example`.

## Passenger Identity recovery repeat, 20 January 2026

- At the 19 January 10:30 UTC preflight, Daniel, Emma and Andrew observe 3 of 3 checks on Andrew's own exercise access: read a request, record and read back a practice decision, and see both verification results. Emma leaves the timed request undecided by design so the repeat exercises the two-minute trigger.
- 20 January UTC boundaries: 09:00 cue → 09:03 logged request → 09:05 Andrew first accepts ownership then records approval → 09:12 ready to verify → 09:18 second good sign-in (first at 09:17). The same four intervals are 3, 2, 7 and 6 minutes, 18 total and two under target.
- Compared with October, the four intervals shrink by 1, 10, 3 and 2 minutes. The decision wait accounts for 10 of the 16 minutes saved; the cause of the other six minutes is unresolved.
- Daniel's comparison deck is BRT-RB-20260120-001. Daniel, Emma and Anjali meet at 14:00 UTC. Daniel's BRT-MT-20260120-001 and its PDF export retain one decision owner for the next rehearsal, assign Daniel to file the two runs separately that day and assign Andrew to recheck his access before the next run.
- Standalone mail, GMT: 19 January 14:12 “Tuesday Identity repeat - preflight clear”, Daniel → Emma, Andrew and Carys; 20 January 15:46 “January Identity exercise readout and interval extract”, Daniel → Anjali and Emma, Cc Andrew and Carys, attaches January exercise readout decisions.pdf and 20 January Identity recovery timings.csv.

## Ticket archive recovery check with RJS, 19–20 August 2024

- The 19 August check used an isolated exercise archive. Daniel Jennings and Sanjay Mehta read back the clocks with Patrick Sutcliffe of RJS Data Storage Ltd at 10:18 UTC. The fixed reference was 10:00 UTC.
- The latest complete eligible set covered through 09:38 UTC and finished writing at 09:44 UTC. Eligibility requires the set to be complete by the reference; age is reference minus coverage: 22 minutes, seven over the inclusive 15-minute limit. The abandoned completion-clock calculation was 16 minutes. Neither calculation represents time spent restoring.
- Patrick issued RJS-WS-20240819-001 on 19 August for Daniel and Sanjay. The witness observations support the clock readings and the missed target. The record leaves the cause of the miss open.
- “19 August ticket archive witness notes” thread, BST: (1) 19 August 13:23 Patrick → Daniel and Sanjay, attaches RJS 19 August archive witness observations.pdf; (2) 19 August 14:04 Daniel → Patrick, Cc Sanjay, quotes (1) and attaches 19 August archive timestamp correspondence.csv.
- (3) 20 August 09:17 Patrick → Daniel and Sanjay, quotes (2), no new attachment. Both replies use “Re: 19 August ticket archive witness notes”. Patrick's address is `patrick@rjs-storage.example`.
- Daniel's BRT-MT-20240820-001 is the 20 August 12:00 UTC review with Sanjay and Anjali and has a same-number PDF. Anjali keeps the miss open, Sanjay and Patrick take the interval review, and Daniel retains both clock readings and the original evidence for a comparable repeat.

## Ticket archive recovery repeat, 11 May 2026

- Daniel and Sanjay kept the 10:00 UTC reference and inclusive 15-minute limit. At 08:10 UTC Sanjay confirmed a 10-minute capture schedule in the isolated exercise environment, a separate coverage marker and an isolated restored copy.
- The eligible set covered through 09:51 UTC and finished writing at 09:55 UTC. Daniel and Sanjay confirmed it at 10:16 UTC. Its age was nine minutes, six inside the limit and 13 minutes younger than the August observation.
- Daniel issued BRT-RP-20260511-001 on 11 May for the two comparable checks. The maximum observed age was 22 minutes. The repeat demonstrates one pass under the exercise conditions. The evidence establishes neither a universal bound nor the cause of the difference between the runs.
- Standalone mail “May archive check for the wording review”, 11 May 13:10 BST, Daniel → Claire Hutchinson, Cc Sanjay, attaches 11 May evidence for the wording review.csv. The attached extract is internal.
- Daniel, Sanjay, Claire and Anjali met at 13:15 UTC on 11 May (BRT-MT-20260511-001, same number on the PDF). Anjali accepted the exercise result. Daniel files both runs, Sanjay retains the comparison conditions and Claire issues the updated evidence-bound wording.

## Westbury ticket archive recovery guidance, 11 May 2026

- Claire's BRT-CU-20260511-001 is dated 11 May in v0.1 (draft and Daniel's commented copy) and v1.0 (issued DOCX and same-version PDF). Daniel's comment at 14:00 BST flags the draft phrase “within nine minutes of recovery”.
- The approved wording describes records covered through 09:51 UTC at the 10:00 UTC exercise reference. The 09:52 UTC example requires a separate check with Support. Nine minutes describes the age of the point, and 15 minutes is the target for the exercise.
- Standalone mail “Ticket archive recovery guidance for Westbury”, 11 May 15:37 BST, Claire → Stephen Cartledge, Cc Daniel, attaches Ticket archive recovery guidance.pdf. Claire uses `claire@brt-booking.example` and Stephen uses `stephen@westbury-coach.example`.

## Fairfield staffed desk switch exercise, 24 February 2025

- 10:00 UTC exercise cue → 10:03 planned release, with Claire's wording ready → Hannah sends at 10:10. Claire expected Hannah to go ahead and Hannah was waiting for an explicit release; the seven minutes measure the slip between the scheduled and actual send.
- Coach Yard's 10:13 UTC entry is a phone ring. Fairfield has exact readback times for 11 desks and confirms coverage at all 12 by the 10:22 UTC roll call; the exact final-readback time and elapsed time remain unverified.
- Original 12-desk order (February, still on 12 September and on the printed v0.1): North Hall, South Hall, Station Road, Market Street, Bus Station, High Street, Park Lane, East Gate, West Gate, Harbour Road, Riverside, Coach Yard.
- At 14:00 UTC Daniel, Claire and Hannah agree that Hannah alone calls the desks and follows up, Claire agrees the wording before the cue, and Daniel measures the repeat. Daniel's meeting record and shared-drive PDF are BRT-MT-20250224-001.
- “Fairfield February exercise receipts”, 24 February 11:32 GMT, Hannah → Claire and Daniel, standalone with Fairfield desk receipt record - 24 February.pdf (FTS-DR-20250224-001) attached. Hannah uses `hannah@fairfield-ticket.example`.
- “Fairfield - who had the release at 10:03?” thread, 24 February GMT: (1) 12:08 Claire → Daniel, (2) 12:48 Daniel → Claire, “Re: Fairfield - who had the release at 10:03?”, quoting (1) and attaching February release clock checks.csv.

## Fairfield staffed desk switch repeat and issue, 12–16 September 2025

- “Fairfield Monday cue and readback”, 12 September 11:42 BST, Claire → Hannah and Daniel, standalone. Hannah releases one message to the existing desk group channel on Daniel's cue and calls for individual readbacks in order; each desk listens on the same channel.
- On 15 September at 09:30 BST Hannah marks up Claire's v0.1: Coach Yard's relief is listening earlier, so Coach Yard moves to 11 and Riverside to 12. The first ten stay in order. Hannah changes the Support reminder to “Hannah rings back at 3 min”.
- From the 09:20:00 UTC cue until the last distinct desk gives its name and says “exercise switch received”, allow at most 180 seconds (09:23:00 UTC). North Hall is first at 09:20:18, Coach Yard eleventh at 09:21:50 and Riverside twelfth at 09:22:00. Riverside's 09:21:56 “Riverside here” does not count.
- Daniel records 12 of 12 in 120 seconds, 60 seconds inside the limit, with no fallback call. The call concerns one exercise booking; the live queue continues as usual.
- At 13:00 UTC Daniel, Anjali, Claire and Hannah accept the result and agree the corrected card; Claire issues it, Hannah replaces desk copies, Daniel files the evidence. The decision record and its PDF are BRT-MT-20250915-001.
- BRT-TC-20250915-001 is dated 15 September: v0.1 on the printout and the hand-marked scan, v1.0 on the issued DOCX and its PDF. Claire's notice points to v1.0. Hannah holds the one receipt list and phones only the missing desks after three minutes.
- The card distinguishes “Exercise only: switch to the continuity procedure for the exercise booking. Keep serving live customers as usual.” from the live cue “Switch to the ticket office continuity procedure now. Keep issuing staffed tickets under the local procedure until Fairfield Operations gives the all-clear.” Hannah only gives the live cue after Fairfield Operations authorises the switch. The matching receipt phrases are “exercise switch received” and “switch received”.
- “Fairfield desk card for the folders”, 15 September 15:53 BST, Claire → Hannah, Cc Daniel, standalone with Fairfield staffed desk switch card.pdf and September desk readbacks.csv as separate attachments. The PDF goes at the desks; the CSV stays with the exercise records.

## Fare Payments Q4 availability readout, 14 January 2025

- BRT Fare Payments elapsed-time availability covers 1 October 2024 00:00 to 1 January 2025 00:00 UTC, start inclusive and end exclusive. Oct/Nov/Dec eligible seconds are 2,678,400 / 2,592,000 / 2,678,400; unavailable seconds are 552 / 2,520 / 1,728.
- Two distinct Faregate interruptions contributed 2,520 seconds (42:00) in November and 1,368 (22:48) in December. They were counted once each and total 3,888 seconds, 81% of the 4,800 unavailable seconds; the other interruptions account for 912 seconds (15:12).
- Q4 availability is 99.94% rounded, from 1 − 4,800 / 7,948,800. Take the ratio across the whole window; the 81% uses unavailable seconds as its denominator. The service's 99.9% target is monthly.
- On 14 January at 11:00 UTC Daniel Jennings reports to Anjali Shah with Matthew Pickering present. Anjali prioritises Matthew's previous-route rehearsal by 24 January, then Daniel's checkout watch check by 31 January, deferring broader work. BRT-MT-20250114-001 is the issued readout/report and its unchanged PDF.
- “Q4 Faregate windows for the Tuesday report” thread, GMT: (1) 13 January 14:06 Daniel → Matthew, requests confirmation by 14 January 09:30 UTC; (2) 14 January 08:47 Matthew → Daniel, “Re: Q4 Faregate windows for the Tuesday report”, quotes (1). Neither message has attachments.
- “Q4 Fare Payments report and monthly figures”, 14 January 12:32 GMT, Daniel → Anjali, Cc Matthew, standalone with Fare Payments availability - October to December.pdf and Fare Payments availability Oct to Dec 2024.csv attached.

## Fare Payments change closeout evidence review, 9 July 2025

- Daniel Jennings and Matthew Pickering review 14 checks at 10:00 UTC on 9 July. The retained change material supports 11; the three unlocated completion records concern fallback route confirmation, the first watched checkout outcome and the second engineer's closeout readback. Execution of those three is undetermined.
- BRT-MT-20250709-001 records the review. The historical rollout end stamp is present and remains separate from verification; the two engineers leave the three gaps open. The Bristol board photo is Daniel's at 11:36 BST.
- Daniel's 8 July wording accepts the spoken all-clear. The guidance issued 9 July requires a time and observed result for the fallback/checkout checks and a named, timed second-engineer readback; while anything is outstanding, duty carries a named owner and next check with closeout open.
- “Three gaps for Wednesday”, 8 July 16:02 BST, Matthew → Daniel, standalone with no attachment. “Fare Payments closeout from the next change”, 9 July 15:37 BST, Daniel → Matthew and Carys Morgan, standalone with When a payments change is complete.md attached.

## SVC-17 feature deferral, 7 April 2026

- BRT-EB-20260407-001 covers 1 April 00:00 to 7 April 08:00 UTC, end exclusive. Cumulative Fare Payments allowance at 99.9% is 600 failures on 600,000 eligible attempts; 570 used, 30 remaining (5.0%). This month-to-date attempt-based balance excludes the rest of April.
- Daniel Jennings presents BRT-RB-20260407-001 to Anjali Shah and Matthew Pickering at the 7 April 10:00 UTC review. SVC-17 is the review reference. The four proposals are Checkout saved-card prompt, Receipt resend link, Pending-payment banner and Wallet button placement.
- Anjali's issued BRT-CD-20260407-001 and its unchanged PDF defer all four from April–May production scheduling until a June review on the budget evidence. Preparation outside production can continue.
- At the scheduled 2 June 10:00 UTC checkpoint Daniel is to bring a fresh budget balance and Matthew the queue and evidence plan. A further recorded Operations decision is required for production scheduling; the checkpoint sets no release date.
- “SVC-17 - approved deferral to June review”, 7 April 12:44 BST, Anjali → Daniel and Matthew, Cc Claire Hutchinson, standalone with SVC-17 - April deferral decision.pdf and April budget before SVC-17.xlsx attached. Claire's 8 April notice keeps launch wording and customer training dates open.

## Westbury booking confirmations, 22 July 2024

- Westbury bookings accepted on 17, 18 and 19 July use four half-open 30-minute UTC slots starting at 14:30, 15:00, 15:30 and 16:00. Count each accepted booking once and mark it delayed if still unconfirmed at 120 seconds. The desk reported in BST (UTC+1).
- Delayed counts in slot order are 1/9/8/2, 1/11/9/3 and 2/10/11/2 by day. The two 15:00–16:00 UTC slots together contain 17, 20 and 21 delays respectively. The Desktop Friday preview groups by confirmation time and cannot be read along the acceptance clock.
- At 11:30 UTC on 22 July Daniel Jennings and Matthew Pickering set preparation at 14:30 UTC (15:30 BST), 30 minutes before the 15:00 UTC busy start. Daniel briefs duty and Matthew handles engineering follow-up. BRT-MT-20240722-001 is the issued readback and its unchanged PDF.
- “Westbury bookings slow before the last departures” thread, 22 July BST: (1) 09:04 Stephen Cartledge → Daniel, Cc Claire Hutchinson; (2) 14:02 Daniel → Stephen, Cc Claire, same subject with “Re:”, quoting (1). Neither message has attachments.

## Westbury overdue refunds and booking match, 21 October 2025

- The late cohort covers Westbury refunds still outstanding at their due times on 20–21 October UTC, all due by the 21 October 10:00 UTC cutoff. Of 62 refunds, 62 match their original bookings and zero have no match. At cutoff, 54 have been submitted and eight await submission; actual bank arrival is unverified.
- At the 21 October 12:00 UTC readback Daniel Jennings, Stephen Cartledge and Claire Hutchinson agreed to keep existing bookings. Matthew Pickering checks submissions on 22 October at 12:00 UTC; Claire gives the next update by 14:00 UTC (15:00 BST), including if some are still waiting.
- BRT-CU-20251021-001 and Claire's Support reference (published 21 October 15:05 BST) give an indicative 3–5 UK working days from submission. The desk keeps the original booking and asks Support for a status check.
- “Westbury refunds still showing as due” thread, 21 October BST: (1) 09:08 Stephen → Claire and Daniel; (2) 14:48 Claire → Stephen, Cc Daniel, same subject with “Re:”, quoting (1) and attaching Westbury refund timing and booking advice.pdf.

## Westbury desk enquiry review and Riderpass training allocation, 15 September 2026

- Claire Hutchinson's Westbury desk tally covers full Monday–Sunday UTC weeks of 10–16 August and 7–13 September 2026. The confirmation-delay and refund-progress counts are 11 + 6 = 17 and 4 + 2 = 6. Count newly opened questions once per case and question under the first category; follow-ups on the same question add nothing. Two weeks do not establish cause.
- At 10:30 UTC Daniel Jennings, Claire and Anjali Shah review the figures. Anjali approves moving four team-training hours in October–December to Riderpass. Daniel owns planning, dates stay open, and Westbury preparation and Support guidance stay in use. BRT-MT-20260915-001 is the issued record and unchanged PDF.
- At 13:00 UTC Claire explains the counting rule and refund contact route to Stephen Cartledge. “Westbury September desk figures and guidance”, 15 September 14:34 BST, Claire → Stephen, Cc Daniel, standalone with Westbury September desk update.pdf (BRT-CU-20260915-001) attached.

## Daniel's February appraisal and post-incident report deadlines

- Daniel Jennings and Anjali Shah meet at 10:30 GMT on 23 February 2026. Use Daniel's required first post-incident reports with closures from 1 September 2025 through 19 February 2026 inclusive, from the review register saved 20 February.
- Due means 17:00 Europe/London on the next UK working day, inclusive. Compare the first formal issue timestamp to the due timestamp after converting both to UTC. Count each required report once, by closure date; a draft save never counts as issue.
- Review-register keys RR-201–RR-222 run in closure order. On time/required by cohort: Sep–Oct 4/5, Nov 3/4, Dec 4/4, Jan 4/5, Feb through the 19th 3/4. Overall 18/22 = 81.8%, displayed as 82% at the appraisal.
- RR-218: closure 29 January 15:10 UTC; due 30 January 17:00 UTC; draft 30 January 16:42 UTC; first formal issue 2 February 09:07 UTC. It is January's late entry and the sole mistaken extra success in the scratch count of 19/22.
- RR-221: closure Friday 6 February 16:20 UTC; due Monday 9 February 17:00 UTC; first formal issue Monday 9 February 09:16 UTC. It is on time in the February cohort.
- BRT-MT-20260223-001 is Daniel's agreed objectives record (DOCX and unchanged PDF). The new March–August 2026 goal is at least 95% of his required first reports by the same deadline; first checkpoint with Anjali is 1 June at 10:00 BST, to see March–May results.
- Daniel owns the pre-submit check of closure and due time, evidence/impact wording and actual issue time. When away before the due time, he leaves the deadline and outstanding evidence with a named duty handover recipient and checks that the issue timestamp is captured.
- Standalone emails, GMT: “Monday appraisal - review timings”, Anjali → Daniel, 20 February 09:26; “Objectives and timing extract”, Daniel → Anjali, 23 February 12:11, attaching the agreed objectives PDF and the separate deadline CSV.

## September Westbury travel claim and month-end close

- Daniel's Westbury visit is 21–22 September 2026, with a single night at Henmere House on 21 September. Henmere House accounts issues folio HH-48217 at checkout on 22 September; Daniel saves it on the 23rd.
- HH-48217 lists one room charge of GBP 168 for 21 September and one GBP 168 card settlement on 22 September with reference HHR-8516, leaving no balance. HHR-8516 is the separate merchant card receipt, linked back to HH-48217. Daniel scans only his marked merchant receipt.
- BRT-EC-20260923-001 refers to the original GBP 760 claim and its correction; preserve the six line IDs. L02 (21 September room, GBP 168, HH-48217) stays. L05 (22 September “Room balance”, GBP 168, HHR-8516) is the same stay's payment and is withdrawn in full.
- Unchanged lines: L01 rail return, GBP 218, 21 September, RT-60724; L03 workshop registration, GBP 164, 21 September, WB-REG-134; L04 outbound taxi, GBP 18, 21 September, TX-3953; L06 return taxi, GBP 24, 22 September, TX-4021.
- Daniel and Richard Fenner reconcile the two room lines at 11:00 BST on 23 September. Daniel's BRT-ER-20260923-001 shows GBP 760 originally claimed, GBP 168 withdrawn and GBP 592 retained across five lines, with L05 visible at zero admitted.
- Claim correction thread, 23 September BST: (1) “Two room lines on Westbury”, Richard → Daniel at 09:18, no attachment; (2) “Re: Two room lines on Westbury”, Daniel → Richard at 13:48, quotes (1) and attaches the reconciliation XLSX and the marked merchant receipt PDF.
- “September travel - correction recorded”, Richard → Daniel, 24 September 09:21 BST, standalone; Finance kept the claim reference and marked L05 withdrawn. On 30 September the close extract is as of 10:00 BST.
- Richard issues BRT-ES-20260930-001 at 15:05 BST on 30 September, approving GBP 592 for reimbursement with bank confirmation pending. “September travel close copy”, Richard → Daniel at 15:24 BST, is standalone and attaches that PDF.
