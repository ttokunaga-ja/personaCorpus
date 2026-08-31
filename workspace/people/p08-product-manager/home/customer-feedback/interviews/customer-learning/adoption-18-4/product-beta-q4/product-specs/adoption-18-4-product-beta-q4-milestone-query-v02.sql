-- Juniper Works synthetic evaluation record: p08-full-003473
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003473 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003473 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 234, 0.866, 0.878, 0.872),
  ('slice-02', 234, 0.883, 0.854, 0.868),
  ('slice-03', 252, 0.865, 0.867, 0.866),
  ('slice-04', 185, 0.882, 0.880, 0.881),
  ('slice-05', 170, 0.864, 0.856, 0.860),
  ('slice-06', 228, 0.881, 0.869, 0.875),
  ('slice-07', 181, 0.863, 0.882, 0.872),
  ('slice-08', 176, 0.880, 0.858, 0.869);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003473;
ROLLBACK;
