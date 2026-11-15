-- Helix Forge Lab synthetic evaluation record: p04-full-006220
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006220 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006220 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 172, 0.874, 0.851, 0.862),
  ('slice-02', 203, 0.891, 0.864, 0.877),
  ('slice-03', 237, 0.873, 0.877, 0.875),
  ('slice-04', 177, 0.890, 0.853, 0.871),
  ('slice-05', 213, 0.872, 0.866, 0.869),
  ('slice-06', 250, 0.889, 0.879, 0.884),
  ('slice-07', 239, 0.871, 0.855, 0.863),
  ('slice-08', 205, 0.888, 0.868, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006220;
ROLLBACK;
