-- Juniper Works synthetic evaluation record: p08-full-003863
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003863 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003863 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 175, 0.892, 0.855, 0.873),
  ('slice-02', 252, 0.874, 0.868, 0.871),
  ('slice-03', 166, 0.891, 0.881, 0.886),
  ('slice-04', 237, 0.873, 0.857, 0.865),
  ('slice-05', 196, 0.890, 0.870, 0.880),
  ('slice-06', 194, 0.872, 0.883, 0.877),
  ('slice-07', 164, 0.889, 0.859, 0.874),
  ('slice-08', 167, 0.871, 0.872, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003863;
ROLLBACK;
