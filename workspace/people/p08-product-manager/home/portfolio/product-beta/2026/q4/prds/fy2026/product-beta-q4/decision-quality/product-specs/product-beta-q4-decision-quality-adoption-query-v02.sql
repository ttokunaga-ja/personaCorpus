-- Juniper Works synthetic evaluation record: p08-full-000716
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000716 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000716 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 215, 0.867, 0.881, 0.874),
  ('slice-02', 224, 0.884, 0.857, 0.870),
  ('slice-03', 200, 0.866, 0.870, 0.868),
  ('slice-04', 254, 0.883, 0.883, 0.883),
  ('slice-05', 204, 0.865, 0.859, 0.862),
  ('slice-06', 245, 0.882, 0.872, 0.877),
  ('slice-07', 189, 0.864, 0.848, 0.856),
  ('slice-08', 180, 0.881, 0.861, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000716;
ROLLBACK;
