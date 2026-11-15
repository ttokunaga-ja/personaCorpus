-- Helix Forge Lab synthetic evaluation record: p04-full-009325
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009325 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009325 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 199, 0.896, 0.872, 0.884),
  ('slice-02', 169, 0.878, 0.848, 0.863),
  ('slice-03', 172, 0.895, 0.861, 0.878),
  ('slice-04', 192, 0.877, 0.874, 0.875),
  ('slice-05', 221, 0.894, 0.850, 0.871),
  ('slice-06', 254, 0.876, 0.863, 0.869),
  ('slice-07', 241, 0.893, 0.876, 0.884),
  ('slice-08', 206, 0.875, 0.852, 0.863);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009325;
ROLLBACK;
