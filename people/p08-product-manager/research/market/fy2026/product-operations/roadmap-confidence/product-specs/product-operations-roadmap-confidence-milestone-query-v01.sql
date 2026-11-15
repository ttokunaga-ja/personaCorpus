-- Juniper Works synthetic evaluation record: p08-full-001593
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001593 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001593 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 200, 0.892, 0.859, 0.875),
  ('slice-02', 217, 0.874, 0.872, 0.873),
  ('slice-03', 196, 0.891, 0.848, 0.869),
  ('slice-04', 252, 0.873, 0.861, 0.867),
  ('slice-05', 203, 0.890, 0.874, 0.882),
  ('slice-06', 197, 0.872, 0.850, 0.861),
  ('slice-07', 165, 0.889, 0.863, 0.876),
  ('slice-08', 215, 0.871, 0.876, 0.873);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001593;
ROLLBACK;
