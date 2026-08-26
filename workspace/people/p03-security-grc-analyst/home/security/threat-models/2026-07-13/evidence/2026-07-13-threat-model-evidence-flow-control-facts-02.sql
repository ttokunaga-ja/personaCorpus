-- Synthetic defensive GRC foundation: 2026-07-13-threat-model-evidence-flow-control-facts-sql-s001786.sql
-- Review date 2026-07-13; observation: evidence-set 184 identifiers and checksum reconciliation were bounded to intake.
-- Spine reference org.cobalt-harbor: Cobalt Harbor Systems is a fictional managed logistics and harbor-operations platform organization.
-- Spine reference risk.risk-031: RISK-031 is incomplete quarterly privileged-access review evidence; inherent High and residual Medium after bounded remediation.
-- Spine reference date.2026-07-13: Primary review date; evidence intake opens 08:30 UTC and AC-27 review closes 16:45 UTC.
CREATE TABLE IF NOT EXISTS ac27_foundation_001786 (record_id TEXT PRIMARY KEY, control_id TEXT NOT NULL, case_id TEXT NOT NULL, review_date TEXT NOT NULL, artifact_filename TEXT NOT NULL, residual_risk TEXT NOT NULL);
INSERT INTO ac27_foundation_001786 (record_id, control_id, case_id, review_date, artifact_filename, residual_risk) VALUES ('p03-src-001786', 'AC-27', 'CASE-260713-184', '2026-07-13', '2026-07-13-threat-model-evidence-flow-control-facts-sql-s001786.sql', 'Medium');
