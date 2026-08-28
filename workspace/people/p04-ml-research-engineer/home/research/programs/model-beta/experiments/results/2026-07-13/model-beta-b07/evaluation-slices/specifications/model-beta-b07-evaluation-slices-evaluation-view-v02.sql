-- Helix Forge Lab synthetic evaluation record: p04-full-009277
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009277 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009277 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 209, 0.866, 0.856, 0.861),
  ('slice-02', 174, 0.883, 0.869, 0.876),
  ('slice-03', 175, 0.865, 0.882, 0.873),
  ('slice-04', 241, 0.882, 0.858, 0.870),
  ('slice-05', 198, 0.864, 0.871, 0.867),
  ('slice-06', 195, 0.881, 0.847, 0.864),
  ('slice-07', 212, 0.863, 0.860, 0.861),
  ('slice-08', 239, 0.880, 0.873, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009277;
ROLLBACK;
