-- Juniper Works synthetic evaluation record: p08-full-003181
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003181 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003181 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 165, 0.882, 0.873, 0.877),
  ('slice-02', 247, 0.864, 0.849, 0.856),
  ('slice-03', 164, 0.881, 0.862, 0.871),
  ('slice-04', 188, 0.863, 0.875, 0.869),
  ('slice-05', 219, 0.880, 0.851, 0.865),
  ('slice-06', 205, 0.862, 0.864, 0.863),
  ('slice-07', 217, 0.879, 0.877, 0.878),
  ('slice-08', 241, 0.896, 0.853, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003181;
ROLLBACK;
