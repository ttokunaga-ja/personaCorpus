-- Juniper Works synthetic evaluation record: p08-full-006941
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006941 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006941 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 174, 0.886, 0.860, 0.873),
  ('slice-02', 204, 0.868, 0.873, 0.870),
  ('slice-03', 190, 0.885, 0.849, 0.867),
  ('slice-04', 201, 0.867, 0.862, 0.864),
  ('slice-05', 225, 0.884, 0.875, 0.879),
  ('slice-06', 161, 0.866, 0.851, 0.858),
  ('slice-07', 195, 0.883, 0.864, 0.873),
  ('slice-08', 230, 0.865, 0.877, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006941;
ROLLBACK;
