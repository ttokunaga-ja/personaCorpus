-- Helix Forge Lab synthetic evaluation record: p04-full-004920
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004920 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004920 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 226, 0.888, 0.876, 0.882),
  ('slice-02', 230, 0.870, 0.852, 0.861),
  ('slice-03', 250, 0.887, 0.865, 0.876),
  ('slice-04', 231, 0.869, 0.878, 0.873),
  ('slice-05', 193, 0.886, 0.854, 0.870),
  ('slice-06', 192, 0.868, 0.867, 0.867),
  ('slice-07', 163, 0.885, 0.880, 0.882),
  ('slice-08', 214, 0.867, 0.856, 0.861);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004920;
ROLLBACK;
