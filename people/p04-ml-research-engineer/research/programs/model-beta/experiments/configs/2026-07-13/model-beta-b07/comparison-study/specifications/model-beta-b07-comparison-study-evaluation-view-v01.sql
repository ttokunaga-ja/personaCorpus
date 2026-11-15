-- Helix Forge Lab synthetic evaluation record: p04-full-005825
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005825 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005825 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 250, 0.877, 0.869, 0.873),
  ('slice-02', 242, 0.894, 0.882, 0.888),
  ('slice-03', 161, 0.876, 0.858, 0.867),
  ('slice-04', 187, 0.893, 0.871, 0.882),
  ('slice-05', 218, 0.875, 0.847, 0.861),
  ('slice-06', 205, 0.892, 0.860, 0.876),
  ('slice-07', 169, 0.874, 0.873, 0.873),
  ('slice-08', 170, 0.891, 0.849, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005825;
ROLLBACK;
