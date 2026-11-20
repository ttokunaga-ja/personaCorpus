---
path: Documents/Stories/Minutes-access/2025-08-06 - Response time workbook.xlsx
kind: acn-source-check.xlsx
author: Leslie Crawford
derived_from:
---

## Sheet: Response dates

A1: three-square masthead; B1: Allegheny Civic News Cooperative  
A2: Minutes access | Response dates  
C3: ACN-INV-20250806-LOG-001; F3: Leslie Crawford; C4: 2025-08-06 (Excel date); Version: v1.0

Row 6: No. | Request | Intake source | Response source and result | Received | First response | Calendar days | Check

Rows 7–20 below are A through F and H; dates in E and F are Excel dates. In each row G is `=IF(AND(ISNUMBER(E7),ISNUMBER(F7)),F7-E7,"")`, with the row number adjusted to that row.

| Row | A | B | C | D | E | F | H |
|---|---:|---|---|---|---|---|---|
| 7 | 1 | MIN-25-101 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-05 | 2025-05-20 | Matched by request |
| 8 | 2 | MIN-25-102 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-08 | 2025-06-16 | Matched by request |
| 9 | 3 | MIN-25-103 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-12 | 2025-05-21 | Matched by request |
| 10 | 4 | MIN-25-104 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-13 | 2025-06-13 | Matched by request |
| 11 | 5 | MIN-25-105 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Status only | 2025-05-14 | 2025-06-27 | No minutes supplied |
| 12 | 6 | MIN-25-106 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-15 | 2025-05-27 | May 19 resubmission, same request |
| 13 | 7 | MIN-25-107 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; MFB-PRO-20250610-002 p.1; Status only | 2025-05-16 | 2025-06-10 | Unapproved draft; no copy enclosed |
| 14 | 8 | MIN-25-108 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-19 | 2025-07-09 | Matched by request |
| 15 | 9 | MIN-25-109 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-20 | 2025-06-09 | Matched by request |
| 16 | 10 | MIN-25-110 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; MFB-PRO-20250617-001 p.1; Status only | 2025-05-21 | 2025-06-17 | Clerk asked about location; no copy |
| 17 | 11 | MIN-25-111 | MFB-PRO-20250804-001 p.1 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-05-22 | 2025-05-28 | Matched by request |
| 18 | 12 | MIN-25-112 | MFB-PRO-20250804-001 p.2 | MFB-PRO-20250804-002 p.1; Status only | 2025-06-16 | 2025-07-15 | No minutes supplied |
| 19 | 13 | MIN-25-113 | MFB-PRO-20250804-001 p.2 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-06-17 | 2025-07-10 | Matched by request |
| 20 | 14 | MIN-25-114 | MFB-PRO-20250804-001 p.2 | MFB-PRO-20250804-002 p.1; Minutes supplied | 2025-06-18 | 2025-07-23 | Matched by request |

Summary cells: A22 “Responses counted”, B22 `=COUNT(G7:G20)` (14); A23 “Seventh”, B23 `=SMALL(G7:G20,7)` (25); A24 “Eighth”, B24 `=SMALL(G7:G20,8)` (27); A25 “Average of middle two”, B25 `=AVERAGE(B23:B24)` (26); A26 “Median”, B26 `=MEDIAN(G7:G20)` (26). A27: “Middle requests: MIN-25-107 (25 days) and MIN-25-110 (27 days). Both are status notices, not copies of minutes. Receipt is day zero; acknowledgment alone is not a substantive response.”
