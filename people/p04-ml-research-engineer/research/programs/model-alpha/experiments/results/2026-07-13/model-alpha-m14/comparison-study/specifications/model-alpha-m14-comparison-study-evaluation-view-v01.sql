-- Helix Forge Lab synthetic evaluation record: p04-full-004673
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004673 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004673 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 230, 0.892, 0.875, 0.883),
  ('slice-02', 184, 0.874, 0.851, 0.862),
  ('slice-03', 227, 0.891, 0.864, 0.877),
  ('slice-04', 220, 0.873, 0.877, 0.875),
  ('slice-05', 235, 0.890, 0.853, 0.871),
  ('slice-06', 213, 0.872, 0.866, 0.869),
  ('slice-07', 173, 0.889, 0.879, 0.884),
  ('slice-08', 219, 0.871, 0.855, 0.863);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004673;
ROLLBACK;
