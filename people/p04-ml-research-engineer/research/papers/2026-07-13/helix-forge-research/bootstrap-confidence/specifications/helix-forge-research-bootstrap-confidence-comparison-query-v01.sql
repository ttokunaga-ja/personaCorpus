-- Helix Forge Lab synthetic evaluation record: p04-full-009526
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009526 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009526 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 161, 0.893, 0.871, 0.882),
  ('slice-02', 197, 0.875, 0.847, 0.861),
  ('slice-03', 186, 0.892, 0.860, 0.876),
  ('slice-04', 199, 0.874, 0.873, 0.873),
  ('slice-05', 177, 0.891, 0.849, 0.869),
  ('slice-06', 232, 0.873, 0.862, 0.867),
  ('slice-07', 230, 0.890, 0.875, 0.882),
  ('slice-08', 248, 0.872, 0.851, 0.861);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009526;
ROLLBACK;
