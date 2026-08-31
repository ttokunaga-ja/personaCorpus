-- Juniper Works synthetic evaluation record: p08-full-004847
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004847 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004847 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 211, 0.888, 0.870, 0.879),
  ('slice-02', 222, 0.870, 0.883, 0.876),
  ('slice-03', 199, 0.887, 0.859, 0.873),
  ('slice-04', 206, 0.869, 0.872, 0.870),
  ('slice-05', 180, 0.886, 0.848, 0.867),
  ('slice-06', 186, 0.868, 0.861, 0.864),
  ('slice-07', 160, 0.885, 0.874, 0.879),
  ('slice-08', 165, 0.867, 0.850, 0.858);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004847;
ROLLBACK;
