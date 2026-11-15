-- Helix Forge Lab synthetic evaluation record: p04-full-005878
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_005878 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_005878 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 242, 0.864, 0.876, 0.870),
  ('slice-02', 190, 0.881, 0.852, 0.866),
  ('slice-03', 230, 0.863, 0.865, 0.864),
  ('slice-04', 221, 0.880, 0.878, 0.879),
  ('slice-05', 235, 0.862, 0.854, 0.858),
  ('slice-06', 213, 0.879, 0.867, 0.873),
  ('slice-07', 173, 0.896, 0.880, 0.888),
  ('slice-08', 219, 0.878, 0.856, 0.867);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_005878;
ROLLBACK;
