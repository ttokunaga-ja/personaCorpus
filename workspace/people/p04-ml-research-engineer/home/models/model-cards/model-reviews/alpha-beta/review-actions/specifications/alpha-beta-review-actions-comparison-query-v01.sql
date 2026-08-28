-- Helix Forge Lab synthetic evaluation record: p04-full-002747
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_002747 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_002747 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 204, 0.871, 0.850, 0.860),
  ('slice-02', 219, 0.888, 0.863, 0.875),
  ('slice-03', 245, 0.870, 0.876, 0.873),
  ('slice-04', 229, 0.887, 0.852, 0.869),
  ('slice-05', 192, 0.869, 0.865, 0.867),
  ('slice-06', 192, 0.886, 0.878, 0.882),
  ('slice-07', 210, 0.868, 0.854, 0.861),
  ('slice-08', 238, 0.885, 0.867, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_002747;
ROLLBACK;
