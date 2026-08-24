-- Synthetic review evidence: CR-A7-003768
CREATE TABLE IF NOT EXISTS review_evidence_003768 (
  review_id TEXT PRIMARY KEY,
  reviewed_at TEXT NOT NULL,
  service TEXT NOT NULL,
  disposition TEXT NOT NULL,
  review_elapsed_minutes INTEGER NOT NULL CHECK (review_elapsed_minutes = 400),
  checks_executed INTEGER NOT NULL,
  checks_passed INTEGER NOT NULL
);
INSERT INTO review_evidence_003768 VALUES ('CR-A7-003768', '2026-08-05T10:00:00+09:00', 'svc-event-bus', 'commented', 400, 29, 28);
