-- Helix Forge Lab synthetic evaluation record: p04-full-007576
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007576 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007576 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 228, 0.875, 0.855, 0.865),
  ('slice-02', 183, 0.892, 0.868, 0.880),
  ('slice-03', 179, 0.874, 0.881, 0.877),
  ('slice-04', 243, 0.891, 0.857, 0.874),
  ('slice-05', 246, 0.873, 0.870, 0.871),
  ('slice-06', 219, 0.890, 0.883, 0.886),
  ('slice-07', 176, 0.872, 0.859, 0.865),
  ('slice-08', 221, 0.889, 0.872, 0.880);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007576;
ROLLBACK;
