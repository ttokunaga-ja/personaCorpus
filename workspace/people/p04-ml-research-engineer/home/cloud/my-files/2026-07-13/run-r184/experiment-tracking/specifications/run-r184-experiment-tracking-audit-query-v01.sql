-- Helix Forge Lab synthetic evaluation record: p04-full-001342
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001342 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001342 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 207, 0.894, 0.865, 0.879),
  ('slice-02', 173, 0.876, 0.878, 0.877),
  ('slice-03', 174, 0.893, 0.854, 0.873),
  ('slice-04', 193, 0.875, 0.867, 0.871),
  ('slice-05', 221, 0.892, 0.880, 0.886),
  ('slice-06', 206, 0.874, 0.856, 0.865),
  ('slice-07', 217, 0.891, 0.869, 0.880),
  ('slice-08', 241, 0.873, 0.882, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001342;
ROLLBACK;
