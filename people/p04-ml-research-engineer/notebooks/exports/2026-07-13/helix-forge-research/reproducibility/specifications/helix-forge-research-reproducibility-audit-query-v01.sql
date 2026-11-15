-- Helix Forge Lab synthetic evaluation record: p04-full-002603
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002603 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002603 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 173, 0.870, 0.853, 0.861),
  ('slice-02', 251, 0.887, 0.866, 0.876),
  ('slice-03', 166, 0.869, 0.879, 0.874),
  ('slice-04', 237, 0.886, 0.855, 0.870),
  ('slice-05', 196, 0.868, 0.868, 0.868),
  ('slice-06', 241, 0.885, 0.881, 0.883),
  ('slice-07', 187, 0.867, 0.857, 0.862),
  ('slice-08', 226, 0.884, 0.870, 0.877);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002603;
ROLLBACK;
