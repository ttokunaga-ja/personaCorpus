-- Juniper Works synthetic evaluation record: p08-full-005842
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005842 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005842 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 179, 0.876, 0.850, 0.863),
  ('slice-02', 206, 0.893, 0.863, 0.878),
  ('slice-03', 191, 0.875, 0.876, 0.875),
  ('slice-04', 202, 0.892, 0.852, 0.872),
  ('slice-05', 226, 0.874, 0.865, 0.869),
  ('slice-06', 209, 0.891, 0.878, 0.884),
  ('slice-07', 171, 0.873, 0.854, 0.863),
  ('slice-08', 171, 0.890, 0.867, 0.878);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005842;
ROLLBACK;
