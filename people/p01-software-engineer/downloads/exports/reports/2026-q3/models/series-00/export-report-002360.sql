-- EXP-26Q3-002360; generated 2026-08-09T10:20:00+09:00
CREATE TABLE IF NOT EXISTS export_snapshot_2360 (
  export_id TEXT PRIMARY KEY,
  source_id TEXT NOT NULL,
  project TEXT NOT NULL,
  service TEXT NOT NULL,
  lifecycle_state TEXT NOT NULL,
  latency_ms INTEGER NOT NULL,
  generated_at TEXT NOT NULL
);
INSERT INTO export_snapshot_2360 VALUES ('EXP-26Q3-002360', 'p01-src-002360', 'Product Alpha R7', 'svc-edge-api', 'reviewed', 184, '2026-08-09T10:20:00+09:00');
