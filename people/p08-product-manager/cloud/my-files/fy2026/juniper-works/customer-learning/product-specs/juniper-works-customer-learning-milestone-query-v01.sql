-- Juniper Works synthetic evaluation record: p08-full-005742
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005742 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005742 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 207, 0.879, 0.855, 0.867),
  ('slice-02', 173, 0.896, 0.868, 0.882),
  ('slice-03', 174, 0.878, 0.881, 0.879),
  ('slice-04', 193, 0.895, 0.857, 0.876),
  ('slice-05', 221, 0.877, 0.870, 0.873),
  ('slice-06', 206, 0.894, 0.883, 0.888),
  ('slice-07', 170, 0.876, 0.859, 0.867),
  ('slice-08', 218, 0.893, 0.872, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005742;
ROLLBACK;
