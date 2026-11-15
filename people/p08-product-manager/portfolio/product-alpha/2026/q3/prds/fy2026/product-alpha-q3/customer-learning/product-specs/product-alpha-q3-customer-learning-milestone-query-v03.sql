-- Juniper Works synthetic evaluation record: p08-full-004460
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004460 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004460 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 249, 0.891, 0.871, 0.881),
  ('slice-02', 194, 0.873, 0.847, 0.860),
  ('slice-03', 185, 0.890, 0.860, 0.875),
  ('slice-04', 246, 0.872, 0.873, 0.872),
  ('slice-05', 248, 0.889, 0.849, 0.869),
  ('slice-06', 220, 0.871, 0.862, 0.866),
  ('slice-07', 224, 0.888, 0.875, 0.881),
  ('slice-08', 245, 0.870, 0.851, 0.860);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004460;
ROLLBACK;
