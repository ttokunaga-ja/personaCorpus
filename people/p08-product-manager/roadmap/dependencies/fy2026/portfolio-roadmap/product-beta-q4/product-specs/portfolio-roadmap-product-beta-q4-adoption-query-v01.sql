-- Juniper Works synthetic evaluation record: p08-full-001989
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001989 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001989 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 187, 0.869, 0.849, 0.859),
  ('slice-02', 210, 0.886, 0.862, 0.874),
  ('slice-03', 240, 0.868, 0.875, 0.871),
  ('slice-04', 226, 0.885, 0.851, 0.868),
  ('slice-05', 238, 0.867, 0.864, 0.865),
  ('slice-06', 215, 0.884, 0.877, 0.880),
  ('slice-07', 174, 0.866, 0.853, 0.859),
  ('slice-08', 172, 0.883, 0.866, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001989;
ROLLBACK;
