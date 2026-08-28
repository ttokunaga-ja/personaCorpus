-- Helix Forge Lab synthetic evaluation record: p04-full-005679
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005679 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005679 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 184, 0.891, 0.850, 0.870),
  ('slice-02', 209, 0.873, 0.863, 0.868),
  ('slice-03', 240, 0.890, 0.876, 0.883),
  ('slice-04', 179, 0.872, 0.852, 0.862),
  ('slice-05', 167, 0.889, 0.865, 0.877),
  ('slice-06', 227, 0.871, 0.878, 0.874),
  ('slice-07', 180, 0.888, 0.854, 0.871),
  ('slice-08', 175, 0.870, 0.867, 0.868);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005679;
ROLLBACK;
