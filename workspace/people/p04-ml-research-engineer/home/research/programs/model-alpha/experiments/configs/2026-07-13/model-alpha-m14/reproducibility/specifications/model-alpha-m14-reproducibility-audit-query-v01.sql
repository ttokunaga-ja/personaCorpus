-- Helix Forge Lab synthetic evaluation record: p04-full-008720
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008720 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008720 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 246, 0.863, 0.858, 0.860),
  ('slice-02', 240, 0.880, 0.871, 0.875),
  ('slice-03', 208, 0.862, 0.847, 0.854),
  ('slice-04', 163, 0.879, 0.860, 0.869),
  ('slice-05', 206, 0.896, 0.873, 0.884),
  ('slice-06', 246, 0.878, 0.849, 0.863),
  ('slice-07', 190, 0.895, 0.862, 0.878),
  ('slice-08', 228, 0.877, 0.875, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008720;
ROLLBACK;
