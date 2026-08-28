-- Helix Forge Lab synthetic evaluation record: p04-full-002845
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002845 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002845 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 229, 0.896, 0.851, 0.873),
  ('slice-02', 231, 0.878, 0.864, 0.871),
  ('slice-03', 203, 0.895, 0.877, 0.886),
  ('slice-04', 160, 0.877, 0.853, 0.865),
  ('slice-05', 205, 0.894, 0.866, 0.880),
  ('slice-06', 246, 0.876, 0.879, 0.877),
  ('slice-07', 237, 0.893, 0.855, 0.874),
  ('slice-08', 251, 0.875, 0.868, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002845;
ROLLBACK;
