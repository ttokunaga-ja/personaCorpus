-- Helix Forge Lab synthetic evaluation record: p04-full-000813
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_000813 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_000813 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 165, 0.882, 0.862, 0.872),
  ('slice-02', 247, 0.864, 0.875, 0.869),
  ('slice-03', 164, 0.881, 0.851, 0.866),
  ('slice-04', 188, 0.863, 0.864, 0.863),
  ('slice-05', 171, 0.880, 0.877, 0.878),
  ('slice-06', 229, 0.862, 0.853, 0.857),
  ('slice-07', 181, 0.879, 0.866, 0.872),
  ('slice-08', 223, 0.896, 0.879, 0.887);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_000813;
ROLLBACK;
