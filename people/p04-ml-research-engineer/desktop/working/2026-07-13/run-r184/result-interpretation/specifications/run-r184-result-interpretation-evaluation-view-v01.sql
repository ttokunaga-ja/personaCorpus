-- Helix Forge Lab synthetic evaluation record: p04-full-005574
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005574 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005574 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 211, 0.878, 0.852, 0.865),
  ('slice-02', 175, 0.895, 0.865, 0.880),
  ('slice-03', 223, 0.877, 0.878, 0.877),
  ('slice-04', 218, 0.894, 0.854, 0.874),
  ('slice-05', 234, 0.876, 0.867, 0.871),
  ('slice-06', 165, 0.893, 0.880, 0.886),
  ('slice-07', 197, 0.875, 0.856, 0.865),
  ('slice-08', 184, 0.892, 0.869, 0.880);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005574;
ROLLBACK;
