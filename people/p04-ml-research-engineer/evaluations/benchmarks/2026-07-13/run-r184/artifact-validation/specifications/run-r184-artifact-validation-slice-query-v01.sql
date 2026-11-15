-- Helix Forge Lab synthetic evaluation record: p04-full-001440
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001440 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001440 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 191, 0.878, 0.880, 0.879),
  ('slice-02', 212, 0.895, 0.856, 0.875),
  ('slice-03', 194, 0.877, 0.869, 0.873),
  ('slice-04', 203, 0.894, 0.882, 0.888),
  ('slice-05', 226, 0.876, 0.858, 0.867),
  ('slice-06', 209, 0.893, 0.871, 0.882),
  ('slice-07', 219, 0.875, 0.847, 0.861),
  ('slice-08', 242, 0.892, 0.860, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001440;
ROLLBACK;
