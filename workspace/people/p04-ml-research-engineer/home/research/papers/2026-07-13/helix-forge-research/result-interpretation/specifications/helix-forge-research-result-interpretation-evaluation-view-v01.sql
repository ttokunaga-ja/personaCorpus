-- Helix Forge Lab synthetic evaluation record: p04-full-008776
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008776 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008776 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 217, 0.894, 0.868, 0.881),
  ('slice-02', 178, 0.876, 0.881, 0.878),
  ('slice-03', 224, 0.893, 0.857, 0.875),
  ('slice-04', 218, 0.875, 0.870, 0.872),
  ('slice-05', 234, 0.892, 0.883, 0.887),
  ('slice-06', 165, 0.874, 0.859, 0.866),
  ('slice-07', 244, 0.891, 0.872, 0.881),
  ('slice-08', 207, 0.873, 0.848, 0.860);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008776;
ROLLBACK;
