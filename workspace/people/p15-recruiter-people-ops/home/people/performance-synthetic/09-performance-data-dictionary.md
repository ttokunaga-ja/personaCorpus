# Orchid People synthetic performance data dictionary

Cycle: H1 2026  
Review date: 2026-08-18  
Population: 36 fully synthetic pseudonymous records, `SYN-E001` through `SYN-E036`.

These artifacts are for a synthetic People Operations exercise. They contain no names, contacts, actual employee IDs, compensation, health, demographic, protected-trait, or other sensitive personal data.

| Field | Type | Definition | Allowed values / example |
|---|---|---|---|
| `record_id` | text | Stable synthetic record key used to reconcile artifacts. | `SYN-E001` to `SYN-E036` |
| `team_label` | text | Aggregate functional label for calibration discussion. | Product, Engineering, Customer Success, Operations, People, Finance |
| `rating` / `proposed_rating` / `calibrated_rating` | integer | H1 2026 illustrative performance rating. | 1 through 5; 3 means expectations met |
| `primary_goal` / `goal_area` | text | Synthetic goal area for the record. | Delivery reliability, Process improvement |
| `goal_progress_pct` | integer / percentage | Documented synthetic progress against the primary goal. | 55 to 100; displayed as percent in workbooks |
| `status` | text | Goal-progress status derived from progress. | On track (85+), Progressing (65-84), Support needed (<65) |
| `calibration_outcome` | text | Synthetic next-step label after calibration. | Ready, Confirm, Develop, Support |
| `evidence_summary` / `decision_basis` | text | Concise, non-sensitive rationale used in the exercise. | Outcomes met; evidence reviewed |
| `review_date` | ISO date | Date of the synthetic cycle review. | `2026-08-18` |
| `record_count` | integer | Aggregate count for a rating distribution row. | 0 to 36 |
| `share_pct` | decimal / percentage | Aggregate share of the 36-record synthetic population. | 0.0 to 1.0 / percentage format |

## Reconciliation rules

- The population is exactly 36 records in every record-level artifact.
- Rating distribution: 5 = 5 records, 4 = 17, 3 = 11, 2 = 3, 1 = 0.
- Calibration outcomes reconcile to the same groups: Ready = 5, Confirm = 17, Develop = 11, Support = 3.
- Workbooks use formulas for totals, shares, team counts, and averages; their record-data tabs are the source of truth.
- Do not use these synthetic fields for pay, promotion, hiring, retention, or any decision about a real person.
