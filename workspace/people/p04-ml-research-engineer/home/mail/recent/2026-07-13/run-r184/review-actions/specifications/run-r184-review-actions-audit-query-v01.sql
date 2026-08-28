-- Helix Forge Lab synthetic evaluation record: p04-full-001969
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001969 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001969 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 161, 0.888, 0.864, 0.876),
  ('slice-02', 245, 0.870, 0.877, 0.873),
  ('slice-03', 210, 0.887, 0.853, 0.870),
  ('slice-04', 211, 0.869, 0.866, 0.867),
  ('slice-05', 230, 0.886, 0.879, 0.882),
  ('slice-06', 211, 0.868, 0.855, 0.861),
  ('slice-07', 220, 0.885, 0.868, 0.876),
  ('slice-08', 195, 0.867, 0.881, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001969;
ROLLBACK;
