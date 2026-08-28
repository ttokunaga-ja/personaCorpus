-- Helix Forge Lab synthetic evaluation record: p04-full-007429
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007429 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007429 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 194, 0.886, 0.863, 0.874),
  ('slice-02', 166, 0.868, 0.876, 0.872),
  ('slice-03', 171, 0.885, 0.852, 0.868),
  ('slice-04', 239, 0.867, 0.865, 0.866),
  ('slice-05', 244, 0.884, 0.878, 0.881),
  ('slice-06', 170, 0.866, 0.854, 0.860),
  ('slice-07', 199, 0.883, 0.867, 0.875),
  ('slice-08', 232, 0.865, 0.880, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007429;
ROLLBACK;
