-- Juniper Works synthetic evaluation record: p08-full-005450
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005450 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005450 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 175, 0.862, 0.869, 0.865),
  ('slice-02', 204, 0.879, 0.882, 0.880),
  ('slice-03', 190, 0.896, 0.858, 0.877),
  ('slice-04', 201, 0.878, 0.871, 0.874),
  ('slice-05', 178, 0.895, 0.847, 0.870),
  ('slice-06', 185, 0.877, 0.860, 0.868),
  ('slice-07', 254, 0.894, 0.873, 0.883),
  ('slice-08', 212, 0.876, 0.849, 0.862);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005450;
ROLLBACK;
