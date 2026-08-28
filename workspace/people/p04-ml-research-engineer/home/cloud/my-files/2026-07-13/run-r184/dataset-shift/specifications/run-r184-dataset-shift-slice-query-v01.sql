-- Helix Forge Lab synthetic evaluation record: p04-full-003826
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003826 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003826 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 249, 0.871, 0.847, 0.859),
  ('slice-02', 241, 0.888, 0.860, 0.874),
  ('slice-03', 161, 0.870, 0.873, 0.871),
  ('slice-04', 187, 0.887, 0.849, 0.868),
  ('slice-05', 171, 0.869, 0.862, 0.865),
  ('slice-06', 229, 0.886, 0.875, 0.880),
  ('slice-07', 181, 0.868, 0.851, 0.859),
  ('slice-08', 176, 0.885, 0.864, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003826;
ROLLBACK;
