-- Synthetic defensive GRC foundation: 2026-07-13-threat-model-control-boundary-control-facts-sql-s001085.sql
-- Review date 2026-07-13; observation: AC-27 role separation and retention boundary were documented without operational access.
-- Spine reference org.cobalt-harbor: Cobalt Harbor Systems is a fictional managed logistics and harbor-operations platform organization.
-- Spine reference evidence.set-184: Evidence set 184 is the bounded July 2026 AC-27 review package with 24 synthetic items.
-- Spine reference date.2026-07-13: Primary review date; evidence intake opens 08:30 UTC and AC-27 review closes 16:45 UTC.
CREATE TABLE IF NOT EXISTS ac27_foundation_001085 (record_id TEXT PRIMARY KEY, control_id TEXT NOT NULL, case_id TEXT NOT NULL, review_date TEXT NOT NULL, artifact_filename TEXT NOT NULL, residual_risk TEXT NOT NULL);
INSERT INTO ac27_foundation_001085 (record_id, control_id, case_id, review_date, artifact_filename, residual_risk) VALUES ('p03-src-001085', 'AC-27', 'CASE-260713-184', '2026-07-13', '2026-07-13-threat-model-control-boundary-control-facts-sql-s001085.sql', 'Medium');
