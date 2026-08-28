-- Helix Forge Lab synthetic evaluation record: p04-full-001148
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001148 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001148 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 215, 0.882, 0.874, 0.878),
  ('slice-02', 224, 0.864, 0.850, 0.857),
  ('slice-03', 200, 0.881, 0.863, 0.872),
  ('slice-04', 206, 0.863, 0.876, 0.869),
  ('slice-05', 228, 0.880, 0.852, 0.866),
  ('slice-06', 210, 0.862, 0.865, 0.863),
  ('slice-07', 172, 0.879, 0.878, 0.878),
  ('slice-08', 171, 0.896, 0.854, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001148;
ROLLBACK;
