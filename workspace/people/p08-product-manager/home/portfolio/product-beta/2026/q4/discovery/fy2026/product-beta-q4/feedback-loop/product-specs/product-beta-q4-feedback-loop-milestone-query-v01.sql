-- Juniper Works synthetic evaluation record: p08-full-002786
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002786 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002786 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 240, 0.882, 0.869, 0.875),
  ('slice-02', 189, 0.864, 0.882, 0.873),
  ('slice-03', 182, 0.881, 0.858, 0.869),
  ('slice-04', 245, 0.863, 0.871, 0.867),
  ('slice-05', 247, 0.880, 0.847, 0.863),
  ('slice-06', 219, 0.862, 0.860, 0.861),
  ('slice-07', 224, 0.879, 0.873, 0.876),
  ('slice-08', 245, 0.896, 0.849, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002786;
ROLLBACK;
