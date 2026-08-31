-- Juniper Works synthetic evaluation record: p08-full-006141
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006141 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006141 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 189, 0.896, 0.875, 0.885),
  ('slice-02', 211, 0.878, 0.851, 0.864),
  ('slice-03', 241, 0.895, 0.864, 0.879),
  ('slice-04', 227, 0.877, 0.877, 0.877),
  ('slice-05', 238, 0.894, 0.853, 0.873),
  ('slice-06', 167, 0.876, 0.866, 0.871),
  ('slice-07', 198, 0.893, 0.879, 0.886),
  ('slice-08', 232, 0.875, 0.855, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006141;
ROLLBACK;
