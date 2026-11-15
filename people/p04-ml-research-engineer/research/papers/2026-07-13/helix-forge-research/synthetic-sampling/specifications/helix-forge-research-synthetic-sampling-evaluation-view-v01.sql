-- Helix Forge Lab synthetic evaluation record: p04-full-002896
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002896 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002896 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 229, 0.896, 0.854, 0.874),
  ('slice-02', 231, 0.878, 0.867, 0.872),
  ('slice-03', 203, 0.895, 0.880, 0.887),
  ('slice-04', 208, 0.877, 0.856, 0.866),
  ('slice-05', 229, 0.894, 0.869, 0.881),
  ('slice-06', 163, 0.876, 0.882, 0.879),
  ('slice-07', 196, 0.893, 0.858, 0.875),
  ('slice-08', 231, 0.875, 0.871, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002896;
ROLLBACK;
