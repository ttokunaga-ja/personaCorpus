-- Juniper Works synthetic evaluation record: p08-full-000267
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_000267 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_000267 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 170, 0.862, 0.847, 0.854),
  ('slice-02', 249, 0.879, 0.860, 0.869),
  ('slice-03', 165, 0.896, 0.873, 0.884),
  ('slice-04', 189, 0.878, 0.849, 0.863),
  ('slice-05', 172, 0.895, 0.862, 0.878),
  ('slice-06', 229, 0.877, 0.875, 0.876),
  ('slice-07', 229, 0.894, 0.851, 0.872),
  ('slice-08', 247, 0.876, 0.864, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_000267;
ROLLBACK;
