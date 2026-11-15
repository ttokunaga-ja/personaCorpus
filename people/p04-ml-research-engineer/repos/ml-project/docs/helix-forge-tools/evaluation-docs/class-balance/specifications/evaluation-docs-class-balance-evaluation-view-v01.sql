-- Helix Forge Lab synthetic evaluation record: p04-full-003283
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003283 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003283 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 218, 0.880, 0.875, 0.877),
  ('slice-02', 226, 0.862, 0.851, 0.856),
  ('slice-03', 201, 0.879, 0.864, 0.871),
  ('slice-04', 207, 0.896, 0.877, 0.886),
  ('slice-05', 181, 0.878, 0.853, 0.865),
  ('slice-06', 234, 0.895, 0.866, 0.880),
  ('slice-07', 184, 0.877, 0.879, 0.878),
  ('slice-08', 177, 0.894, 0.855, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003283;
ROLLBACK;
