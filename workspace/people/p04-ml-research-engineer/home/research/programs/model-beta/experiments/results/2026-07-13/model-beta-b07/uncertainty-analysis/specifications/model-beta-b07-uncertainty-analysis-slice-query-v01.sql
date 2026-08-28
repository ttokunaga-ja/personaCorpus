-- Helix Forge Lab synthetic evaluation record: p04-full-009077
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009077 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009077 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 210, 0.882, 0.882, 0.882),
  ('slice-02', 174, 0.864, 0.858, 0.861),
  ('slice-03', 222, 0.881, 0.871, 0.876),
  ('slice-04', 217, 0.863, 0.847, 0.855),
  ('slice-05', 186, 0.880, 0.860, 0.870),
  ('slice-06', 189, 0.862, 0.873, 0.867),
  ('slice-07', 161, 0.879, 0.849, 0.864),
  ('slice-08', 166, 0.896, 0.862, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009077;
ROLLBACK;
