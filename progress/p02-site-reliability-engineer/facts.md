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
