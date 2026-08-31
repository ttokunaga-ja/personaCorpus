-- Juniper Works synthetic evaluation record: p08-full-001883
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001883 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001883 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 221, 0.863, 0.848, 0.855),
  ('slice-02', 227, 0.880, 0.861, 0.870),
  ('slice-03', 201, 0.862, 0.874, 0.868),
  ('slice-04', 254, 0.879, 0.850, 0.864),
  ('slice-05', 252, 0.896, 0.863, 0.879),
  ('slice-06', 222, 0.878, 0.876, 0.877),
  ('slice-07', 225, 0.895, 0.852, 0.873),
  ('slice-08', 245, 0.877, 0.865, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001883;
ROLLBACK;
