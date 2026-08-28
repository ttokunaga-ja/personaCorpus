-- Helix Forge Lab synthetic evaluation record: p04-full-000962
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_000962 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_000962 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 191, 0.893, 0.883, 0.888),
  ('slice-02', 212, 0.875, 0.859, 0.867),
  ('slice-03', 241, 0.892, 0.872, 0.882),
  ('slice-04', 227, 0.874, 0.848, 0.861),
  ('slice-05', 191, 0.891, 0.861, 0.876),
  ('slice-06', 191, 0.873, 0.874, 0.873),
  ('slice-07', 162, 0.890, 0.850, 0.870),
  ('slice-08', 166, 0.872, 0.863, 0.867);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_000962;
ROLLBACK;
