-- Helix Forge Lab synthetic evaluation record: p04-full-005627
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005627 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005627 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 194, 0.886, 0.879, 0.882),
  ('slice-02', 214, 0.868, 0.855, 0.861),
  ('slice-03', 195, 0.885, 0.868, 0.876),
  ('slice-04', 204, 0.867, 0.881, 0.874),
  ('slice-05', 179, 0.884, 0.857, 0.870),
  ('slice-06', 185, 0.866, 0.870, 0.868),
  ('slice-07', 254, 0.883, 0.883, 0.883),
  ('slice-08', 165, 0.865, 0.859, 0.862);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005627;
ROLLBACK;
