-- Helix Forge Lab synthetic evaluation record: p04-full-001584
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001584 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001584 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 169, 0.866, 0.881, 0.873),
  ('slice-02', 249, 0.883, 0.857, 0.870),
  ('slice-03', 165, 0.865, 0.870, 0.867),
  ('slice-04', 236, 0.882, 0.883, 0.882),
  ('slice-05', 195, 0.864, 0.859, 0.861),
  ('slice-06', 241, 0.881, 0.872, 0.876),
  ('slice-07', 187, 0.863, 0.848, 0.855),
  ('slice-08', 179, 0.880, 0.861, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001584;
ROLLBACK;
