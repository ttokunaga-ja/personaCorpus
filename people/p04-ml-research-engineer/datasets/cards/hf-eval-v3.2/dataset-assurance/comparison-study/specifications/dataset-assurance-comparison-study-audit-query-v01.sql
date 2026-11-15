-- Helix Forge Lab synthetic evaluation record: p04-full-003971
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003971 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003971 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 207, 0.864, 0.868, 0.866),
  ('slice-02', 173, 0.881, 0.881, 0.881),
  ('slice-03', 222, 0.863, 0.857, 0.860),
  ('slice-04', 217, 0.880, 0.870, 0.875),
  ('slice-05', 233, 0.862, 0.883, 0.872),
  ('slice-06', 212, 0.879, 0.859, 0.869),
  ('slice-07', 220, 0.896, 0.872, 0.884),
  ('slice-08', 243, 0.878, 0.848, 0.863);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003971;
ROLLBACK;
