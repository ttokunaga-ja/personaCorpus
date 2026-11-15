-- Helix Forge Lab synthetic evaluation record: p04-full-006523
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006523 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006523 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 175, 0.862, 0.854, 0.858),
  ('slice-02', 252, 0.879, 0.867, 0.873),
  ('slice-03', 214, 0.896, 0.880, 0.888),
  ('slice-04', 213, 0.878, 0.856, 0.867),
  ('slice-05', 184, 0.895, 0.869, 0.882),
  ('slice-06', 188, 0.877, 0.882, 0.879),
  ('slice-07', 161, 0.894, 0.858, 0.876),
  ('slice-08', 166, 0.876, 0.871, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006523;
ROLLBACK;
