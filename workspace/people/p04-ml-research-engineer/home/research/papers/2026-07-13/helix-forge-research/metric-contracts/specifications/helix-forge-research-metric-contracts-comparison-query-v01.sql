-- Helix Forge Lab synthetic evaluation record: p04-full-005276
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005276 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005276 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 223, 0.870, 0.858, 0.864),
  ('slice-02', 181, 0.887, 0.871, 0.879),
  ('slice-03', 226, 0.869, 0.847, 0.858),
  ('slice-04', 219, 0.886, 0.860, 0.873),
  ('slice-05', 187, 0.868, 0.873, 0.870),
  ('slice-06', 189, 0.885, 0.849, 0.867),
  ('slice-07', 161, 0.867, 0.862, 0.864),
  ('slice-08', 166, 0.884, 0.875, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005276;
ROLLBACK;
