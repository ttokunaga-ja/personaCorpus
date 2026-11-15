-- Helix Forge Lab synthetic evaluation record: p04-full-008969
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008969 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008969 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 204, 0.891, 0.859, 0.875),
  ('slice-02', 171, 0.873, 0.872, 0.872),
  ('slice-03', 221, 0.890, 0.848, 0.868),
  ('slice-04', 169, 0.872, 0.861, 0.866),
  ('slice-05', 162, 0.889, 0.874, 0.881),
  ('slice-06', 224, 0.871, 0.850, 0.860),
  ('slice-07', 179, 0.888, 0.863, 0.875),
  ('slice-08', 222, 0.870, 0.876, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008969;
ROLLBACK;
