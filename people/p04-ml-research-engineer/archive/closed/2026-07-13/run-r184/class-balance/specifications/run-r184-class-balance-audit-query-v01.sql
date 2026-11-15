-- Helix Forge Lab synthetic evaluation record: p04-full-008074
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_008074 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_008074 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 162, 0.879, 0.854, 0.866),
  ('slice-02', 245, 0.896, 0.867, 0.881),
  ('slice-03', 163, 0.878, 0.880, 0.879),
  ('slice-04', 235, 0.895, 0.856, 0.875),
  ('slice-05', 242, 0.877, 0.869, 0.873),
  ('slice-06', 169, 0.894, 0.882, 0.888),
  ('slice-07', 199, 0.876, 0.858, 0.867),
  ('slice-08', 185, 0.893, 0.871, 0.882);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_008074;
ROLLBACK;
