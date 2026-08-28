-- Helix Forge Lab synthetic evaluation record: p04-full-003228
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003228 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003228 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 234, 0.871, 0.862, 0.866),
  ('slice-02', 234, 0.888, 0.875, 0.881),
  ('slice-03', 205, 0.870, 0.851, 0.860),
  ('slice-04', 161, 0.887, 0.864, 0.875),
  ('slice-05', 253, 0.869, 0.877, 0.873),
  ('slice-06', 175, 0.886, 0.853, 0.869),
  ('slice-07', 202, 0.868, 0.866, 0.867),
  ('slice-08', 186, 0.885, 0.879, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003228;
ROLLBACK;
