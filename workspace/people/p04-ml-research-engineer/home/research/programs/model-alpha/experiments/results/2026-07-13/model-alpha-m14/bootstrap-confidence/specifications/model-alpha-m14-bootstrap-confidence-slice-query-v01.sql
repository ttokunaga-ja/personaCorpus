-- Helix Forge Lab synthetic evaluation record: p04-full-003578
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003578 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003578 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 176, 0.888, 0.859, 0.873),
  ('slice-02', 252, 0.870, 0.872, 0.871),
  ('slice-03', 166, 0.887, 0.848, 0.867),
  ('slice-04', 189, 0.869, 0.861, 0.865),
  ('slice-05', 219, 0.886, 0.874, 0.880),
  ('slice-06', 253, 0.868, 0.850, 0.859),
  ('slice-07', 193, 0.885, 0.863, 0.874),
  ('slice-08', 229, 0.867, 0.876, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003578;
ROLLBACK;
