-- Juniper Works synthetic evaluation record: p08-full-007448
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007448 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007448 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 240, 0.887, 0.855, 0.871),
  ('slice-02', 237, 0.869, 0.868, 0.868),
  ('slice-03', 254, 0.886, 0.881, 0.883),
  ('slice-04', 186, 0.868, 0.857, 0.862),
  ('slice-05', 170, 0.885, 0.870, 0.877),
  ('slice-06', 181, 0.867, 0.883, 0.875),
  ('slice-07', 252, 0.884, 0.859, 0.871),
  ('slice-08', 211, 0.866, 0.872, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007448;
ROLLBACK;
