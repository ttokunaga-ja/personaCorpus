-- Juniper Works synthetic evaluation record: p08-full-007341
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_007341 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_007341 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 210, 0.887, 0.863, 0.875),
  ('slice-02', 174, 0.869, 0.876, 0.872),
  ('slice-03', 175, 0.886, 0.852, 0.869),
  ('slice-04', 194, 0.868, 0.865, 0.866),
  ('slice-05', 174, 0.885, 0.878, 0.881),
  ('slice-06', 183, 0.867, 0.854, 0.860),
  ('slice-07', 206, 0.884, 0.867, 0.875),
  ('slice-08', 188, 0.866, 0.880, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_007341;
ROLLBACK;
