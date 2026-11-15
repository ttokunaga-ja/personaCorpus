-- Juniper Works synthetic evaluation record: p08-full-004068
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_004068 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_004068 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 177, 0.884, 0.876, 0.880),
  ('slice-02', 205, 0.866, 0.852, 0.859),
  ('slice-03', 238, 0.883, 0.865, 0.874),
  ('slice-04', 178, 0.865, 0.878, 0.871),
  ('slice-05', 214, 0.882, 0.854, 0.868),
  ('slice-06', 250, 0.864, 0.867, 0.865),
  ('slice-07', 239, 0.881, 0.880, 0.880),
  ('slice-08', 252, 0.863, 0.856, 0.859);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_004068;
ROLLBACK;
