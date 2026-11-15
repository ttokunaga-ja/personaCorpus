-- Helix Forge Lab synthetic evaluation record: p04-full-004127
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004127 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004127 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 246, 0.888, 0.874, 0.881),
  ('slice-02', 192, 0.870, 0.850, 0.860),
  ('slice-03', 184, 0.887, 0.863, 0.875),
  ('slice-04', 246, 0.869, 0.876, 0.872),
  ('slice-05', 200, 0.886, 0.852, 0.869),
  ('slice-06', 196, 0.868, 0.865, 0.866),
  ('slice-07', 165, 0.885, 0.878, 0.881),
  ('slice-08', 215, 0.867, 0.854, 0.860);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004127;
ROLLBACK;
