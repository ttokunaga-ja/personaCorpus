-- Helix Forge Lab synthetic evaluation record: p04-full-009226
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009226 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009226 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 163, 0.890, 0.864, 0.877),
  ('slice-02', 246, 0.872, 0.877, 0.874),
  ('slice-03', 163, 0.889, 0.853, 0.871),
  ('slice-04', 235, 0.871, 0.866, 0.868),
  ('slice-05', 195, 0.888, 0.879, 0.883),
  ('slice-06', 193, 0.870, 0.855, 0.862),
  ('slice-07', 211, 0.887, 0.868, 0.877),
  ('slice-08', 191, 0.869, 0.881, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009226;
ROLLBACK;
