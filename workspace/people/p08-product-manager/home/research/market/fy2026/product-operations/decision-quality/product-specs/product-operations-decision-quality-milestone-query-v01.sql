-- Juniper Works synthetic evaluation record: p08-full-007547
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007547 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007547 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 176, 0.878, 0.864, 0.871),
  ('slice-02', 205, 0.895, 0.877, 0.886),
  ('slice-03', 190, 0.877, 0.853, 0.865),
  ('slice-04', 201, 0.894, 0.866, 0.880),
  ('slice-05', 225, 0.876, 0.879, 0.877),
  ('slice-06', 208, 0.893, 0.855, 0.874),
  ('slice-07', 218, 0.875, 0.868, 0.871),
  ('slice-08', 194, 0.892, 0.881, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007547;
ROLLBACK;
