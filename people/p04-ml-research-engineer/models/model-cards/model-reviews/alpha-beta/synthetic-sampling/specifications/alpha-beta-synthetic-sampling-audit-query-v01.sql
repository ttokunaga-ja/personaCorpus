-- Helix Forge Lab synthetic evaluation record: p04-full-009470
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009470 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009470 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 196, 0.868, 0.872, 0.870),
  ('slice-02', 167, 0.885, 0.848, 0.866),
  ('slice-03', 171, 0.867, 0.861, 0.864),
  ('slice-04', 239, 0.884, 0.874, 0.879),
  ('slice-05', 244, 0.866, 0.850, 0.858),
  ('slice-06', 218, 0.883, 0.863, 0.873),
  ('slice-07', 223, 0.865, 0.876, 0.870),
  ('slice-08', 197, 0.882, 0.852, 0.867);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009470;
ROLLBACK;
