-- Helix Forge Lab synthetic evaluation record: p04-full-004377
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004377 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004377 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 183, 0.875, 0.881, 0.878),
  ('slice-02', 208, 0.892, 0.857, 0.874),
  ('slice-03', 192, 0.874, 0.870, 0.872),
  ('slice-04', 250, 0.891, 0.883, 0.887),
  ('slice-05', 202, 0.873, 0.859, 0.866),
  ('slice-06', 244, 0.890, 0.872, 0.881),
  ('slice-07', 236, 0.872, 0.848, 0.860),
  ('slice-08', 251, 0.889, 0.861, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004377;
ROLLBACK;
