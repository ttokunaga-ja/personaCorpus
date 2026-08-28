-- Helix Forge Lab synthetic evaluation record: p04-full-006824
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006824 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006824 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 198, 0.875, 0.853, 0.864),
  ('slice-02', 168, 0.892, 0.866, 0.879),
  ('slice-03', 172, 0.874, 0.879, 0.876),
  ('slice-04', 240, 0.891, 0.855, 0.873),
  ('slice-05', 245, 0.873, 0.868, 0.870),
  ('slice-06', 218, 0.890, 0.881, 0.885),
  ('slice-07', 223, 0.872, 0.857, 0.864),
  ('slice-08', 244, 0.889, 0.870, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006824;
ROLLBACK;
