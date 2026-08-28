-- Helix Forge Lab synthetic evaluation record: p04-full-001926
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001926 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001926 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 237, 0.874, 0.863, 0.868),
  ('slice-02', 235, 0.891, 0.876, 0.883),
  ('slice-03', 253, 0.873, 0.852, 0.862),
  ('slice-04', 185, 0.890, 0.865, 0.877),
  ('slice-05', 170, 0.872, 0.878, 0.875),
  ('slice-06', 228, 0.889, 0.854, 0.871),
  ('slice-07', 181, 0.871, 0.867, 0.869),
  ('slice-08', 223, 0.888, 0.880, 0.884);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001926;
ROLLBACK;
