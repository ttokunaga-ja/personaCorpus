-- Helix Forge Lab synthetic evaluation record: p04-full-009126
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_009126 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_009126 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 206, 0.868, 0.852, 0.860),
  ('slice-02', 220, 0.885, 0.865, 0.875),
  ('slice-03', 245, 0.867, 0.878, 0.872),
  ('slice-04', 181, 0.884, 0.854, 0.869),
  ('slice-05', 168, 0.866, 0.867, 0.866),
  ('slice-06', 227, 0.883, 0.880, 0.881),
  ('slice-07', 180, 0.865, 0.856, 0.860),
  ('slice-08', 223, 0.882, 0.869, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_009126;
ROLLBACK;
