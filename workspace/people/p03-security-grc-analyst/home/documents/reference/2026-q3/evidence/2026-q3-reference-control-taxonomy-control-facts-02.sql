-- Synthetic, offline AC-27 evidence reference: 2026-q3-reference-control-taxonomy-control-facts-sql-s005316 / AC27-RF-gvl.
-- org.cobalt-harbor: Cobalt Harbor Systems is a fictional managed logistics and harbor-operations platform organization.
-- control.ac-27: Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition.
-- period.2026-q3: Governance planning period from 2026-07-01 through 2026-09-30.
CREATE TABLE IF NOT EXISTS ac27_evidence_reference (organization TEXT, review_date TEXT, evidence_set INTEGER, control_id TEXT, case_id TEXT, risk_id TEXT, reviewed INTEGER, timely INTEGER, final_completion INTEGER, residual_risk TEXT);
INSERT INTO ac27_evidence_reference VALUES ('Cobalt Harbor Systems', '2026-07-13', 184, 'AC-27', 'CASE-260713-184', 'RISK-031', 48, 47, 48, 'Medium');
