-- Helix Forge Lab synthetic evaluation record: p04-full-007878
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007878 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007878 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 253, 0.880, 0.851, 0.865),
  ('slice-02', 196, 0.862, 0.864, 0.863),
  ('slice-03', 186, 0.879, 0.877, 0.878),
  ('slice-04', 199, 0.896, 0.853, 0.874),
  ('slice-05', 177, 0.878, 0.866, 0.872),
  ('slice-06', 184, 0.895, 0.879, 0.887),
  ('slice-07', 206, 0.877, 0.855, 0.866),
  ('slice-08', 188, 0.894, 0.868, 0.881);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007878;
ROLLBACK;
