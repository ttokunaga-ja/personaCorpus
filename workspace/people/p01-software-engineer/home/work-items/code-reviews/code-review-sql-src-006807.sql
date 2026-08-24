-- Synthetic review evidence: CR-B4-006807
CREATE TABLE IF NOT EXISTS review_evidence_006807 (
  review_id TEXT PRIMARY KEY,
  reviewed_at TEXT NOT NULL,
  service TEXT NOT NULL,
  disposition TEXT NOT NULL,
  review_elapsed_minutes INTEGER NOT NULL CHECK (review_elapsed_minutes = 400),
  checks_executed INTEGER NOT NULL,
  checks_passed INTEGER NOT NULL
);
INSERT INTO review_evidence_006807 VALUES ('CR-B4-006807', '2026-08-29T10:00:00+09:00', 'svc-decision-store', 'approved', 400, 30, 29);
