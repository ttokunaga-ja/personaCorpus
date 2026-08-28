-- Helix Forge Lab synthetic evaluation record: p04-full-007777
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007777 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007777 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 224, 0.891, 0.856, 0.873),
  ('slice-02', 229, 0.873, 0.869, 0.871),
  ('slice-03', 202, 0.890, 0.882, 0.886),
  ('slice-04', 160, 0.872, 0.858, 0.865),
  ('slice-05', 205, 0.889, 0.871, 0.880),
  ('slice-06', 246, 0.871, 0.847, 0.859),
  ('slice-07', 190, 0.888, 0.860, 0.874),
  ('slice-08', 228, 0.870, 0.873, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007777;
ROLLBACK;
