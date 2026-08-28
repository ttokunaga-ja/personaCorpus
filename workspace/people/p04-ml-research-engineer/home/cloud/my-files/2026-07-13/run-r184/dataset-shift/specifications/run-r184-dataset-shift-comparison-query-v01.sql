-- Helix Forge Lab synthetic evaluation record: p04-full-005424
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005424 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005424 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 198, 0.885, 0.876, 0.880),
  ('slice-02', 168, 0.867, 0.852, 0.859),
  ('slice-03', 219, 0.884, 0.865, 0.874),
  ('slice-04', 168, 0.866, 0.878, 0.872),
  ('slice-05', 209, 0.883, 0.854, 0.868),
  ('slice-06', 200, 0.865, 0.867, 0.866),
  ('slice-07', 167, 0.882, 0.880, 0.881),
  ('slice-08', 169, 0.864, 0.856, 0.860);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005424;
ROLLBACK;
