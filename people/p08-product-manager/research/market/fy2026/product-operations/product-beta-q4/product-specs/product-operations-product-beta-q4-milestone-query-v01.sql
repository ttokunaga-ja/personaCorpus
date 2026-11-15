-- Juniper Works synthetic evaluation record: p08-full-007647
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007647 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007647 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 215, 0.872, 0.855, 0.863),
  ('slice-02', 177, 0.889, 0.868, 0.878),
  ('slice-03', 176, 0.871, 0.881, 0.876),
  ('slice-04', 194, 0.888, 0.857, 0.872),
  ('slice-05', 222, 0.870, 0.870, 0.870),
  ('slice-06', 254, 0.887, 0.883, 0.885),
  ('slice-07', 241, 0.869, 0.859, 0.864),
  ('slice-08', 253, 0.886, 0.872, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007647;
ROLLBACK;
