---
path: Documents/Equipment/Calibration/Reader hours July to December.xlsx
kind: bennett-equipment-review.xlsx
author: Melissa Watkins
derived_from:
---

## Sheet: Monthly use

A2: CE-Reader-2 active use — July through December 2025
B4: BCE-CE-2026-TBL-001-R00 | E4: Melissa Watkins
B5: Jan 7, 2026 | E5: CE-Reader-2

| Row | A Period start | B Period end | C Source record | D Active min | E Active hours | F Use category | G Scope note |
|---:|---|---|---|---:|---|---|---|
| 8 | Jul 1, 2025 | Jul 31, 2025 | Jan 5 monthly use export | 4680 | =D8/60 | Active operation | July |
| 9 | Aug 1, 2025 | Aug 31, 2025 | Jan 5 monthly use export | 4800 | =D9/60 | Active operation | August |
| 10 | Sep 1, 2025 | Sep 30, 2025 | Jan 5 monthly use export | 4920 | =D10/60 | Active operation | September |
| 11 | Oct 1, 2025 | Oct 31, 2025 | Jan 5 monthly use export | 5670 | =D11/60 | Active operation | October |
| 12 | Nov 1, 2025 | Nov 30, 2025 | Jan 5 monthly use export | 5850 | =D12/60 | Active operation | November |
| 13 | Dec 1, 2025 | Dec 31, 2025 | Jan 5 monthly use export | 6048 | =D13/60 | Active operation | December |
| 14 | Total active use | | | =SUM(D8:D13) | =D14/60 | | Jul–Dec 2025 |

A15: Source: Lauren's Jan 5, 2026 monthly usage-counter export and BCE-CE-2026-LAB-001-R00 close. Active operating time only; setup, idle reservations and Mason service are excluded. Cutoff Dec 31, 2025.

## Sheet: Period comparison

A2: CE-Reader-2 active use — equal-period comparison
B4: BCE-CE-2026-TBL-001-R00 | E4: Melissa Watkins
B5: Jan 7, 2026 | E5: CE-Reader-2

| Row | A Period start | B Period end | C Calendar days | D Active min | E Active hours | F Change (%) | G Scope |
|---:|---|---|---|---|---|---|---|
| 8 | Jul 1, 2025 | Sep 30, 2025 | =B8-A8+1 | =SUM('Monthly use'!D8:D10) | =D8/60 | | Baseline |
| 9 | Oct 1, 2025 | Dec 31, 2025 | =B9-A9+1 | =SUM('Monthly use'!D11:D13) | =D9/60 | | Comparison |
| 10 | Change vs. baseline | | | =D9-D8 | =E9-E8 | =IF(E8=0,"",E10/E8) | Denominator: Jul 1–Sep 30, 2025 |

A11: Both periods include their start and end dates and contain 92 calendar days. Active operation only; setup, idle reservations and Mason service are excluded. Baseline 14,400 min (240.0 h); comparison 17,568 min (292.8 h); increase 3,168 min (52.8 h), or 22.0% of baseline.
