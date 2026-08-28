-- Helix Forge Lab synthetic evaluation record: p04-full-003722
-- Run R184 | HF-Eval-S1/S2 v3.2 | 2,400 synthetic records
BEGIN;
CREATE TEMP TABLE eval_p04_full_003722 (
  slice_id TEXT PRIMARY KEY,
  records INTEGER NOT NULL CHECK (records > 0),
  precision_score REAL NOT NULL CHECK (precision_score BETWEEN 0 AND 1),
  recall_score REAL NOT NULL CHECK (recall_score BETWEEN 0 AND 1),
  f1_score REAL NOT NULL CHECK (f1_score BETWEEN 0 AND 1)
);
INSERT INTO eval_p04_full_003722 (slice_id, records, precision_score, recall_score, f1_score) VALUES
  ('slice-01', 206, 0.893, 0.863, 0.878),
  ('slice-02', 220, 0.875, 0.876, 0.875),
  ('slice-03', 245, 0.892, 0.852, 0.872),
  ('slice-04', 181, 0.874, 0.865, 0.869),
  ('slice-05', 168, 0.891, 0.878, 0.884),
  ('slice-06', 227, 0.873, 0.854, 0.863),
  ('slice-07', 228, 0.890, 0.867, 0.878),
  ('slice-08', 199, 0.872, 0.880, 0.876);
SELECT COUNT(*) AS slice_count, ROUND(AVG(f1_score), 3) AS mean_f1 FROM eval_p04_full_003722;
ROLLBACK;
