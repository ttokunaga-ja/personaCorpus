-- Juniper Works synthetic evaluation record: p08-full-004948
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004948 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004948 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 215, 0.862, 0.855, 0.858),
  ('slice-02', 224, 0.879, 0.868, 0.873),
  ('slice-03', 200, 0.896, 0.881, 0.888),
  ('slice-04', 206, 0.878, 0.857, 0.867),
  ('slice-05', 180, 0.895, 0.870, 0.882),
  ('slice-06', 233, 0.877, 0.883, 0.880),
  ('slice-07', 231, 0.894, 0.859, 0.876),
  ('slice-08', 248, 0.876, 0.872, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004948;
ROLLBACK;
