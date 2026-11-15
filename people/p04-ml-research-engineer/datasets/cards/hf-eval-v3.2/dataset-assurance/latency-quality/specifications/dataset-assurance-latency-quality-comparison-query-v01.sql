-- Helix Forge Lab synthetic evaluation record: p04-full-002011
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002011 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002011 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 174, 0.896, 0.856, 0.876),
  ('slice-02', 204, 0.878, 0.869, 0.873),
  ('slice-03', 237, 0.895, 0.882, 0.888),
  ('slice-04', 225, 0.877, 0.858, 0.867),
  ('slice-05', 190, 0.894, 0.871, 0.882),
  ('slice-06', 191, 0.876, 0.847, 0.861),
  ('slice-07', 210, 0.893, 0.860, 0.876),
  ('slice-08', 190, 0.875, 0.873, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002011;
ROLLBACK;
