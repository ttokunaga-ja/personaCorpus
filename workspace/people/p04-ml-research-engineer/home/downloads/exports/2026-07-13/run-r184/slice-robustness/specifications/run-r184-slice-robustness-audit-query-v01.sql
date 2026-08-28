-- Helix Forge Lab synthetic evaluation record: p04-full-006923
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006923 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006923 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 162, 0.889, 0.865, 0.877),
  ('slice-02', 198, 0.871, 0.878, 0.874),
  ('slice-03', 234, 0.888, 0.854, 0.871),
  ('slice-04', 176, 0.870, 0.867, 0.868),
  ('slice-05', 213, 0.887, 0.880, 0.883),
  ('slice-06', 250, 0.869, 0.856, 0.862),
  ('slice-07', 239, 0.886, 0.869, 0.877),
  ('slice-08', 252, 0.868, 0.882, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006923;
ROLLBACK;
