-- Helix Forge Lab synthetic evaluation record: p04-full-005375
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005375 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005375 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 188, 0.875, 0.879, 0.877),
  ('slice-02', 211, 0.892, 0.855, 0.873),
  ('slice-03', 193, 0.874, 0.868, 0.871),
  ('slice-04', 250, 0.891, 0.881, 0.886),
  ('slice-05', 202, 0.873, 0.857, 0.865),
  ('slice-06', 197, 0.890, 0.870, 0.880),
  ('slice-07', 213, 0.872, 0.883, 0.877),
  ('slice-08', 239, 0.889, 0.859, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005375;
ROLLBACK;
