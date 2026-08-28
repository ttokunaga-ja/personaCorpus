-- Helix Forge Lab synthetic evaluation record: p04-full-002702
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002702 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002702 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 170, 0.877, 0.860, 0.868),
  ('slice-02', 249, 0.894, 0.873, 0.883),
  ('slice-03', 212, 0.876, 0.849, 0.862),
  ('slice-04', 212, 0.893, 0.862, 0.877),
  ('slice-05', 231, 0.875, 0.875, 0.875),
  ('slice-06', 211, 0.892, 0.851, 0.871),
  ('slice-07', 220, 0.874, 0.864, 0.869),
  ('slice-08', 243, 0.891, 0.877, 0.884);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002702;
ROLLBACK;
