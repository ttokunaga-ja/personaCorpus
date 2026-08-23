-- Cobalt Harbor: local SQLite queries for fictional GRC review data.
-- Scope: defensive audit preparation; no external database or personal records.
CREATE TABLE IF NOT EXISTS review_evidence (
  evidence_id TEXT PRIMARY KEY,
  risk_id TEXT NOT NULL,
  control_id TEXT NOT NULL,
  review_date TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('verified', 'review'))
);

INSERT OR IGNORE INTO review_evidence (evidence_id, risk_id, control_id, review_date, status)
VALUES
  ('EV-184-ALPHA', 'RISK-031', 'Aegis Control AC-27', '2026-07-13', 'verified'),
  ('EV-184-BRAVO', 'RISK-031', 'Aegis Control AC-27', '2026-07-13', 'review');

-- evidence set 184: identify the small local follow-up set.
SELECT evidence_id, control_id, status
FROM review_evidence
WHERE risk_id = 'RISK-031' AND review_date = '2026-07-13'
ORDER BY evidence_id;
