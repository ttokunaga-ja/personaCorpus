-- Juniper Works synthetic evaluation record: p08-full-005948
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005948 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005948 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 172, 0.884, 0.879, 0.881),
  ('slice-02', 250, 0.866, 0.855, 0.860),
  ('slice-03', 165, 0.883, 0.868, 0.875),
  ('slice-04', 189, 0.865, 0.881, 0.873),
  ('slice-05', 219, 0.882, 0.857, 0.869),
  ('slice-06', 205, 0.864, 0.870, 0.867),
  ('slice-07', 217, 0.881, 0.883, 0.882),
  ('slice-08', 194, 0.863, 0.859, 0.861);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005948;
ROLLBACK;
