-- Helix Forge Lab synthetic evaluation record: p04-full-006877
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006877 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006877 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 245, 0.877, 0.874, 0.875),
  ('slice-02', 239, 0.894, 0.850, 0.871),
  ('slice-03', 160, 0.876, 0.863, 0.869),
  ('slice-04', 234, 0.893, 0.876, 0.884),
  ('slice-05', 194, 0.875, 0.852, 0.863),
  ('slice-06', 193, 0.892, 0.865, 0.878),
  ('slice-07', 211, 0.874, 0.878, 0.876),
  ('slice-08', 191, 0.891, 0.854, 0.872);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006877;
ROLLBACK;
