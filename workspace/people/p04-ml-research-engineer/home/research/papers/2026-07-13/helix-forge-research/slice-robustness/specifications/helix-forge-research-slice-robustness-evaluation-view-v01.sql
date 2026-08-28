-- Helix Forge Lab synthetic evaluation record: p04-full-002167
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002167 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002167 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 251, 0.863, 0.874, 0.868),
  ('slice-02', 195, 0.880, 0.850, 0.865),
  ('slice-03', 233, 0.862, 0.863, 0.862),
  ('slice-04', 223, 0.879, 0.876, 0.877),
  ('slice-05', 236, 0.896, 0.852, 0.873),
  ('slice-06', 214, 0.878, 0.865, 0.871),
  ('slice-07', 221, 0.895, 0.878, 0.886),
  ('slice-08', 243, 0.877, 0.854, 0.865);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002167;
ROLLBACK;
