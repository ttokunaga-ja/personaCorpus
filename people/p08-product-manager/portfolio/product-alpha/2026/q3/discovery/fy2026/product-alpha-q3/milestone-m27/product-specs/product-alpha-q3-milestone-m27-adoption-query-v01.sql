-- Juniper Works synthetic evaluation record: p08-full-006641
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006641 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006641 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 238, 0.890, 0.881, 0.885),
  ('slice-02', 188, 0.872, 0.857, 0.864),
  ('slice-03', 229, 0.889, 0.870, 0.879),
  ('slice-04', 173, 0.871, 0.883, 0.877),
  ('slice-05', 164, 0.888, 0.859, 0.873),
  ('slice-06', 225, 0.870, 0.872, 0.871),
  ('slice-07', 179, 0.887, 0.848, 0.867),
  ('slice-08', 175, 0.869, 0.861, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006641;
ROLLBACK;
