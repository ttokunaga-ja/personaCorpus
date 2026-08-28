-- Helix Forge Lab synthetic evaluation record: p04-full-006969
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006969 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006969 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 242, 0.879, 0.848, 0.863),
  ('slice-02', 238, 0.896, 0.861, 0.878),
  ('slice-03', 207, 0.878, 0.874, 0.876),
  ('slice-04', 210, 0.895, 0.850, 0.872),
  ('slice-05', 230, 0.877, 0.863, 0.870),
  ('slice-06', 211, 0.894, 0.876, 0.885),
  ('slice-07', 172, 0.876, 0.852, 0.864),
  ('slice-08', 171, 0.893, 0.865, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006969;
ROLLBACK;
