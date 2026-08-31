-- Juniper Works synthetic evaluation record: p08-full-000356
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000356 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000356 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 170, 0.882, 0.860, 0.871),
  ('slice-02', 202, 0.864, 0.873, 0.868),
  ('slice-03', 189, 0.881, 0.849, 0.865),
  ('slice-04', 201, 0.863, 0.862, 0.862),
  ('slice-05', 178, 0.880, 0.875, 0.877),
  ('slice-06', 185, 0.862, 0.851, 0.856),
  ('slice-07', 207, 0.879, 0.864, 0.871),
  ('slice-08', 189, 0.896, 0.877, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000356;
ROLLBACK;
