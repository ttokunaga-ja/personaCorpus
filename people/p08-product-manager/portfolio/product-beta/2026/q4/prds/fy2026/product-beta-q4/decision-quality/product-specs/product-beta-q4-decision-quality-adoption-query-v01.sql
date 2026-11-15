-- Juniper Works synthetic evaluation record: p08-full-000178
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000178 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000178 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 241, 0.873, 0.881, 0.877),
  ('slice-02', 190, 0.890, 0.857, 0.873),
  ('slice-03', 230, 0.872, 0.870, 0.871),
  ('slice-04', 174, 0.889, 0.883, 0.886),
  ('slice-05', 164, 0.871, 0.859, 0.865),
  ('slice-06', 178, 0.888, 0.872, 0.880),
  ('slice-07', 203, 0.870, 0.848, 0.859),
  ('slice-08', 234, 0.887, 0.861, 0.874);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000178;
ROLLBACK;
