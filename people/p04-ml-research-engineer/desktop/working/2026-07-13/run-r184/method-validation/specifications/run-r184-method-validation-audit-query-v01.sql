-- Helix Forge Lab synthetic evaluation record: p04-full-007173
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007173 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007173 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 194, 0.876, 0.862, 0.869),
  ('slice-02', 214, 0.893, 0.875, 0.884),
  ('slice-03', 195, 0.875, 0.851, 0.863),
  ('slice-04', 251, 0.892, 0.864, 0.878),
  ('slice-05', 203, 0.874, 0.877, 0.875),
  ('slice-06', 197, 0.891, 0.853, 0.872),
  ('slice-07', 213, 0.873, 0.866, 0.869),
  ('slice-08', 192, 0.890, 0.879, 0.884);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007173;
ROLLBACK;
