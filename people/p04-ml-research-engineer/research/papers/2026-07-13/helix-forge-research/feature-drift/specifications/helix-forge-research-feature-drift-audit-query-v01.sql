-- Helix Forge Lab synthetic evaluation record: p04-full-006776
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_006776 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_006776 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 180, 0.867, 0.863, 0.865),
  ('slice-02', 207, 0.884, 0.876, 0.880),
  ('slice-03', 239, 0.866, 0.852, 0.859),
  ('slice-04', 178, 0.883, 0.865, 0.874),
  ('slice-05', 166, 0.865, 0.878, 0.871),
  ('slice-06', 179, 0.882, 0.854, 0.868),
  ('slice-07', 204, 0.864, 0.867, 0.865),
  ('slice-08', 187, 0.881, 0.880, 0.880);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_006776;
ROLLBACK;
