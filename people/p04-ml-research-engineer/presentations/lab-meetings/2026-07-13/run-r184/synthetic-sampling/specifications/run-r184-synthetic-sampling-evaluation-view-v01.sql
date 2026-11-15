-- Helix Forge Lab synthetic evaluation record: p04-full-001533
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_001533 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_001533 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 233, 0.875, 0.882, 0.878),
  ('slice-02', 186, 0.892, 0.858, 0.875),
  ('slice-03', 181, 0.874, 0.871, 0.872),
  ('slice-04', 244, 0.891, 0.847, 0.868),
  ('slice-05', 247, 0.873, 0.860, 0.866),
  ('slice-06', 219, 0.890, 0.873, 0.881),
  ('slice-07', 224, 0.872, 0.849, 0.860),
  ('slice-08', 197, 0.889, 0.862, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_001533;
ROLLBACK;
