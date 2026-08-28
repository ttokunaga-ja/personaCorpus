-- Helix Forge Lab synthetic evaluation record: p04-full-008877
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008877 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008877 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 171, 0.888, 0.852, 0.870),
  ('slice-02', 250, 0.870, 0.865, 0.867),
  ('slice-03', 165, 0.887, 0.878, 0.882),
  ('slice-04', 236, 0.869, 0.854, 0.861),
  ('slice-05', 243, 0.886, 0.867, 0.876),
  ('slice-06', 217, 0.868, 0.880, 0.874),
  ('slice-07', 175, 0.885, 0.856, 0.870),
  ('slice-08', 173, 0.867, 0.869, 0.868);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008877;
ROLLBACK;
