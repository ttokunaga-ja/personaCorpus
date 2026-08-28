-- Helix Forge Lab synthetic evaluation record: p04-full-000865
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_000865 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_000865 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 200, 0.887, 0.854, 0.870),
  ('slice-02', 217, 0.869, 0.867, 0.868),
  ('slice-03', 244, 0.886, 0.880, 0.883),
  ('slice-04', 181, 0.868, 0.856, 0.862),
  ('slice-05', 215, 0.885, 0.869, 0.877),
  ('slice-06', 251, 0.867, 0.882, 0.874),
  ('slice-07', 240, 0.884, 0.858, 0.871),
  ('slice-08', 253, 0.866, 0.871, 0.868);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_000865;
ROLLBACK;
