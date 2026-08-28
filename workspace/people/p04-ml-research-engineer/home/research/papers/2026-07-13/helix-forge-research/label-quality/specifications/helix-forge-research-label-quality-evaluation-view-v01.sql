-- Helix Forge Lab synthetic evaluation record: p04-full-008276
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008276 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008276 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 213, 0.895, 0.875, 0.885),
  ('slice-02', 176, 0.877, 0.851, 0.864),
  ('slice-03', 176, 0.894, 0.864, 0.879),
  ('slice-04', 194, 0.876, 0.877, 0.876),
  ('slice-05', 222, 0.893, 0.853, 0.873),
  ('slice-06', 207, 0.875, 0.866, 0.870),
  ('slice-07', 170, 0.892, 0.879, 0.885),
  ('slice-08', 170, 0.874, 0.855, 0.864);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008276;
ROLLBACK;
