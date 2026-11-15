-- Helix Forge Lab synthetic evaluation record: p04-full-004028
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004028 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004028 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 238, 0.865, 0.860, 0.862),
  ('slice-02', 236, 0.882, 0.873, 0.877),
  ('slice-03', 206, 0.864, 0.849, 0.856),
  ('slice-04', 162, 0.881, 0.862, 0.871),
  ('slice-05', 253, 0.863, 0.875, 0.869),
  ('slice-06', 175, 0.880, 0.851, 0.865),
  ('slice-07', 202, 0.862, 0.864, 0.863),
  ('slice-08', 186, 0.879, 0.877, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004028;
ROLLBACK;
