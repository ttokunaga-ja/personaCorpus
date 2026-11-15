# Orchid People ATS requisition export field map

**Export timestamp:** `2026-08-18T16:30:00+09:00`  
**Data file:** `00-ats-requisition-export.csv`  
**Schema:** `orchid_people.ats_requisition_opening.v1`  
**Data rows:** 5 (one row per synthetic requisition opening)

| Column | Type | Meaning | Privacy treatment |
|---|---|---|---|
| `export_schema_version` | string | Fixed schema identifier. | Non-personal metadata. |
| `exported_at_jst` | ISO 8601 timestamp | Time this export package was produced in JST. | Non-personal metadata. |
| `organization` | string | Synthetic organization label: Orchid People. | Synthetic organization data. |
| `requisition_id` | string | Synthetic requisition identifier. | Synthetic ID only. |
| `requisition_title` | string | Requisition role title. | Role-level metadata. |
| `opening_ordinal` | integer | Position of the row within a requisition's openings. | Non-personal metadata. |
| `opening_count` | integer | Authorized openings for that requisition. | Aggregate headcount metadata. |
| `workflow_state` | string | Requisition-level workflow status. | Non-personal metadata. |
| `candidate_reference_id` | string | Synthetic workflow reference, not a candidate identity. | Synthetic ID only. |
| `candidate_stage` | string | Workflow stage associated with the synthetic reference. | No resumes or evaluation text. |
| `compensation_currency` | string | Currency for the published requisition band. | Not individual pay. |
| `compensation_band_min_jpy` | integer | Lower bound of the published role band. | Requisition-level band only. |
| `compensation_band_max_jpy` | integer | Upper bound of the published role band. | Requisition-level band only. |
| `pii_redaction_status` | string | Fixed confirmation that each row contains synthetic references only. | Privacy control marker. |

The audit presentation in `01-offer-audit-export.html` renders the same five opening-level rows. The execution record is `02-export-job-2026-08-18.log`; handling requirements are in `05-export-privacy-note.markdown`.
