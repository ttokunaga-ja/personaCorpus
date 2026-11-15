-- Synthetic, offline AC-27 evidence reference: 2026-07-20-reference-control-taxonomy-control-facts-sql-s001216 / AC27-RF-att.
-- org.cobalt-harbor: Cobalt Harbor Systems is a fictional managed logistics and harbor-operations platform organization.
-- date.2026-07-20: Seven-day follow-up validates closure evidence and residual-risk status.
CREATE TABLE IF NOT EXISTS ac27_evidence_reference (organization TEXT, review_date TEXT, evidence_set INTEGER, control_id TEXT, case_id TEXT, risk_id TEXT, reviewed INTEGER, timely INTEGER, final_completion INTEGER, residual_risk TEXT);
INSERT INTO ac27_evidence_reference VALUES ('Cobalt Harbor Systems', '2026-07-13', 184, 'AC-27', 'CASE-260713-184', 'RISK-031', 48, 47, 48, 'Medium');
