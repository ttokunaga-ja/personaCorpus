-- Helix Forge Lab synthetic evaluation record: p04-full-005026
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005026 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005026 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 191, 0.873, 0.880, 0.876),
  ('slice-02', 165, 0.890, 0.856, 0.873),
  ('slice-03', 218, 0.872, 0.869, 0.870),
  ('slice-04', 215, 0.889, 0.882, 0.885),
  ('slice-05', 232, 0.871, 0.858, 0.864),
  ('slice-06', 212, 0.888, 0.871, 0.879),
  ('slice-07', 173, 0.870, 0.847, 0.858),
  ('slice-08', 219, 0.887, 0.860, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005026;
ROLLBACK;
