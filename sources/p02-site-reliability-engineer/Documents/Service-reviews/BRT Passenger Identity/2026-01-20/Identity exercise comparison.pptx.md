---
path: Documents/Service-reviews/BRT Passenger Identity/2026-01-20/Identity exercise comparison.pptx
kind: brt-reliability-briefing.pptx
author: Daniel Jennings
derived_from:
---

## Slide 1 — Cover

Title: Passenger Identity recovery exercises

Subtitle: October baseline and January repeat — review with Emma Wilkinson and Anjali Shah

Management block:
BRT-RB-20260120-001  
20 Jan 2026 • Daniel Jennings

Speaker notes: For the 14:00 UTC review. These are rehearsals in the exercise environment; neither result is a production incident.

## Slide 2 — Heading and body

Title: One clock, one finish line

- Carys started each clock at the simulated sign-in failure cue at 09:00 UTC.
- She stopped at the second of two consecutive successful exercise sign-ins, one minute apart.
- The common target for the complete interval was 20 minutes.
- October: second success at 09:34. January: second success at 09:18.

Speaker notes: Readiness to verify is not the finish. The first success was at 09:33 in October and 09:17 in January; Carys called the stop after the second. The two records therefore use the same boundaries.

## Slide 3 — Figure

Title: Decision wait in both Identity exercises

Figure: Two horizontal stacked bars on a common elapsed-time axis from 0 to 35 minutes, with a target line at 20 minutes. Legend: cue to request; request to recorded decision; decision to ready for verification; verification.

| Exercise | Cue to request | Request to recorded decision | Decision to ready for verification | Verification | Total |
|---|---:|---:|---:|---:|---:|
| 16 October 2024 | 4 | 12 | 10 | 8 | 34 |
| 20 January 2026 | 3 | 2 | 7 | 6 | 18 |

Label the decision segments 12 and 2 and the bar ends 34 and 18. Axis: elapsed time (minutes).

Figure caption: Same cue-to-second-success boundary; target 20 minutes.

Speaker notes: The January run is 16 minutes shorter overall. Ten minutes of that difference lies in the request-to-recorded-decision interval. The figure compares two exercises; it does not establish a production recovery time.

## Slide 4 — Table

Title: Where the time changed

| Interval | October (min) | January (min) | Reduction (min) |
|---|---:|---:|---:|
| Cue to request | 4 | 3 | 1 |
| Request to recorded decision | 12 | 2 | 10 |
| Decision to ready for verification | 10 | 7 | 3 |
| Verification | 8 | 6 | 2 |
| Total | 34 | 18 | 16 |

Speaker notes: October was 14 minutes over target; January was two minutes under. The other three intervals account for six minutes of the reduction. Familiarity or other differences may contribute; a single scripted repeat does not tell us how much.

## Slide 5 — Two columns

Title: Transfer needs an owner and authority

Left — Decision route:
- Emma owns the decision until transfer.
- After two minutes without a decision on a logged request, or immediately on Emma's explicit handover, the checked delegate can take it.
- Andrew confirms scope and records acceptance of sole ownership before recording the decision. Daniel holds restoration until that decision.
- If no checked delegate accepts, hold and contact Anjali.

Right — Authority and repeat:
- 17 October: 2 of 3 checks passed; Andrew could not write the decision.
- 18 October: exercise-only access corrected; witnessed recheck passed 3 of 3.
- 19 January preflight: Andrew read the request, recorded and read back a practice decision, and saw both results: 3 of 3.
- 20 January: request 09:03; acceptance then decision at 09:05 UTC.

Speaker notes: Emma deliberately left the timed request undecided to exercise the trigger. Andrew's acknowledgement preceded his decision. This run demonstrates that route within the rehearsal; it does not prove the same outcome for future runs or production.

Document reference on every slide: BRT-RB-20260120-001
