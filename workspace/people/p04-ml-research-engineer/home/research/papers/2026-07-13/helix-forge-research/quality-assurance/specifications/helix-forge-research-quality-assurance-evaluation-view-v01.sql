-- Helix Forge Lab synthetic evaluation record: p04-full-002654
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002654 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002654 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 206, 0.878, 0.860, 0.869),
  ('slice-02', 220, 0.895, 0.873, 0.884),
  ('slice-03', 198, 0.877, 0.849, 0.863),
  ('slice-04', 253, 0.894, 0.862, 0.878),
  ('slice-05', 204, 0.876, 0.875, 0.875),
  ('slice-06', 198, 0.893, 0.851, 0.871),
  ('slice-07', 213, 0.875, 0.864, 0.869),
  ('slice-08', 239, 0.892, 0.877, 0.884);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002654;
ROLLBACK;
