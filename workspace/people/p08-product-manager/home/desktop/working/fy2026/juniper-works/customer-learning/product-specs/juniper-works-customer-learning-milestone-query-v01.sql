-- Juniper Works synthetic evaluation record: p08-full-006749
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006749 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006749 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 189, 0.896, 0.857, 0.876),
  ('slice-02', 164, 0.878, 0.870, 0.874),
  ('slice-03', 217, 0.895, 0.883, 0.889),
  ('slice-04', 167, 0.877, 0.859, 0.868),
  ('slice-05', 208, 0.894, 0.872, 0.883),
  ('slice-06', 247, 0.876, 0.848, 0.862),
  ('slice-07', 190, 0.893, 0.861, 0.877),
  ('slice-08', 228, 0.875, 0.874, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006749;
ROLLBACK;
