-- Juniper Works synthetic evaluation record: p08-full-006547
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_006547 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_006547 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 169, 0.871, 0.873, 0.872),
  ('slice-02', 249, 0.888, 0.849, 0.868),
  ('slice-03', 212, 0.870, 0.862, 0.866),
  ('slice-04', 165, 0.887, 0.875, 0.881),
  ('slice-05', 160, 0.869, 0.851, 0.860),
  ('slice-06', 223, 0.886, 0.864, 0.875),
  ('slice-07', 178, 0.868, 0.877, 0.872),
  ('slice-08', 174, 0.885, 0.853, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_006547;
ROLLBACK;
