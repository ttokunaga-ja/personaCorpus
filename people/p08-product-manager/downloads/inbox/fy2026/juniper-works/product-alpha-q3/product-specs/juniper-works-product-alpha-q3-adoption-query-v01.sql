-- Juniper Works synthetic evaluation record: p08-full-006840
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006840 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006840 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 201, 0.883, 0.862, 0.872),
  ('slice-02', 217, 0.865, 0.875, 0.870),
  ('slice-03', 196, 0.882, 0.851, 0.866),
  ('slice-04', 204, 0.864, 0.864, 0.864),
  ('slice-05', 227, 0.881, 0.877, 0.879),
  ('slice-06', 209, 0.863, 0.853, 0.858),
  ('slice-07', 171, 0.880, 0.866, 0.873),
  ('slice-08', 171, 0.862, 0.879, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006840;
ROLLBACK;
