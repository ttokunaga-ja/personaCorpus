-- Helix Forge Lab synthetic evaluation record: p04-full-009174
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009174 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009174 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 187, 0.869, 0.881, 0.875),
  ('slice-02', 210, 0.886, 0.857, 0.871),
  ('slice-03', 240, 0.868, 0.870, 0.869),
  ('slice-04', 179, 0.885, 0.883, 0.884),
  ('slice-05', 214, 0.867, 0.859, 0.863),
  ('slice-06', 250, 0.884, 0.872, 0.878),
  ('slice-07', 239, 0.866, 0.848, 0.857),
  ('slice-08', 205, 0.883, 0.861, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009174;
ROLLBACK;
