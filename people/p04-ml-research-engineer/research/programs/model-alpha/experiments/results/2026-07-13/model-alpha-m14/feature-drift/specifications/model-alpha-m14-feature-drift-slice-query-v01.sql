-- Helix Forge Lab synthetic evaluation record: p04-full-005720
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005720 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005720 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 172, 0.894, 0.860, 0.877),
  ('slice-02', 250, 0.876, 0.873, 0.874),
  ('slice-03', 165, 0.893, 0.849, 0.870),
  ('slice-04', 189, 0.875, 0.862, 0.868),
  ('slice-05', 172, 0.892, 0.875, 0.883),
  ('slice-06', 229, 0.874, 0.851, 0.862),
  ('slice-07', 181, 0.891, 0.864, 0.877),
  ('slice-08', 176, 0.873, 0.877, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005720;
ROLLBACK;
