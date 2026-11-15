-- Juniper Works synthetic evaluation record: p08-full-001789
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_001789 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_001789 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 248, 0.885, 0.859, 0.872),
  ('slice-02', 193, 0.867, 0.872, 0.869),
  ('slice-03', 232, 0.884, 0.848, 0.866),
  ('slice-04', 222, 0.866, 0.861, 0.863),
  ('slice-05', 188, 0.883, 0.874, 0.878),
  ('slice-06', 190, 0.865, 0.850, 0.857),
  ('slice-07', 209, 0.882, 0.863, 0.872),
  ('slice-08', 237, 0.864, 0.876, 0.870);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_001789;
ROLLBACK;
