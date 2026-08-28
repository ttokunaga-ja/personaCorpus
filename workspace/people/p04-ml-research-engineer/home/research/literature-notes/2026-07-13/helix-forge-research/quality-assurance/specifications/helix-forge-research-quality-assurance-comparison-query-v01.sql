-- Helix Forge Lab synthetic evaluation record: p04-full-007075
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007075 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007075 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 253, 0.885, 0.858, 0.871),
  ('slice-02', 243, 0.867, 0.871, 0.869),
  ('slice-03', 209, 0.884, 0.847, 0.865),
  ('slice-04', 211, 0.866, 0.860, 0.863),
  ('slice-05', 230, 0.883, 0.873, 0.878),
  ('slice-06', 163, 0.865, 0.849, 0.857),
  ('slice-07', 243, 0.882, 0.862, 0.872),
  ('slice-08', 254, 0.864, 0.875, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007075;
ROLLBACK;
