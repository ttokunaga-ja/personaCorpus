-- Synthetic defensive governance facts; no operational connection or live data.
CREATE TABLE IF NOT EXISTS detection_review_evidence (
  record_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, control_id TEXT NOT NULL,
  evidence_set INTEGER NOT NULL, reviewed_records INTEGER NOT NULL, timely_records INTEGER NOT NULL,
  completion_records INTEGER NOT NULL, residual_risk TEXT NOT NULL, synthetic_only BOOLEAN NOT NULL
);
INSERT INTO detection_review_evidence VALUES ('p03-full-001981', 'p03-src-001981', 'AC-27', 184, 48, 47, 48, 'Medium', TRUE);
-- review_context: {"decision": "AC-27 control review confirms preparer and reviewer separation; CASE-260713-184 retains RISK-031 at Medium residual risk.", "evidence_reference": "evidence set 184, reconciliation lane 001981", "review_date_or_period": "2026-07-20", "review_topic": "alert coverage", "synthetic_boundary": "defensive governance record only; no live telemetry, credentials, personal data, or routable targets"}
-- content_spine_traceability: {"control.ac-27": "Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition.", "date.2026-07-20": "Seven-day follow-up validates closure evidence and residual-risk status.", "org.cobalt-harbor": "Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform."}
