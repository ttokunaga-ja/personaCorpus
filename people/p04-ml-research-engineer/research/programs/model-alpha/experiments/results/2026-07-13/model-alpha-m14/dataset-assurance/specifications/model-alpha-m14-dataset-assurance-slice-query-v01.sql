-- Helix Forge Lab synthetic evaluation record: p04-full-001196
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001196 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001196 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 172, 0.869, 0.872, 0.870),
  ('slice-02', 250, 0.886, 0.848, 0.867),
  ('slice-03', 213, 0.868, 0.861, 0.864),
  ('slice-04', 165, 0.885, 0.874, 0.879),
  ('slice-05', 207, 0.867, 0.850, 0.858),
  ('slice-06', 247, 0.884, 0.863, 0.873),
  ('slice-07', 190, 0.866, 0.876, 0.871),
  ('slice-08', 228, 0.883, 0.852, 0.867);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001196;
ROLLBACK;
