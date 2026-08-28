-- Helix Forge Lab synthetic evaluation record: p04-full-008324
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008324 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008324 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 190, 0.892, 0.879, 0.885),
  ('slice-02', 212, 0.874, 0.855, 0.864),
  ('slice-03', 241, 0.891, 0.868, 0.879),
  ('slice-04', 179, 0.873, 0.881, 0.877),
  ('slice-05', 167, 0.890, 0.857, 0.873),
  ('slice-06', 179, 0.872, 0.870, 0.871),
  ('slice-07', 204, 0.889, 0.883, 0.886),
  ('slice-08', 235, 0.871, 0.859, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008324;
ROLLBACK;
