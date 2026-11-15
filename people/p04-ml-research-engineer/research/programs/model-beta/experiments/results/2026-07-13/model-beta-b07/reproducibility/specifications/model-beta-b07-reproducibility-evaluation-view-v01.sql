-- Helix Forge Lab synthetic evaluation record: p04-full-001056
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001056 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001056 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 222, 0.879, 0.863, 0.871),
  ('slice-02', 180, 0.896, 0.876, 0.886),
  ('slice-03', 225, 0.878, 0.852, 0.865),
  ('slice-04', 171, 0.895, 0.865, 0.880),
  ('slice-05', 163, 0.877, 0.878, 0.877),
  ('slice-06', 177, 0.894, 0.854, 0.874),
  ('slice-07', 203, 0.876, 0.867, 0.871),
  ('slice-08', 234, 0.893, 0.880, 0.886);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001056;
ROLLBACK;
