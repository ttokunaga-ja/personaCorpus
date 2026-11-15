-- Juniper Works synthetic evaluation record: p08-full-002486
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002486 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002486 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 250, 0.882, 0.859, 0.870),
  ('slice-02', 194, 0.864, 0.872, 0.868),
  ('slice-03', 185, 0.881, 0.848, 0.864),
  ('slice-04', 246, 0.863, 0.861, 0.862),
  ('slice-05', 248, 0.880, 0.874, 0.877),
  ('slice-06', 220, 0.862, 0.850, 0.856),
  ('slice-07', 224, 0.879, 0.863, 0.871),
  ('slice-08', 245, 0.896, 0.876, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002486;
ROLLBACK;
