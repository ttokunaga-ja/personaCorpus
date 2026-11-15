-- Helix Forge Lab synthetic evaluation record: p04-full-003629
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003629 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003629 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 226, 0.868, 0.851, 0.859),
  ('slice-02', 230, 0.885, 0.864, 0.874),
  ('slice-03', 203, 0.867, 0.877, 0.872),
  ('slice-04', 208, 0.884, 0.853, 0.868),
  ('slice-05', 229, 0.866, 0.866, 0.866),
  ('slice-06', 210, 0.883, 0.879, 0.881),
  ('slice-07', 172, 0.865, 0.855, 0.860),
  ('slice-08', 219, 0.882, 0.868, 0.875);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003629;
ROLLBACK;
