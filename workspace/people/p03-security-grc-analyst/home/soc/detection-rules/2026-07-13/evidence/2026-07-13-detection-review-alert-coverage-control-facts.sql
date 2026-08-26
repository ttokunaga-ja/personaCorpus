-- Synthetic defensive governance facts; no operational connection or live data.
CREATE TABLE IF NOT EXISTS detection_review_evidence (
  record_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, control_id TEXT NOT NULL,
  evidence_set INTEGER NOT NULL, reviewed_records INTEGER NOT NULL, timely_records INTEGER NOT NULL,
  completion_records INTEGER NOT NULL, residual_risk TEXT NOT NULL, synthetic_only BOOLEAN NOT NULL
);
INSERT INTO detection_review_evidence VALUES ('p03-full-006585', 'p03-src-006585', 'AC-27', 184, 48, 47, 48, 'Medium', TRUE);
-- review_context: {"decision": "SOC duty review confirms the minimized Sentinel Export boundary; CASE-260713-184 retains RISK-031 at Medium residual risk.", "evidence_reference": "evidence set 184, reconciliation lane 006585", "review_date_or_period": "2026-07-13", "review_topic": "alert coverage", "synthetic_boundary": "defensive governance record only; no live telemetry, credentials, personal data, or routable targets"}
-- content_spine_traceability: {"date.2026-07-13": "Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC.", "org.cobalt-harbor": "Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.", "team.soc": "Security Operations owns bounded SIEM exports, detection review evidence, and incident evidence-chain handoff."}
