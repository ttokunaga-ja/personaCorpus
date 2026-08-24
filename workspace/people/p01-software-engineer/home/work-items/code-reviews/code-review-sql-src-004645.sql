-- Synthetic review evidence: CR-B4-004645
CREATE TABLE IF NOT EXISTS review_evidence_004645 (
  review_id TEXT PRIMARY KEY,
  reviewed_at TEXT NOT NULL,
  service TEXT NOT NULL,
  disposition TEXT NOT NULL,
  review_elapsed_minutes INTEGER NOT NULL CHECK (review_elapsed_minutes = 400),
  checks_executed INTEGER NOT NULL,
  checks_passed INTEGER NOT NULL
);
INSERT INTO review_evidence_004645 VALUES ('CR-B4-004645', '2026-08-11T10:00:00+09:00', 'svc-edge-api', 'approved with notes', 400, 38, 37);
