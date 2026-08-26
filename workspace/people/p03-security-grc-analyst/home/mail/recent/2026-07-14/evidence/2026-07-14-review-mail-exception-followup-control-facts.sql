-- Content spine traceability: org.cobalt-harbor = Cobalt Harbor Systems | audit.ar-260713-027 = AR-260713-027 is the fictional AC-27 audit request for review material | date.2026-07-14 = 2026-07-14 is evidence normalization, checksum confirmation, and action-owner acknowledgement
-- CASE timeline: intake 2026-07-13; normalization 2026-07-14; remediation validation 2026-07-15; follow-up closure 2026-07-20.
-- synthetic defensive governance record
-- MR-021: exception follow-up; 2026-07-14
CREATE TABLE IF NOT EXISTS ac27_mail_review_fact (
  record_id TEXT PRIMARY KEY, review_date TEXT NOT NULL, control_id TEXT NOT NULL,
  case_id TEXT NOT NULL, risk_id TEXT NOT NULL, evidence_set INTEGER NOT NULL,
  timely_attestations INTEGER NOT NULL, final_attestations INTEGER NOT NULL, residual_risk TEXT NOT NULL
);
INSERT INTO ac27_mail_review_fact VALUES ('MR-021', '2026-07-14', 'AC-27', 'CASE-260713-184', 'RISK-031', 184, 47, 48, 'Medium');
