-- Synthetic Product Beta R4 architecture evidence
CREATE TABLE IF NOT EXISTS architecture_review_005520 (
  review_id TEXT PRIMARY KEY,
  release_code TEXT NOT NULL CHECK (release_code = 'BETA-R4'),
  review_date TEXT NOT NULL CHECK (review_date = '2026-08-17'),
  subject TEXT NOT NULL,
  design_p95_budget_ms INTEGER NOT NULL CHECK (design_p95_budget_ms = 225),
  status TEXT NOT NULL CHECK (status = 'reviewed')
);
INSERT INTO architecture_review_005520 VALUES ('ADR-B4-148', 'BETA-R4', '2026-08-17', 'event envelope compatibility', 225, 'reviewed');
