---
path: Documents/Clients/Whitfield/Finance/12 Feb - Japan duty windows.xlsx
kind: holt-whitfield-finance.xlsx
author: Siobhan Barrett
derived_from:
---

# Sheet: Regular payments

A2: Whitfield | Regular payment duties for the March options  
A4: HRA-WHI-DIAG-PLN-2025-002 v0.1  
D4: 12 Feb 2025  
F4: Siobhan Barrett

| Row | A Date | B Start JST | C End JST | D Duty type | E Display name | F Work | G Schedule status | H Clash to raise with Kenneth |
|---|---|---|---|---|---|---|---|---|
| 7 | 17 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | |
| 8 | 17 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | |
| 9 | 18 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | |
| 10 | 18 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | |
| 11 | 19 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | Account reconciliation overlaps 15:00–16:30 |
| 12 | 19 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | Check who is listed as Close Reviewer B |
| 13 | 24 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | |
| 14 | 24 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | |
| 15 | 25 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | |
| 16 | 25 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | |
| 17 | 26 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-P | 07 domestic supplier payments | Scheduled | Variance review overlaps 15:00–16:30 |
| 18 | 26 Mar 2025 | 15:00 | 16:30 | Regular payments | JP07-D | 07 domestic supplier payments | Scheduled | Check who is listed as Close Reviewer B |

Source for both sheets: Japan Finance March duty slots.csv, supplied by Kenneth Townsend on 10 February. Column A contains dates and B:C contain local Excel time values in JST. The display names and schedule status are as supplied.

For the call with Kenneth: check whether the two assignments against JP07-P have to be worked in full and whether the close reviewer already appears on this sheet. Ask separately for confirmation of 07 authority.

# Sheet: Close preparation

A2: Whitfield | Close preparation and 07 clashes  
A4: HRA-WHI-DIAG-PLN-2025-002 v0.1  
D4: 12 Feb 2025  
F4: Siobhan Barrett

| Row | A Date | B Start JST | C End JST | D Duty type | E Display name | F Work and status | G Regular duty match | H 07 overlap (min) |
|---|---|---|---|---|---|---|---|---|
| 7 | 19 Mar 2025 | 14:30 | 16:30 | Close preparation | JP07-P | Account reconciliation · Scheduled | JP07-P (same label) | `=ROUND(MAX(0,MIN(C7,'Regular payments'!C11)-MAX(B7,'Regular payments'!B11))*1440,0)` |
| 8 | 19 Mar 2025 | 14:30 | 16:30 | Close preparation | Close Reviewer B | Account reconciliation · Scheduled | Unmatched — ask Kenneth | |
| 9 | 26 Mar 2025 | 15:00 | 16:45 | Close preparation | JP07-P | Month-end variance review · Scheduled | JP07-P (same label) | `=ROUND(MAX(0,MIN(C9,'Regular payments'!C17)-MAX(B9,'Regular payments'!B17))*1440,0)` |
| 10 | 26 Mar 2025 | 15:00 | 16:45 | Close preparation | Close Reviewer B | Month-end variance review · Scheduled | Unmatched — ask Kenneth | |

Columns A–C use the same date and JST time values as the regular sheet. The same-label rows each show 90 minutes of overlap. The cells for Close Reviewer B stay blank until Kenneth identifies the regular-duty counterpart, if there is one. Keep the two rotas for the enquiry before drawing any conclusion about available cover.
