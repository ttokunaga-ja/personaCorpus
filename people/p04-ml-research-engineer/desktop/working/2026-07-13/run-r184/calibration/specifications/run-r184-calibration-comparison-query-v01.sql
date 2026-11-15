-- Helix Forge Lab synthetic evaluation record: p04-full-000536
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_000536 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_000536 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 181, 0.878, 0.866, 0.872),
  ('slice-02', 160, 0.895, 0.879, 0.887),
  ('slice-03', 215, 0.877, 0.855, 0.866),
  ('slice-04', 166, 0.894, 0.868, 0.881),
  ('slice-05', 160, 0.876, 0.881, 0.878),
  ('slice-06', 176, 0.893, 0.857, 0.875),
  ('slice-07', 202, 0.875, 0.870, 0.872),
  ('slice-08', 234, 0.892, 0.883, 0.887);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_000536;
ROLLBACK;
