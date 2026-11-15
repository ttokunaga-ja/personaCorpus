-- Juniper Works synthetic evaluation record: p08-full-003664
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003664 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003664 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 210, 0.882, 0.868, 0.875),
  ('slice-02', 222, 0.864, 0.881, 0.872),
  ('slice-03', 199, 0.881, 0.857, 0.869),
  ('slice-04', 206, 0.863, 0.870, 0.866),
  ('slice-05', 180, 0.880, 0.883, 0.881),
  ('slice-06', 186, 0.862, 0.859, 0.860),
  ('slice-07', 207, 0.879, 0.872, 0.875),
  ('slice-08', 236, 0.896, 0.848, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003664;
ROLLBACK;
