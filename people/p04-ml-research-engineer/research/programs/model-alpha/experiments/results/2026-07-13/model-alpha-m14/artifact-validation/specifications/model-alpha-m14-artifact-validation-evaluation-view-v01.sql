-- Helix Forge Lab synthetic evaluation record: p04-full-006720
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006720 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006720 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 160, 0.877, 0.882, 0.879),
  ('slice-02', 244, 0.894, 0.858, 0.876),
  ('slice-03', 210, 0.876, 0.871, 0.873),
  ('slice-04', 164, 0.893, 0.847, 0.869),
  ('slice-05', 207, 0.875, 0.860, 0.867),
  ('slice-06', 199, 0.892, 0.873, 0.882),
  ('slice-07', 214, 0.874, 0.849, 0.861),
  ('slice-08', 240, 0.891, 0.862, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006720;
ROLLBACK;
