-- Helix Forge Lab synthetic evaluation record: p04-full-004471
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004471 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004471 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 204, 0.886, 0.856, 0.871),
  ('slice-02', 219, 0.868, 0.869, 0.868),
  ('slice-03', 197, 0.885, 0.882, 0.883),
  ('slice-04', 205, 0.867, 0.858, 0.862),
  ('slice-05', 180, 0.884, 0.871, 0.877),
  ('slice-06', 186, 0.866, 0.847, 0.856),
  ('slice-07', 160, 0.883, 0.860, 0.871),
  ('slice-08', 165, 0.865, 0.873, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004471;
ROLLBACK;
