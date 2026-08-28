-- Helix Forge Lab synthetic evaluation record: p04-full-003330
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003330 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003330 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 254, 0.871, 0.873, 0.872),
  ('slice-02', 196, 0.888, 0.849, 0.868),
  ('slice-03', 186, 0.870, 0.862, 0.866),
  ('slice-04', 247, 0.887, 0.875, 0.881),
  ('slice-05', 201, 0.869, 0.851, 0.860),
  ('slice-06', 196, 0.886, 0.864, 0.875),
  ('slice-07', 212, 0.868, 0.877, 0.872),
  ('slice-08', 191, 0.885, 0.853, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003330;
ROLLBACK;
