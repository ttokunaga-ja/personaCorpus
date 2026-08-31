-- Juniper Works synthetic evaluation record: p08-full-001684
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001684 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001684 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 206, 0.873, 0.875, 0.874),
  ('slice-02', 220, 0.890, 0.851, 0.870),
  ('slice-03', 245, 0.872, 0.864, 0.868),
  ('slice-04', 181, 0.889, 0.877, 0.883),
  ('slice-05', 168, 0.871, 0.853, 0.862),
  ('slice-06', 227, 0.888, 0.866, 0.877),
  ('slice-07', 228, 0.870, 0.879, 0.874),
  ('slice-08', 199, 0.887, 0.855, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001684;
ROLLBACK;
