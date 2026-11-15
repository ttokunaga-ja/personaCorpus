-- Helix Forge Lab synthetic evaluation record: p04-full-007920
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_007920 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_007920 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 180, 0.892, 0.865, 0.878),
  ('slice-02', 207, 0.874, 0.878, 0.876),
  ('slice-03', 191, 0.891, 0.854, 0.872),
  ('slice-04', 249, 0.873, 0.867, 0.870),
  ('slice-05', 249, 0.890, 0.880, 0.885),
  ('slice-06', 220, 0.872, 0.856, 0.864),
  ('slice-07', 177, 0.889, 0.869, 0.879),
  ('slice-08', 174, 0.871, 0.882, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_007920;
ROLLBACK;
