-- Helix Forge Lab synthetic evaluation record: p04-full-004575
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004575 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004575 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 170, 0.892, 0.859, 0.875),
  ('slice-02', 249, 0.874, 0.872, 0.873),
  ('slice-03', 165, 0.891, 0.848, 0.869),
  ('slice-04', 189, 0.873, 0.861, 0.867),
  ('slice-05', 219, 0.890, 0.874, 0.882),
  ('slice-06', 205, 0.872, 0.850, 0.861),
  ('slice-07', 169, 0.889, 0.863, 0.876),
  ('slice-08', 170, 0.871, 0.876, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004575;
ROLLBACK;
