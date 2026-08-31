-- Juniper Works synthetic evaluation record: p08-full-000814
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000814 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000814 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 233, 0.880, 0.862, 0.871),
  ('slice-02', 233, 0.862, 0.875, 0.868),
  ('slice-03', 252, 0.879, 0.851, 0.865),
  ('slice-04', 185, 0.896, 0.864, 0.880),
  ('slice-05', 217, 0.878, 0.877, 0.877),
  ('slice-06', 204, 0.895, 0.853, 0.873),
  ('slice-07', 169, 0.877, 0.866, 0.871),
  ('slice-08', 217, 0.894, 0.879, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000814;
ROLLBACK;
