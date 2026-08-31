-- Juniper Works synthetic evaluation record: p08-full-000104
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000104 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000104 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 222, 0.894, 0.851, 0.872),
  ('slice-02', 180, 0.876, 0.864, 0.870),
  ('slice-03', 178, 0.893, 0.877, 0.885),
  ('slice-04', 195, 0.875, 0.853, 0.864),
  ('slice-05', 222, 0.892, 0.866, 0.879),
  ('slice-06', 254, 0.874, 0.879, 0.876),
  ('slice-07', 241, 0.891, 0.855, 0.873),
  ('slice-08', 206, 0.873, 0.868, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000104;
ROLLBACK;
