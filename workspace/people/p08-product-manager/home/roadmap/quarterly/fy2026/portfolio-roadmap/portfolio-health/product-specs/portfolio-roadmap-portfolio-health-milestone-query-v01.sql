-- Juniper Works synthetic evaluation record: p08-full-001497
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001497 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001497 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 201, 0.893, 0.848, 0.870),
  ('slice-02', 170, 0.875, 0.861, 0.868),
  ('slice-03', 220, 0.892, 0.874, 0.883),
  ('slice-04', 216, 0.874, 0.850, 0.862),
  ('slice-05', 185, 0.891, 0.863, 0.877),
  ('slice-06', 188, 0.873, 0.876, 0.874),
  ('slice-07', 161, 0.890, 0.852, 0.871),
  ('slice-08', 213, 0.872, 0.865, 0.868);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001497;
ROLLBACK;
