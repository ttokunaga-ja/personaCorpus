-- Juniper Works synthetic evaluation record: p08-full-002685
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002685 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002685 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 219, 0.886, 0.873, 0.879),
  ('slice-02', 226, 0.868, 0.849, 0.858),
  ('slice-03', 201, 0.885, 0.862, 0.873),
  ('slice-04', 254, 0.867, 0.875, 0.871),
  ('slice-05', 204, 0.884, 0.851, 0.867),
  ('slice-06', 245, 0.866, 0.864, 0.865),
  ('slice-07', 189, 0.883, 0.877, 0.880),
  ('slice-08', 227, 0.865, 0.853, 0.859);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002685;
ROLLBACK;
