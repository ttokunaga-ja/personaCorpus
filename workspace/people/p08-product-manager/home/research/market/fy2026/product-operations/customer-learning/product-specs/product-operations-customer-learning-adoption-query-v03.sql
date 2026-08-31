-- Juniper Works synthetic evaluation record: p08-full-007747
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007747 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007747 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 237, 0.874, 0.851, 0.862),
  ('slice-02', 235, 0.891, 0.864, 0.877),
  ('slice-03', 205, 0.873, 0.877, 0.875),
  ('slice-04', 161, 0.890, 0.853, 0.871),
  ('slice-05', 253, 0.872, 0.866, 0.869),
  ('slice-06', 175, 0.889, 0.879, 0.884),
  ('slice-07', 202, 0.871, 0.855, 0.863),
  ('slice-08', 186, 0.888, 0.868, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007747;
ROLLBACK;
