---
path: Documents/Service-reviews/BRT Passenger Identity/2024-10-16/Passenger Identity recovery exercise - October baseline.pdf
kind: tex:brt-recovery-exercise.cls
author: Daniel Jennings
derived_from:
---

# Passenger Identity recovery exercise — October baseline

Reference: BRT-EX-20241018-001  
Issued: 18 Oct 2024  
Prepared by: Daniel Jennings  
To: Emma Wilkinson and Anjali Shah

## Purpose and measurement

On 16 October we rehearsed recovery of BRT Passenger Identity sign-in. Carys Morgan started the clock at the simulated sign-in failure cue at 09:00 UTC and stopped it when she recorded the second of two consecutive successful exercise sign-ins, one minute apart. The target for that entire interval was 20 minutes. Daniel operated in the exercise environment and Emma owned the decision. The result is an exercise observation, not a service incident or availability measure.

## Observed intervals, 16 October 2024

| Interval | Minutes | Observation at end of interval |
|---|---:|---|
| Cue to request | 4 | Request logged at 09:04 UTC. |
| Request to recorded decision | 12 | Emma's approval recorded at 09:16 UTC. |
| Decision to ready for verification | 10 | Restoration ready for verification at 09:26 UTC. |
| Verification | 8 | First successful exercise sign-in at 09:33; second at 09:34 UTC. |
| Total | 34 | Carys stopped at the second successful sign-in, 14 minutes beyond target. |

The 20-minute point fell during restoration. Readiness at 09:26 was not the finish: the verification interval still took eight minutes. At our 13:00 UTC readback Emma, Carys and I identified the 12 minutes between the logged request and recorded approval as the decision bottleneck. An acknowledgement of receipt did not release restoration.

## Follow-up agreed after the exercise

**17 October.** The revised handover calls for transfer after two minutes without a decision on a logged request, or immediately on Emma's explicit handover. Andrew Cartwright is the named delegate, subject to checking his authority. He must confirm scope and acknowledge sole ownership in the exercise record before recording a decision; the operator holds restoration during transfer. If no checked delegate accepts, hold and contact Anjali. The first access check passed 2 of 3: Andrew could read the request and both verification results, but could not record the decision.

**18 October.** After Emma adjusted the exercise-only access, she witnessed the 09:15 UTC recheck with Andrew and me. All three actions passed, including recording a clearly labelled rehearsal decision. Emma published the role explanation following that check. The handover still needs to be demonstrated under the clock in a later rehearsal; it does not change the 16 October observations or the 34-minute baseline above.
