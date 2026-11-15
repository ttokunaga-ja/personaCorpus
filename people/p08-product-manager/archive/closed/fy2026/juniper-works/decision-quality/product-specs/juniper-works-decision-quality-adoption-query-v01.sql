-- Juniper Works synthetic evaluation record: p08-full-003764
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_003764 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_003764 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 169, 0.876, 0.856, 0.866),
  ('slice-02', 249, 0.893, 0.869, 0.881),
  ('slice-03', 165, 0.875, 0.882, 0.878),
  ('slice-04', 189, 0.892, 0.858, 0.875),
  ('slice-05', 172, 0.874, 0.871, 0.872),
  ('slice-06', 229, 0.891, 0.847, 0.868),
  ('slice-07', 181, 0.873, 0.860, 0.866),
  ('slice-08', 176, 0.890, 0.873, 0.881);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_003764;
ROLLBACK;
