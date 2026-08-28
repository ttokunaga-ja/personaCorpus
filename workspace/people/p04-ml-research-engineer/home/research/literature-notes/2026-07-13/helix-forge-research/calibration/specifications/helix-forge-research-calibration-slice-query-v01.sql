-- Helix Forge Lab synthetic evaluation record: p04-full-002068
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002068 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002068 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 176, 0.893, 0.867, 0.880),
  ('slice-02', 205, 0.875, 0.880, 0.877),
  ('slice-03', 238, 0.892, 0.856, 0.874),
  ('slice-04', 225, 0.874, 0.869, 0.871),
  ('slice-05', 237, 0.891, 0.882, 0.886),
  ('slice-06', 214, 0.873, 0.858, 0.865),
  ('slice-07', 174, 0.890, 0.871, 0.880),
  ('slice-08', 172, 0.872, 0.847, 0.859);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002068;
ROLLBACK;
