-- Synthetic Cobalt Harbor Systems policy evidence query definition.
-- This statement documents a bounded reporting shape; it performs no privileged action.
SELECT 'p03-full-000647' AS artifact_id,
       'Aegis Control AC-27' AS control_name,
       'CASE-260713-184' AS case_id,
       'RISK-031' AS risk_id,
       'Medium' AS residual_rating,
       184 AS evidence_set,
       24 AS verified_items,
       48 AS reviewed_roles,
       48 AS final_completion,
       18 AS retention_months,
       14 AS record_sequence;
-- Content-spine traceability: org.cobalt-harbor = Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. | decision.dec-260713-004 = Retain evidence set 184 under the synthetic GRC retention schedule; no live-source data is included. | date.2026-07-15 = Control owner validates the remediation plan and retention disposition.
