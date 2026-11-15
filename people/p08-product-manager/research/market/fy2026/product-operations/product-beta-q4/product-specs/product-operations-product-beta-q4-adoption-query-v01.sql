-- Juniper Works synthetic evaluation record: p08-full-005550
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_005550 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_005550 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 188, 0.870, 0.859, 0.864),
  ('slice-02', 163, 0.887, 0.872, 0.879),
  ('slice-03', 169, 0.869, 0.848, 0.858),
  ('slice-04', 238, 0.886, 0.861, 0.873),
  ('slice-05', 196, 0.868, 0.874, 0.871),
  ('slice-06', 194, 0.885, 0.850, 0.867),
  ('slice-07', 164, 0.867, 0.863, 0.865),
  ('slice-08', 167, 0.884, 0.876, 0.880);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_005550;
ROLLBACK;
