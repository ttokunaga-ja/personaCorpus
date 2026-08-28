-- Helix Forge Lab synthetic evaluation record: p04-full-001105
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001105 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001105 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 254, 0.896, 0.860, 0.878),
  ('slice-02', 244, 0.878, 0.873, 0.875),
  ('slice-03', 162, 0.895, 0.849, 0.871),
  ('slice-04', 187, 0.877, 0.862, 0.869),
  ('slice-05', 171, 0.894, 0.875, 0.884),
  ('slice-06', 229, 0.876, 0.851, 0.863),
  ('slice-07', 229, 0.893, 0.864, 0.878),
  ('slice-08', 200, 0.875, 0.877, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001105;
ROLLBACK;
