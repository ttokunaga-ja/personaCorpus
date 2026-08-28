-- Helix Forge Lab synthetic evaluation record: p04-full-001873
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001873 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001873 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 215, 0.872, 0.848, 0.860),
  ('slice-02', 177, 0.889, 0.861, 0.875),
  ('slice-03', 224, 0.871, 0.874, 0.872),
  ('slice-04', 218, 0.888, 0.850, 0.869),
  ('slice-05', 234, 0.870, 0.863, 0.866),
  ('slice-06', 165, 0.887, 0.876, 0.881),
  ('slice-07', 244, 0.869, 0.852, 0.860),
  ('slice-08', 207, 0.886, 0.865, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001873;
ROLLBACK;
