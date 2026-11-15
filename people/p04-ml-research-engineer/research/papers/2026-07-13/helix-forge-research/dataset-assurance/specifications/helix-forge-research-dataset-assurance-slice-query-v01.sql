-- Helix Forge Lab synthetic evaluation record: p04-full-007526
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007526 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007526 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 201, 0.868, 0.880, 0.874),
  ('slice-02', 217, 0.885, 0.856, 0.870),
  ('slice-03', 244, 0.867, 0.869, 0.868),
  ('slice-04', 228, 0.884, 0.882, 0.883),
  ('slice-05', 191, 0.866, 0.858, 0.862),
  ('slice-06', 239, 0.883, 0.871, 0.877),
  ('slice-07', 186, 0.865, 0.847, 0.856),
  ('slice-08', 178, 0.882, 0.860, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007526;
ROLLBACK;
