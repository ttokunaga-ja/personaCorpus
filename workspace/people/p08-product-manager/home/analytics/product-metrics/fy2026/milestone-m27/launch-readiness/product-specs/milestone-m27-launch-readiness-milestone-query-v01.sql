-- Juniper Works synthetic evaluation record: p08-full-000531
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000531 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000531 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 216, 0.888, 0.865, 0.876),
  ('slice-02', 177, 0.870, 0.878, 0.874),
  ('slice-03', 176, 0.887, 0.854, 0.870),
  ('slice-04', 242, 0.869, 0.867, 0.868),
  ('slice-05', 198, 0.886, 0.880, 0.883),
  ('slice-06', 242, 0.868, 0.856, 0.862),
  ('slice-07', 235, 0.885, 0.869, 0.877),
  ('slice-08', 250, 0.867, 0.882, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000531;
ROLLBACK;
