-- Helix Forge Lab synthetic evaluation record: p04-full-002319
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002319 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002319 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 177, 0.869, 0.882, 0.875),
  ('slice-02', 205, 0.886, 0.858, 0.872),
  ('slice-03', 190, 0.868, 0.871, 0.869),
  ('slice-04', 201, 0.885, 0.847, 0.866),
  ('slice-05', 225, 0.867, 0.860, 0.863),
  ('slice-06', 208, 0.884, 0.873, 0.878),
  ('slice-07', 218, 0.866, 0.849, 0.857),
  ('slice-08', 194, 0.883, 0.862, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002319;
ROLLBACK;
