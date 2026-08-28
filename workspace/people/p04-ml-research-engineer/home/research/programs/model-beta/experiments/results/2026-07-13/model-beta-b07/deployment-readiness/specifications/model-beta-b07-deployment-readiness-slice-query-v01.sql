-- Helix Forge Lab synthetic evaluation record: p04-full-003178
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003178 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003178 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 202, 0.879, 0.857, 0.868),
  ('slice-02', 218, 0.896, 0.870, 0.883),
  ('slice-03', 244, 0.878, 0.883, 0.880),
  ('slice-04', 228, 0.895, 0.859, 0.877),
  ('slice-05', 239, 0.877, 0.872, 0.874),
  ('slice-06', 215, 0.894, 0.848, 0.870),
  ('slice-07', 174, 0.876, 0.861, 0.868),
  ('slice-08', 220, 0.893, 0.874, 0.883);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003178;
ROLLBACK;
