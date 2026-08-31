-- Juniper Works synthetic evaluation record: p08-full-005343
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005343 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005343 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 176, 0.893, 0.855, 0.874),
  ('slice-02', 252, 0.875, 0.868, 0.871),
  ('slice-03', 214, 0.892, 0.881, 0.886),
  ('slice-04', 213, 0.874, 0.857, 0.865),
  ('slice-05', 231, 0.891, 0.870, 0.880),
  ('slice-06', 211, 0.873, 0.883, 0.878),
  ('slice-07', 172, 0.890, 0.859, 0.874),
  ('slice-08', 219, 0.872, 0.872, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005343;
ROLLBACK;
