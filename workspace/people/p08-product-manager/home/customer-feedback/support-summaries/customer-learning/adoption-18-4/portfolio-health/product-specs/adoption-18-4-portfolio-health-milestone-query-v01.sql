-- Juniper Works synthetic evaluation record: p08-full-000027
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000027 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000027 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 205, 0.877, 0.857, 0.867),
  ('slice-02', 172, 0.894, 0.870, 0.882),
  ('slice-03', 174, 0.876, 0.883, 0.879),
  ('slice-04', 241, 0.893, 0.859, 0.876),
  ('slice-05', 245, 0.875, 0.872, 0.873),
  ('slice-06', 171, 0.892, 0.848, 0.869),
  ('slice-07', 247, 0.874, 0.861, 0.867),
  ('slice-08', 161, 0.891, 0.874, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000027;
ROLLBACK;
