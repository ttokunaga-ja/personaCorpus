CREATE TABLE IF NOT EXISTS incident_evidence_000549 (
  record_id TEXT PRIMARY KEY, organization TEXT NOT NULL, case_id TEXT NOT NULL, control_id TEXT NOT NULL,
  evidence_set INTEGER NOT NULL, risk_id TEXT NOT NULL, residual_rating TEXT NOT NULL,
  reviewed_roles INTEGER NOT NULL, timely_attestations INTEGER NOT NULL, final_completion INTEGER NOT NULL
);
INSERT INTO incident_evidence_000549 VALUES
  ('IR-000549', 'Cobalt Harbor Systems', 'CASE-260713-184', 'AC-27', 184, 'RISK-031', 'Medium', 48, 47, 48);
-- Content spine references: org.cobalt-harbor = Cobalt Harbor Systems, a fictional managed logistics and harbor-operations platform; case.case-260713-184 = CASE-260713-184, the synthetic evidence intake case; it opens and closes on 2026-07-13; date.2026-07-20 = 2026-07-20, seven-day follow-up for closure evidence and residual-risk status
