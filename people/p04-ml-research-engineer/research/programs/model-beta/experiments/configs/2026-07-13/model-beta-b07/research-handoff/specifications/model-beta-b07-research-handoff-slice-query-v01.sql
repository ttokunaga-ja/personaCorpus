-- Helix Forge Lab synthetic evaluation record: p04-full-005225
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005225 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005225 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 214, 0.876, 0.848, 0.862),
  ('slice-02', 176, 0.893, 0.861, 0.877),
  ('slice-03', 223, 0.875, 0.874, 0.874),
  ('slice-04', 170, 0.892, 0.850, 0.870),
  ('slice-05', 210, 0.874, 0.863, 0.868),
  ('slice-06', 248, 0.891, 0.876, 0.883),
  ('slice-07', 238, 0.873, 0.852, 0.862),
  ('slice-08', 204, 0.890, 0.865, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005225;
ROLLBACK;
