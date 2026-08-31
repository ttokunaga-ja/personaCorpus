-- Juniper Works synthetic evaluation record: p08-full-002873
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002873 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002873 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 227, 0.894, 0.860, 0.877),
  ('slice-02', 230, 0.876, 0.873, 0.874),
  ('slice-03', 250, 0.893, 0.849, 0.870),
  ('slice-04', 184, 0.875, 0.862, 0.868),
  ('slice-05', 169, 0.892, 0.875, 0.883),
  ('slice-06', 228, 0.874, 0.851, 0.862),
  ('slice-07', 228, 0.891, 0.864, 0.877),
  ('slice-08', 199, 0.873, 0.877, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002873;
ROLLBACK;
