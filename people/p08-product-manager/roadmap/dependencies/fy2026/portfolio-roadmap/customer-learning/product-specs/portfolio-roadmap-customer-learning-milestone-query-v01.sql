-- Juniper Works synthetic evaluation record: p08-full-003273
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003273 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003273 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 186, 0.873, 0.852, 0.862),
  ('slice-02', 162, 0.890, 0.865, 0.877),
  ('slice-03', 216, 0.872, 0.878, 0.875),
  ('slice-04', 167, 0.889, 0.854, 0.871),
  ('slice-05', 208, 0.871, 0.867, 0.869),
  ('slice-06', 200, 0.888, 0.880, 0.884),
  ('slice-07', 167, 0.870, 0.856, 0.863),
  ('slice-08', 216, 0.887, 0.869, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003273;
ROLLBACK;
