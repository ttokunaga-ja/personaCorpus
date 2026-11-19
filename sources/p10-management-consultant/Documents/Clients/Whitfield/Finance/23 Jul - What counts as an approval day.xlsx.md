---
path: Documents/Clients/Whitfield/Finance/23 Jul - What counts as an approval day.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# What counts as an approval day

For Neha Patel · Connor Atkinson · 23 Jul 2025

## Sheet: Endpoints

| Candidate | Event | Use | Reason |
|---|---|---|---|
| Start | Invoice receipt at 01 | No | Receipt precedes matching and confirmed allocation; it measures a different interval. |
| Start | First eligible 06 to 07 | Yes | Invoice, matching evidence and confirmed cost allocation are assembled. Keep the first circulation even if returned. |
| Start | 06 recirculation after return | No | Resetting removes correction time. Record separately. |
| End | 09 Finance approval recorded | Yes | The same recorded decision in each period; 08 checks the 07 decision and exception closure. |
| End | 12 bank release authorisation | No | Includes later Treasury preparation and checks. |

## Sheet: Cohorts and calculation

| Item | Definition |
|---|---|
| Population | Standard domestic supplier payments for Components and Packaging; exclude payroll, tax and emergency manual transfers. |
| Baseline | First eligible 06 date in Japan: 21 Jul–17 Aug 2025 inclusive. |
| Pilot | First eligible 06 date in Japan: 18 Aug–14 Sep 2025 inclusive; the opening week changes only the 07 side. |
| Elapsed days | (09 approval timestamp minus first eligible 06 timestamp) / 24 hours; include nights, weekends and returns. |
| Candidate without time | A verified 06 date places a record in a cohort, but without both endpoint times leave elapsed days blank and exclude from the mean. |
| Return | Record reason and recirculation separately; never overwrite the initial 06. |
| Comparison | Use identical endpoints and units for both windows. Report missing times and outstanding approvals separately. |

Keep the Japanese timestamps and their time zone. The definition does not authorise a 09 route change.
