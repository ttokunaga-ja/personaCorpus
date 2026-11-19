---
path: Documents/Clients/Pioneer/Buying/15 Jul - Coverage and urgent breakdown.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

## Sheet: Coverage

A1: Pioneer — supplier coverage  
A2: Period: invoices and credits posted 01 Jan–31 Mar 2025; denominator: 42 suppliers grouped by supplier ID. Net purchases exclude tax.  
A4: Group | B4: Suppliers
A5: Selected for diagnosis | B5: 12
A6: Remaining in population | B6: 30
A7: Total population | B7: =SUM(B5:B6)
A9: Source: HRA-PIO-BUY-SEL-2025-001 v1.0. Top 12 by net USD purchases; ties by supplier ID ascending. Remaining 30 stay in the population.
A10: For individual supplier detail, contact Nicholas Hargreaves.

## Sheet: Requests

A1: Pioneer — exception requests by priority  
A2: Period: requests received 01 Apr–30 Jun 2025 across all suppliers; denominator: 93 unique request IDs. Resends are not additional requests.  
A4: Priority at submission | B4: Requests
A5: Urgent | B5: 37
A6: Other | B6: 56
A7: Total | B7: =SUM(B5:B6)
A9: Source: HRA-PIO-BUY-EXC-2025-001 v1.0. Contact Nicholas Hargreaves for individual requests.

## Sheet: Urgent reasons

A1: Pioneer — urgent requests after evidence check  
A2: Period: requests received 01 Apr–30 Jun 2025; denominator: 37 unique requests marked Urgent at submission, across all suppliers.  
A4: Evidence-based category | B4: Requests | C4: Share of urgent requests
A5: Known before cut-off, shared late | B5: 23 | C5: =B5/$B$9
A6: New or increased demand after cut-off | B6: 8 | C6: =B6/$B$9
A7: Supply-side change | B7: 4 | C7: =B7/$B$9
A8: Insufficient evidence | B8: 2 | C8: =B8/$B$9
A9: Total urgent | B9: =SUM(B5:B8) | C9: =SUM(C5:C8)
A11: Source: HRA-PIO-BUY-EXC-2025-001 v1.0. Classification uses the first evidence of quantity and required date, Buying's first receipt and the preceding-business-day 15:00 EDT cut-off. The two evidence gaps are not zero-delay cases.
A12: These counts describe the 37 urgent requests, not all 93. They do not measure avoided exceptions. For individual detail contact Nicholas Hargreaves.
