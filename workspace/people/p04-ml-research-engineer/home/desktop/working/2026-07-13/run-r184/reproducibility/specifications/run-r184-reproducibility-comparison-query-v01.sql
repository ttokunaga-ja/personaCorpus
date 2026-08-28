-- Helix Forge Lab synthetic evaluation record: p04-full-006574
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006574 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006574 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 225, 0.882, 0.852, 0.867),
  ('slice-02', 182, 0.864, 0.865, 0.864),
  ('slice-03', 179, 0.881, 0.878, 0.879),
  ('slice-04', 243, 0.863, 0.854, 0.858),
  ('slice-05', 199, 0.880, 0.867, 0.873),
  ('slice-06', 195, 0.862, 0.880, 0.871),
  ('slice-07', 164, 0.879, 0.856, 0.867),
  ('slice-08', 215, 0.896, 0.869, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006574;
ROLLBACK;
