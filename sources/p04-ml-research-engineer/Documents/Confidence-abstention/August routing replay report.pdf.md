---
path: Documents/Confidence-abstention/August routing replay report.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# What we can infer from the first queue

Document ID: MDR-CA-20240814-RM-01-R01  
Issued: August 14, 2024  
Author: Vanessa Nguyen  
To: Michael Serrano and Brandon Romero

## Finding

The 0.60 replay sends 144 of 800 labeled development bodies for review (18%). It captures 22 of the 100 errors made before routing (22%). These are different denominators: the first describes the size of the queue, the second the share of errors found in that queue.

| Saved prediction | Review | Automatic | Total |
|---|---:|---:|---:|
| Correct | 122 | 578 | 700 |
| Incorrect | 22 | 78 | 100 |
| Total | 144 | 656 | 800 |

Of the 144 awaiting review, 122 already have a correct saved prediction. The 22 mismatches are opportunities for a person to inspect the result, not 22 resolved cases. Seventy-eight errors remain on the automatic side. We do not know from the routing rule whether a reviewer would find and correct each queued error.

## Where the queue falls

Web form contributes 59 of its 360 scored documents, Shared inbox 58 of 264, and Service desk relay 27 of 176. Although the inbox contributes one fewer queued item than the web form, its own review share is about 22%, the highest of the three. These are shares of the fixed replay population, not forecasts of arrivals at each entrance.

## Limits and next question

This is a replay of 800 historical bodies from development, not live routing. We have not measured actual human processing time, rework, or variation in arrivals over time. The 126 July cases without a supported reference label were outside scoring and must not be added to the 144.

I recommend maintaining current intake while we reconsider the threshold. For any candidate, show both the total sent to review and the errors remaining on the automatic side. Brandon, we need an account of what review entails and its capacity before treating the queue as a manageable workload; the present replay alone cannot supply that answer.

References: What the replay would send for review, August 12; Inbox review load in the August replay, August 13.
