-- Juniper Works synthetic evaluation record: p08-full-003366
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003366 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003366 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 245, 0.872, 0.847, 0.859),
  ('slice-02', 192, 0.889, 0.860, 0.874),
  ('slice-03', 184, 0.871, 0.873, 0.872),
  ('slice-04', 246, 0.888, 0.849, 0.868),
  ('slice-05', 248, 0.870, 0.862, 0.866),
  ('slice-06', 172, 0.887, 0.875, 0.881),
  ('slice-07', 248, 0.869, 0.851, 0.860),
  ('slice-08', 209, 0.886, 0.864, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003366;
ROLLBACK;
