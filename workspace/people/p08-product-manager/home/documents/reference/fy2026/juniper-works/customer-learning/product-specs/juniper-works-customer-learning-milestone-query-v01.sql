-- Juniper Works synthetic evaluation record: p08-full-002181
-- Run M-27 | Product Alpha Q3 / Product Beta Q4 | 18.4% adoption synthetic-only
BEGIN;
CREATE TEMP TABLE eval_p08_full_002181 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p08_full_002181 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 163, 0.870, 0.861, 0.865),
  ('slice-02', 198, 0.887, 0.874, 0.880),
  ('slice-03', 187, 0.869, 0.850, 0.859),
  ('slice-04', 247, 0.886, 0.863, 0.874),
  ('slice-05', 248, 0.868, 0.876, 0.872),
  ('slice-06', 172, 0.885, 0.852, 0.868),
  ('slice-07', 248, 0.867, 0.865, 0.866),
  ('slice-08', 209, 0.884, 0.878, 0.881);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p08_full_002181;
ROLLBACK;
