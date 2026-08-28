-- Helix Forge Lab synthetic evaluation record: p04-full-001286
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001286 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001286 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 226, 0.863, 0.865, 0.864),
  ('slice-02', 230, 0.880, 0.878, 0.879),
  ('slice-03', 250, 0.862, 0.854, 0.858),
  ('slice-04', 231, 0.879, 0.867, 0.873),
  ('slice-05', 240, 0.896, 0.880, 0.888),
  ('slice-06', 216, 0.878, 0.856, 0.867),
  ('slice-07', 175, 0.895, 0.869, 0.882),
  ('slice-08', 173, 0.877, 0.882, 0.879);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001286;
ROLLBACK;
