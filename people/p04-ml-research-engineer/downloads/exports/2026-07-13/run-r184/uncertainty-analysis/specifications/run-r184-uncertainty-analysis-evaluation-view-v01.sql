-- Helix Forge Lab synthetic evaluation record: p04-full-003925
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003925 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003925 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 236, 0.873, 0.856, 0.864),
  ('slice-02', 187, 0.890, 0.869, 0.879),
  ('slice-03', 229, 0.872, 0.882, 0.877),
  ('slice-04', 173, 0.889, 0.858, 0.873),
  ('slice-05', 164, 0.871, 0.871, 0.871),
  ('slice-06', 225, 0.888, 0.847, 0.867),
  ('slice-07', 227, 0.870, 0.860, 0.865),
  ('slice-08', 199, 0.887, 0.873, 0.880);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003925;
ROLLBACK;
