-- Helix Forge Lab synthetic evaluation record: p04-full-008124
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008124 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008124 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 248, 0.895, 0.870, 0.882),
  ('slice-02', 193, 0.877, 0.883, 0.880),
  ('slice-03', 184, 0.894, 0.859, 0.876),
  ('slice-04', 198, 0.876, 0.872, 0.874),
  ('slice-05', 176, 0.893, 0.848, 0.870),
  ('slice-06', 231, 0.875, 0.861, 0.868),
  ('slice-07', 230, 0.892, 0.874, 0.883),
  ('slice-08', 200, 0.874, 0.850, 0.862);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008124;
ROLLBACK;
