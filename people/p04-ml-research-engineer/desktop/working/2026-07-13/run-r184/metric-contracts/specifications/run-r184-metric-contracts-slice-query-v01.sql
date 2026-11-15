-- Helix Forge Lab synthetic evaluation record: p04-full-001685
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001685 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001685 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 206, 0.868, 0.857, 0.862),
  ('slice-02', 172, 0.885, 0.870, 0.877),
  ('slice-03', 221, 0.867, 0.883, 0.875),
  ('slice-04', 169, 0.884, 0.859, 0.871),
  ('slice-05', 209, 0.866, 0.872, 0.869),
  ('slice-06', 248, 0.883, 0.848, 0.865),
  ('slice-07', 238, 0.865, 0.861, 0.863),
  ('slice-08', 204, 0.882, 0.874, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001685;
ROLLBACK;
