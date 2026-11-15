-- Helix Forge Lab synthetic evaluation record: p04-full-002463
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002463 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002463 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 243, 0.890, 0.858, 0.874),
  ('slice-02', 238, 0.872, 0.871, 0.871),
  ('slice-03', 254, 0.889, 0.847, 0.867),
  ('slice-04', 186, 0.871, 0.860, 0.865),
  ('slice-05', 170, 0.888, 0.873, 0.880),
  ('slice-06', 228, 0.870, 0.849, 0.859),
  ('slice-07', 228, 0.887, 0.862, 0.874),
  ('slice-08', 247, 0.869, 0.875, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002463;
ROLLBACK;
