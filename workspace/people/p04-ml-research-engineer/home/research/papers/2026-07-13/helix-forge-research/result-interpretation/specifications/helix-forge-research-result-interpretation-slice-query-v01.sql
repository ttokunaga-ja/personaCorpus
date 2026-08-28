-- Helix Forge Lab synthetic evaluation record: p04-full-006376
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006376 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006376 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 181, 0.863, 0.856, 0.859),
  ('slice-02', 160, 0.880, 0.869, 0.874),
  ('slice-03', 168, 0.862, 0.882, 0.872),
  ('slice-04', 238, 0.879, 0.858, 0.868),
  ('slice-05', 196, 0.896, 0.871, 0.883),
  ('slice-06', 241, 0.878, 0.847, 0.862),
  ('slice-07', 187, 0.895, 0.860, 0.877),
  ('slice-08', 226, 0.877, 0.873, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006376;
ROLLBACK;
