-- Helix Forge Lab synthetic evaluation record: p04-full-003528
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003528 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003528 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 244, 0.881, 0.882, 0.881),
  ('slice-02', 239, 0.863, 0.858, 0.860),
  ('slice-03', 160, 0.880, 0.871, 0.875),
  ('slice-04', 186, 0.862, 0.847, 0.854),
  ('slice-05', 170, 0.879, 0.860, 0.869),
  ('slice-06', 228, 0.896, 0.873, 0.884),
  ('slice-07', 228, 0.878, 0.849, 0.863),
  ('slice-08', 247, 0.895, 0.862, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003528;
ROLLBACK;
