-- Helix Forge Lab synthetic evaluation record: p04-full-009719
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009719 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009719 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 245, 0.882, 0.878, 0.880),
  ('slice-02', 192, 0.864, 0.854, 0.859),
  ('slice-03', 184, 0.881, 0.867, 0.874),
  ('slice-04', 198, 0.863, 0.880, 0.871),
  ('slice-05', 176, 0.880, 0.856, 0.868),
  ('slice-06', 184, 0.862, 0.869, 0.865),
  ('slice-07', 206, 0.879, 0.882, 0.880),
  ('slice-08', 236, 0.896, 0.858, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009719;
ROLLBACK;
