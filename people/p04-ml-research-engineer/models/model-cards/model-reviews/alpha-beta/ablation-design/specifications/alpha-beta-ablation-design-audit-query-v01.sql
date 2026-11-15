-- Helix Forge Lab synthetic evaluation record: p04-full-004627
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004627 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004627 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 242, 0.869, 0.862, 0.865),
  ('slice-02', 238, 0.886, 0.875, 0.880),
  ('slice-03', 207, 0.868, 0.851, 0.859),
  ('slice-04', 210, 0.885, 0.864, 0.874),
  ('slice-05', 230, 0.867, 0.877, 0.872),
  ('slice-06', 163, 0.884, 0.853, 0.868),
  ('slice-07', 196, 0.866, 0.866, 0.866),
  ('slice-08', 231, 0.883, 0.879, 0.881);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004627;
ROLLBACK;
