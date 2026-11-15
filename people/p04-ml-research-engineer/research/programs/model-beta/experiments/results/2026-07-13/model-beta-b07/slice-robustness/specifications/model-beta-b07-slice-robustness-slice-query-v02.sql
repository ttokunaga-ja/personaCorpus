-- Helix Forge Lab synthetic evaluation record: p04-full-007277
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007277 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007277 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 229, 0.866, 0.880, 0.873),
  ('slice-02', 231, 0.883, 0.856, 0.869),
  ('slice-03', 203, 0.865, 0.869, 0.867),
  ('slice-04', 208, 0.882, 0.882, 0.882),
  ('slice-05', 181, 0.864, 0.858, 0.861),
  ('slice-06', 186, 0.881, 0.871, 0.876),
  ('slice-07', 160, 0.863, 0.847, 0.855),
  ('slice-08', 213, 0.880, 0.860, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007277;
ROLLBACK;
