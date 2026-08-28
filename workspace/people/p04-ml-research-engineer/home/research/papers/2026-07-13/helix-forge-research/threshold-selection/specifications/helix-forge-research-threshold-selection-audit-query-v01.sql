-- Helix Forge Lab synthetic evaluation record: p04-full-006027
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006027 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006027 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 189, 0.896, 0.869, 0.882),
  ('slice-02', 164, 0.878, 0.882, 0.880),
  ('slice-03', 170, 0.895, 0.858, 0.876),
  ('slice-04', 239, 0.877, 0.871, 0.874),
  ('slice-05', 197, 0.894, 0.847, 0.870),
  ('slice-06', 194, 0.876, 0.860, 0.868),
  ('slice-07', 211, 0.893, 0.873, 0.883),
  ('slice-08', 191, 0.875, 0.849, 0.862);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006027;
ROLLBACK;
