-- Helix Forge Lab synthetic evaluation record: p04-full-000674
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_000674 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_000674 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 164, 0.871, 0.855, 0.863),
  ('slice-02', 246, 0.888, 0.868, 0.878),
  ('slice-03', 163, 0.870, 0.881, 0.875),
  ('slice-04', 188, 0.887, 0.857, 0.872),
  ('slice-05', 171, 0.869, 0.870, 0.869),
  ('slice-06', 229, 0.886, 0.883, 0.884),
  ('slice-07', 229, 0.868, 0.859, 0.863),
  ('slice-08', 200, 0.885, 0.872, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_000674;
ROLLBACK;
