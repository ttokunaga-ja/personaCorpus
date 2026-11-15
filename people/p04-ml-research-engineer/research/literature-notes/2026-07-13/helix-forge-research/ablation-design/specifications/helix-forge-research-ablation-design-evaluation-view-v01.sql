-- Helix Forge Lab synthetic evaluation record: p04-full-005075
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005075 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005075 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 211, 0.878, 0.881, 0.879),
  ('slice-02', 175, 0.895, 0.857, 0.876),
  ('slice-03', 223, 0.877, 0.870, 0.873),
  ('slice-04', 170, 0.894, 0.883, 0.888),
  ('slice-05', 162, 0.876, 0.859, 0.867),
  ('slice-06', 177, 0.893, 0.872, 0.882),
  ('slice-07', 203, 0.875, 0.848, 0.861),
  ('slice-08', 234, 0.892, 0.861, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005075;
ROLLBACK;
