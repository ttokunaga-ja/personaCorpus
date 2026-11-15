-- Helix Forge Lab synthetic evaluation record: p04-full-004823
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004823 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004823 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 242, 0.894, 0.858, 0.876),
  ('slice-02', 238, 0.876, 0.871, 0.873),
  ('slice-03', 207, 0.893, 0.847, 0.869),
  ('slice-04', 162, 0.875, 0.860, 0.867),
  ('slice-05', 253, 0.892, 0.873, 0.882),
  ('slice-06', 222, 0.874, 0.849, 0.861),
  ('slice-07', 178, 0.891, 0.862, 0.876),
  ('slice-08', 174, 0.873, 0.875, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004823;
ROLLBACK;
