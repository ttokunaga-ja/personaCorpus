-- Juniper Works synthetic evaluation record: p08-full-004651
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004651 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004651 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 241, 0.878, 0.867, 0.872),
  ('slice-02', 190, 0.895, 0.880, 0.887),
  ('slice-03', 183, 0.877, 0.856, 0.866),
  ('slice-04', 198, 0.894, 0.869, 0.881),
  ('slice-05', 224, 0.876, 0.882, 0.879),
  ('slice-06', 208, 0.893, 0.858, 0.875),
  ('slice-07', 218, 0.875, 0.871, 0.873),
  ('slice-08', 194, 0.892, 0.847, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004651;
ROLLBACK;
