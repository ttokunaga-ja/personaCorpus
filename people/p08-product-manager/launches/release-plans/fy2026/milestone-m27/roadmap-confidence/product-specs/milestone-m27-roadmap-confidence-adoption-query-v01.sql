-- Juniper Works synthetic evaluation record: p08-full-000904
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000904 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000904 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 217, 0.894, 0.849, 0.871),
  ('slice-02', 225, 0.876, 0.862, 0.869),
  ('slice-03', 200, 0.893, 0.875, 0.884),
  ('slice-04', 206, 0.875, 0.851, 0.863),
  ('slice-05', 228, 0.892, 0.864, 0.878),
  ('slice-06', 210, 0.874, 0.877, 0.875),
  ('slice-07', 172, 0.891, 0.853, 0.872),
  ('slice-08', 171, 0.873, 0.866, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000904;
ROLLBACK;
