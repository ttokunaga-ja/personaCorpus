-- Helix Forge Lab synthetic evaluation record: p04-full-007026
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007026 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007026 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 218, 0.885, 0.880, 0.882),
  ('slice-02', 226, 0.867, 0.856, 0.861),
  ('slice-03', 201, 0.884, 0.869, 0.876),
  ('slice-04', 254, 0.866, 0.882, 0.874),
  ('slice-05', 204, 0.883, 0.858, 0.870),
  ('slice-06', 245, 0.865, 0.871, 0.868),
  ('slice-07', 237, 0.882, 0.847, 0.864),
  ('slice-08', 204, 0.864, 0.860, 0.862);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007026;
ROLLBACK;
