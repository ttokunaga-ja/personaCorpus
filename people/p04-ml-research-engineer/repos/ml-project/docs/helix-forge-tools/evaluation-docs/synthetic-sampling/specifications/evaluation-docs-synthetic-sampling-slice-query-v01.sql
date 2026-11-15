-- Helix Forge Lab synthetic evaluation record: p04-full-002554
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002554 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002554 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 214, 0.896, 0.883, 0.889),
  ('slice-02', 176, 0.878, 0.859, 0.868),
  ('slice-03', 223, 0.895, 0.872, 0.883),
  ('slice-04', 218, 0.877, 0.848, 0.862),
  ('slice-05', 186, 0.894, 0.861, 0.877),
  ('slice-06', 189, 0.876, 0.874, 0.875),
  ('slice-07', 209, 0.893, 0.850, 0.871),
  ('slice-08', 237, 0.875, 0.863, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002554;
ROLLBACK;
