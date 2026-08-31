-- Juniper Works synthetic evaluation record: p08-full-007048
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007048 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007048 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 196, 0.893, 0.853, 0.873),
  ('slice-02', 167, 0.875, 0.866, 0.870),
  ('slice-03', 219, 0.892, 0.879, 0.885),
  ('slice-04', 168, 0.874, 0.855, 0.864),
  ('slice-05', 161, 0.891, 0.868, 0.879),
  ('slice-06', 224, 0.873, 0.881, 0.877),
  ('slice-07', 179, 0.890, 0.857, 0.873),
  ('slice-08', 175, 0.872, 0.870, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007048;
ROLLBACK;
