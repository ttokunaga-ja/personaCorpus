-- Juniper Works synthetic evaluation record: p08-full-001102
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001102 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001102 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 168, 0.880, 0.857, 0.868),
  ('slice-02', 201, 0.862, 0.870, 0.866),
  ('slice-03', 188, 0.879, 0.883, 0.881),
  ('slice-04', 248, 0.896, 0.859, 0.877),
  ('slice-05', 201, 0.878, 0.872, 0.875),
  ('slice-06', 196, 0.895, 0.848, 0.871),
  ('slice-07', 212, 0.877, 0.861, 0.869),
  ('slice-08', 239, 0.894, 0.874, 0.884);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001102;
ROLLBACK;
