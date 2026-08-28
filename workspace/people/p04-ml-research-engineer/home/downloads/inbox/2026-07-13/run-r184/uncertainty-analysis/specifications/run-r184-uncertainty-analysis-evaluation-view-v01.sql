-- Helix Forge Lab synthetic evaluation record: p04-full-003041
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003041 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003041 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 173, 0.875, 0.874, 0.874),
  ('slice-02', 203, 0.892, 0.850, 0.870),
  ('slice-03', 237, 0.874, 0.863, 0.868),
  ('slice-04', 225, 0.891, 0.876, 0.883),
  ('slice-05', 190, 0.873, 0.852, 0.862),
  ('slice-06', 238, 0.890, 0.865, 0.877),
  ('slice-07', 186, 0.872, 0.878, 0.875),
  ('slice-08', 178, 0.889, 0.854, 0.871);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003041;
ROLLBACK;
