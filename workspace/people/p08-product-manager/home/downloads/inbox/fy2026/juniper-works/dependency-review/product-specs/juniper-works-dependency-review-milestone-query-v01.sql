-- Juniper Works synthetic evaluation record: p08-full-005054
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005054 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005054 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 246, 0.873, 0.860, 0.866),
  ('slice-02', 240, 0.890, 0.873, 0.881),
  ('slice-03', 160, 0.872, 0.849, 0.860),
  ('slice-04', 234, 0.889, 0.862, 0.875),
  ('slice-05', 194, 0.871, 0.875, 0.873),
  ('slice-06', 193, 0.888, 0.851, 0.869),
  ('slice-07', 163, 0.870, 0.864, 0.867),
  ('slice-08', 167, 0.887, 0.877, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005054;
ROLLBACK;
