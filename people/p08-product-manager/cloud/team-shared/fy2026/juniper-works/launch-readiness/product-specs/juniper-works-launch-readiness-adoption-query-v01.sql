-- Juniper Works synthetic evaluation record: p08-full-001388
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001388 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001388 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 193, 0.865, 0.873, 0.869),
  ('slice-02', 166, 0.882, 0.849, 0.865),
  ('slice-03', 218, 0.864, 0.862, 0.863),
  ('slice-04', 215, 0.881, 0.875, 0.878),
  ('slice-05', 232, 0.863, 0.851, 0.857),
  ('slice-06', 164, 0.880, 0.864, 0.872),
  ('slice-07', 244, 0.862, 0.877, 0.869),
  ('slice-08', 160, 0.879, 0.853, 0.866);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001388;
ROLLBACK;
