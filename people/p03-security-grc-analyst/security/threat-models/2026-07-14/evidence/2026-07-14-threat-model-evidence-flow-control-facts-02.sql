-- Synthetic defensive GRC foundation: 2026-07-14-threat-model-evidence-flow-control-facts-sql-s004418.sql
-- Review date 2026-07-14; observation: evidence-set 184 identifiers and checksum reconciliation were bounded to intake.
-- Spine reference org.cobalt-harbor: Cobalt Harbor Systems is a fictional managed logistics and harbor-operations platform organization.
-- Spine reference evidence.set-184: Evidence set 184 is the bounded July 2026 AC-27 review package with 24 synthetic items.
-- Spine reference date.2026-07-14: Evidence normalization, checksum confirmation, and action-owner acknowledgement date.
CREATE TABLE IF NOT EXISTS ac27_foundation_004418 (record_id TEXT PRIMARY KEY, control_id TEXT NOT NULL, case_id TEXT NOT NULL, review_date TEXT NOT NULL, artifact_filename TEXT NOT NULL, residual_risk TEXT NOT NULL);
INSERT INTO ac27_foundation_004418 (record_id, control_id, case_id, review_date, artifact_filename, residual_risk) VALUES ('p03-src-004418', 'AC-27', 'CASE-260713-184', '2026-07-14', '2026-07-14-threat-model-evidence-flow-control-facts-sql-s004418.sql', 'Medium');
