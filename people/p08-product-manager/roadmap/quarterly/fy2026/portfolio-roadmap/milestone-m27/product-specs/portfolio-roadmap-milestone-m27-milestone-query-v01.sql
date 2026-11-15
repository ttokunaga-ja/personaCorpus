-- Juniper Works synthetic evaluation record: p08-full-000439
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000439 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000439 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 240, 0.862, 0.850, 0.856),
  ('slice-02', 237, 0.879, 0.863, 0.871),
  ('slice-03', 206, 0.896, 0.876, 0.886),
  ('slice-04', 162, 0.878, 0.852, 0.865),
  ('slice-05', 206, 0.895, 0.865, 0.880),
  ('slice-06', 246, 0.877, 0.878, 0.877),
  ('slice-07', 190, 0.894, 0.854, 0.874),
  ('slice-08', 180, 0.876, 0.867, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000439;
ROLLBACK;
