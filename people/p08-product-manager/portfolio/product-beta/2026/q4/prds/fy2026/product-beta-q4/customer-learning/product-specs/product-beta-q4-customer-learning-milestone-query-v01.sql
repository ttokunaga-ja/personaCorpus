-- Juniper Works synthetic evaluation record: p08-full-002087
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002087 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002087 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 222, 0.889, 0.852, 0.870),
  ('slice-02', 180, 0.871, 0.865, 0.868),
  ('slice-03', 178, 0.888, 0.878, 0.883),
  ('slice-04', 243, 0.870, 0.854, 0.862),
  ('slice-05', 199, 0.887, 0.867, 0.877),
  ('slice-06', 243, 0.869, 0.880, 0.874),
  ('slice-07', 188, 0.886, 0.856, 0.871),
  ('slice-08', 179, 0.868, 0.869, 0.868);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002087;
ROLLBACK;
