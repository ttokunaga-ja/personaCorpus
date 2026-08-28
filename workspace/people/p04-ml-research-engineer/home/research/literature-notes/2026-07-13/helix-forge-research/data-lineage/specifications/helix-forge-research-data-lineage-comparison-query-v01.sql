-- Helix Forge Lab synthetic evaluation record: p04-full-008475
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008475 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008475 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 167, 0.889, 0.869, 0.879),
  ('slice-02', 248, 0.871, 0.882, 0.876),
  ('slice-03', 212, 0.888, 0.858, 0.873),
  ('slice-04', 212, 0.870, 0.871, 0.870),
  ('slice-05', 231, 0.887, 0.847, 0.867),
  ('slice-06', 211, 0.869, 0.860, 0.864),
  ('slice-07', 220, 0.886, 0.873, 0.879),
  ('slice-08', 243, 0.868, 0.849, 0.858);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008475;
ROLLBACK;
