CREATE TABLE IF NOT EXISTS incident_evidence_001715 (
  record_id TEXT PRIMARY KEY, organization TEXT NOT NULL, case_id TEXT NOT NULL, control_id TEXT NOT NULL,
  evidence_set INTEGER NOT NULL, risk_id TEXT NOT NULL, residual_rating TEXT NOT NULL,
  reviewed_roles INTEGER NOT NULL, timely_attestations INTEGER NOT NULL, final_completion INTEGER NOT NULL
);
INSERT INTO incident_evidence_001715 VALUES
  ('IR-001715', 'Cobalt Harbor Systems', 'CASE-260713-184', 'AC-27', 184, 'RISK-031', 'Medium', 48, 47, 48);
-- Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; date.2026-07-13 = 2026-07-13, primary review date; evidence intake 08:30 UTC and AC-27 review close 16:45 UTC
