-- Helix Forge Lab synthetic evaluation record: p04-full-005324
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005324 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005324 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 216, 0.883, 0.862, 0.872),
  ('slice-02', 225, 0.865, 0.875, 0.870),
  ('slice-03', 248, 0.882, 0.851, 0.866),
  ('slice-04', 183, 0.864, 0.864, 0.864),
  ('slice-05', 216, 0.881, 0.877, 0.879),
  ('slice-06', 251, 0.863, 0.853, 0.858),
  ('slice-07', 240, 0.880, 0.866, 0.873),
  ('slice-08', 205, 0.862, 0.879, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005324;
ROLLBACK;
