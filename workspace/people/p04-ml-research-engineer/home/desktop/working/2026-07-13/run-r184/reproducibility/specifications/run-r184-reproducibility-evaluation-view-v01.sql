-- Helix Forge Lab synthetic evaluation record: p04-full-004175
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004175 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004175 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 250, 0.882, 0.870, 0.876),
  ('slice-02', 242, 0.864, 0.883, 0.873),
  ('slice-03', 209, 0.881, 0.859, 0.870),
  ('slice-04', 163, 0.863, 0.872, 0.867),
  ('slice-05', 254, 0.880, 0.848, 0.864),
  ('slice-06', 175, 0.862, 0.861, 0.861),
  ('slice-07', 249, 0.879, 0.874, 0.876),
  ('slice-08', 210, 0.896, 0.850, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004175;
ROLLBACK;
