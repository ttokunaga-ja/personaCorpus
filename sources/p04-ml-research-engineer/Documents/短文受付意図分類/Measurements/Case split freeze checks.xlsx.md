---
path: Documents/短文受付意図分類/Measurements/Case split freeze checks.xlsx
kind: mercer-adjudication.xlsx
author: Joshua Ramirez
derived_from:
---

# Case split freeze checks

Document ID: MDR-STI-20250403-AW-01-R01  
Issued: April 3, 2025  
Author: Joshua Ramirez  
Scope: 2024 labeled intake candidates and Hold disposition; for Fatima Rahman and Vanessa Nguyen

## Sheet: Adjudication

A2: Case split freeze checks  
B3: MDR-STI-20250403-AW-01-R01  
E3: April 3, 2025  
B4: Joshua Ramirez  
E4: 2024 intake allocation before freeze

| Row | A Ref. | B Check | C Expected | D Count | E Disposition | F Evidence / notes |
|---:|---:|---|---|---:|---|---|
| 6 | 1 | Training / development shared cases | 0 | 0 | Pass | Cross-channel case keys compared across membership lists |
| 7 | 2 | Training / evaluation shared cases | 0 | 0 | Pass | Cross-channel case keys compared across membership lists |
| 8 | 3 | Development / evaluation shared cases | 0 | 0 | Pass | Cross-channel case keys compared across membership lists |
| 9 | 4 | Labeled candidates assigned once | 4,674 | 4674 | Pass | Unique document IDs checked against labeled candidate inventory; no missing or repeated assignment |
| 10 | 5 | Hold outside all partitions | 126 | 126 | Pass | Hold IDs checked against all three membership lists; gold labels remain blank |
| 11 | 6 | Training | Assigned documents | 2804 | Recorded | Case-level allocation |
| 12 | 7 | Development | Assigned documents | 935 | Recorded | Case-level allocation |
| 13 | 8 | Evaluation | Assigned documents | 935 | Recorded | Case-level allocation |
| 14 | | Assigned total | 4,674 | =SUM(D11:D13) | Pass | Compare with row 9; Hold is not included |
| 15 | 9 | Model input | Received body only | | Pass | Input field list checked: no case key or channel field |

## Sheet: Evaluation support

Reference labels from the July guide; every category must have positive support before the evaluation set is fixed.

| Row | A Ref. | B Reference class | C Minimum | D Support | E Disposition | F Evidence / notes |
|---:|---:|---|---:|---:|---|---|
| 6 | 1 | New work request | 1 | 132 | Pass | Frozen evaluation reference labels |
| 7 | 2 | Quote request | 1 | 90 | Pass | Frozen evaluation reference labels |
| 8 | 3 | Schedule check | 1 | 110 | Pass | Frozen evaluation reference labels |
| 9 | 4 | Reschedule | 1 | 85 | Pass | Frozen evaluation reference labels |
| 10 | 5 | Cancel request | 1 | 55 | Pass | Frozen evaluation reference labels |
| 11 | 6 | Job status | 1 | 98 | Pass | Frozen evaluation reference labels |
| 12 | 7 | Send document | 1 | 70 | Pass | Frozen evaluation reference labels |
| 13 | 8 | Request document | 1 | 65 | Pass | Frozen evaluation reference labels |
| 14 | 9 | Correct record | 1 | 52 | Pass | Frozen evaluation reference labels |
| 15 | 10 | Access issue | 1 | 58 | Pass | Frozen evaluation reference labels |
| 16 | 11 | Report defect | 1 | 67 | Pass | Frozen evaluation reference labels |
| 17 | 12 | Invoice question | 1 | 53 | Pass | Frozen evaluation reference labels |
| 18 | | Total | 935 | =SUM(D6:D17) | Pass | Each evaluation document counted once |

Support is checked against reference labels before scoring. The worksheet records allocation checks, not model predictions or performance.
