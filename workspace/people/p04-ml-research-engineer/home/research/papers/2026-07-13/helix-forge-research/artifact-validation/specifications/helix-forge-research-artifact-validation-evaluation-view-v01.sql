-- Helix Forge Lab synthetic evaluation record: p04-full-003778
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003778 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003778 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 209, 0.886, 0.870, 0.878),
  ('slice-02', 221, 0.868, 0.883, 0.875),
  ('slice-03', 246, 0.885, 0.859, 0.872),
  ('slice-04', 229, 0.867, 0.872, 0.869),
  ('slice-05', 192, 0.884, 0.848, 0.866),
  ('slice-06', 192, 0.866, 0.861, 0.863),
  ('slice-07', 163, 0.883, 0.874, 0.878),
  ('slice-08', 214, 0.865, 0.850, 0.857);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003778;
ROLLBACK;
