---
path: Documents/長文点検記録分類/Cost-discussions/Aug 11 monthly cost review.docx
kind: mercer-lab-record.docx
author: Vanessa Nguyen
derived_from:
---
# August 11 monthly cost review

Document ID: MDR-LDI-20260811-LR-01-R01  
Issued: August 11, 2026  
Author: Vanessa Nguyen

Meeting: August 11, 2026, 2:00 p.m. PDT  
Present: Vanessa Nguyen, Brandon Romero

## Comparison reviewed

Brandon and I put both conditions on 100,000 original records per month. The second inference is included in resource use and does not increase the count of received records. At this common volume, the opening-only estimate is USD 3,600 per month and the second-read estimate is USD 4,800, including USD 600 monthly standby monitoring in each. The application ceiling is USD 4,500; the proposal exceeds it by USD 300.

| Monthly cost, USD | Opening only | Second read | Increase |
|---|---:|---:|---:|
| Input preparation | 400 | 400 | 0 |
| Host | 600 | 800 | 200 |
| Accelerator | 2,000 | 3,000 | 1,000 |
| Standby monitoring | 600 | 600 | 0 |
| Total | 3,600 | 4,800 | 1,200 |

The accelerator accounts for USD 1,000 of the increase and the host for USD 200. Replacing only the accelerator amount in the opening-only estimate gives USD 4,600, USD 100 above the ceiling. The estimates scale the timed runs; they do not establish peak-time performance or the cost of unmeasured recovery.

## Research result and use

On the independent evaluation of 4,000 original records, misses among the 1,000 action-required records fell from 32 (3.2%) to 24 (2.4%). Fourteen opening-only misses were recovered and six records became new misses. False alarms among 3,000 non-action records rose from 90 to 103. We will retain this as a research comparison, hold adoption for the application, and continue the current human checking.

## Follow-up

| Owner | Work | Due |
|---|---|---|
| Brandon | Summarize the questions and measurements needed for uneven and peak arrivals. | August 25, 2026 |
| Vanessa | Summarize the review and recovery path and conditions for additional evaluation. | August 25, 2026 |

The monthly estimate does not resolve the response time at peak arrival or the person and effort needed to recover a misrouted record. These remain open for the next review.
