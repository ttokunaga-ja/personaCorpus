-- Helix Forge Lab synthetic evaluation record: p04-full-007825
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007825 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007825 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 196, 0.873, 0.850, 0.861),
  ('slice-02', 167, 0.890, 0.863, 0.876),
  ('slice-03', 219, 0.872, 0.876, 0.874),
  ('slice-04', 216, 0.889, 0.852, 0.870),
  ('slice-05', 185, 0.871, 0.865, 0.868),
  ('slice-06', 188, 0.888, 0.878, 0.883),
  ('slice-07', 208, 0.870, 0.854, 0.862),
  ('slice-08', 189, 0.887, 0.867, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007825;
ROLLBACK;
