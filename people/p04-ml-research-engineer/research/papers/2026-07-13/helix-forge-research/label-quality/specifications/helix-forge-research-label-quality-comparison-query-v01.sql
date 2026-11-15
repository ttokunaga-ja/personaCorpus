-- Helix Forge Lab synthetic evaluation record: p04-full-003133
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003133 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003133 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 168, 0.880, 0.853, 0.866),
  ('slice-02', 201, 0.862, 0.866, 0.864),
  ('slice-03', 236, 0.879, 0.879, 0.879),
  ('slice-04', 224, 0.896, 0.855, 0.875),
  ('slice-05', 237, 0.878, 0.868, 0.873),
  ('slice-06', 167, 0.895, 0.881, 0.888),
  ('slice-07', 198, 0.877, 0.857, 0.867),
  ('slice-08', 184, 0.894, 0.870, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003133;
ROLLBACK;
