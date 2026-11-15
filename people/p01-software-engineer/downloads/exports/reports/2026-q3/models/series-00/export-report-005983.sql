-- EXP-26Q3-005983; generated 2026-08-20T15:01:00+09:00
CREATE TABLE IF NOT EXISTS export_snapshot_5983 (
  export_id TEXT PRIMARY KEY,
  source_id TEXT NOT NULL,
  project TEXT NOT NULL,
  service TEXT NOT NULL,
  lifecycle_state TEXT NOT NULL,
  latency_ms INTEGER NOT NULL,
  generated_at TEXT NOT NULL
);
INSERT INTO export_snapshot_5983 VALUES ('EXP-26Q3-005983', 'p01-src-005983', 'Product Beta R4', 'svc-event-bus', 'closed', 225, '2026-08-20T15:01:00+09:00');
