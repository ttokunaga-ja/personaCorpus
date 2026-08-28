-- Helix Forge Lab synthetic evaluation record: p04-full-004078
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004078 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004078 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 242, 0.889, 0.854, 0.871),
  ('slice-02', 238, 0.871, 0.867, 0.869),
  ('slice-03', 207, 0.888, 0.880, 0.884),
  ('slice-04', 162, 0.870, 0.856, 0.863),
  ('slice-05', 253, 0.887, 0.869, 0.878),
  ('slice-06', 175, 0.869, 0.882, 0.875),
  ('slice-07', 202, 0.886, 0.858, 0.872),
  ('slice-08', 186, 0.868, 0.871, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004078;
ROLLBACK;
