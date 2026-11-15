-- Juniper Works synthetic evaluation record: p08-full-004156
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004156 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004156 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 193, 0.890, 0.874, 0.882),
  ('slice-02', 166, 0.872, 0.850, 0.861),
  ('slice-03', 171, 0.889, 0.863, 0.876),
  ('slice-04', 192, 0.871, 0.876, 0.873),
  ('slice-05', 221, 0.888, 0.852, 0.870),
  ('slice-06', 206, 0.870, 0.865, 0.867),
  ('slice-07', 170, 0.887, 0.878, 0.882),
  ('slice-08', 218, 0.869, 0.854, 0.861);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004156;
ROLLBACK;
