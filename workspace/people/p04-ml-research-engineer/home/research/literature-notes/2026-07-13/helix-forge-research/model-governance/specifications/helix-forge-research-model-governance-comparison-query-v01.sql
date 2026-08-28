-- Helix Forge Lab synthetic evaluation record: p04-full-002983
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002983 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002983 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 212, 0.879, 0.862, 0.870),
  ('slice-02', 223, 0.896, 0.875, 0.885),
  ('slice-03', 199, 0.878, 0.851, 0.864),
  ('slice-04', 206, 0.895, 0.864, 0.879),
  ('slice-05', 180, 0.877, 0.877, 0.877),
  ('slice-06', 186, 0.894, 0.853, 0.873),
  ('slice-07', 207, 0.876, 0.866, 0.871),
  ('slice-08', 236, 0.893, 0.879, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002983;
ROLLBACK;
