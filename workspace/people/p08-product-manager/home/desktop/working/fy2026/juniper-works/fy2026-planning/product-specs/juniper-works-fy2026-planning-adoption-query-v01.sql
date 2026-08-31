-- Juniper Works synthetic evaluation record: p08-full-002585
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002585 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002585 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 195, 0.892, 0.880, 0.886),
  ('slice-02', 214, 0.874, 0.856, 0.865),
  ('slice-03', 195, 0.891, 0.869, 0.880),
  ('slice-04', 251, 0.873, 0.882, 0.877),
  ('slice-05', 203, 0.890, 0.858, 0.874),
  ('slice-06', 197, 0.872, 0.871, 0.871),
  ('slice-07', 165, 0.889, 0.847, 0.867),
  ('slice-08', 168, 0.871, 0.860, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002585;
ROLLBACK;
