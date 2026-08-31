-- Juniper Works synthetic evaluation record: p08-full-005151
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005151 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005151 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 166, 0.888, 0.883, 0.885),
  ('slice-02', 200, 0.870, 0.859, 0.864),
  ('slice-03', 235, 0.887, 0.872, 0.879),
  ('slice-04', 176, 0.869, 0.848, 0.858),
  ('slice-05', 165, 0.886, 0.861, 0.873),
  ('slice-06', 178, 0.868, 0.874, 0.871),
  ('slice-07', 203, 0.885, 0.850, 0.867),
  ('slice-08', 234, 0.867, 0.863, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005151;
ROLLBACK;
