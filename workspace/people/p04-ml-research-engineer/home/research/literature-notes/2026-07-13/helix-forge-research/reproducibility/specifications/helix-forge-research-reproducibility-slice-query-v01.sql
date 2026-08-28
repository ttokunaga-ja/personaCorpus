-- Helix Forge Lab synthetic evaluation record: p04-full-001827
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001827 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001827 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 169, 0.866, 0.858, 0.862),
  ('slice-02', 201, 0.883, 0.871, 0.877),
  ('slice-03', 188, 0.865, 0.847, 0.856),
  ('slice-04', 248, 0.882, 0.860, 0.871),
  ('slice-05', 249, 0.864, 0.873, 0.868),
  ('slice-06', 173, 0.881, 0.849, 0.865),
  ('slice-07', 248, 0.863, 0.862, 0.862),
  ('slice-08', 209, 0.880, 0.875, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001827;
ROLLBACK;
