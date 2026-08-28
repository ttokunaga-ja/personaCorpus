-- Helix Forge Lab synthetic evaluation record: p04-full-004278
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_004278 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_004278 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 162, 0.884, 0.854, 0.869),
  ('slice-02', 245, 0.866, 0.867, 0.866),
  ('slice-03', 210, 0.883, 0.880, 0.881),
  ('slice-04', 164, 0.865, 0.856, 0.860),
  ('slice-05', 207, 0.882, 0.869, 0.875),
  ('slice-06', 247, 0.864, 0.882, 0.873),
  ('slice-07', 190, 0.881, 0.858, 0.869),
  ('slice-08', 228, 0.863, 0.871, 0.867);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_004278;
ROLLBACK;
