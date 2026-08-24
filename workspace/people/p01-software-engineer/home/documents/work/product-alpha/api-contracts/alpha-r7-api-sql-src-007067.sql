-- Synthetic Product Alpha R7 contract registry record: API-A7-7067
CREATE TABLE IF NOT EXISTS contract_7067 (
  contract_id TEXT PRIMARY KEY,
  release_code TEXT NOT NULL,
  resource_name TEXT NOT NULL,
  operation_name TEXT NOT NULL,
  target_p95_ms INTEGER NOT NULL CHECK (target_p95_ms = 200),
  reviewed_on TEXT NOT NULL
);
INSERT INTO contract_7067 (contract_id, release_code, resource_name, operation_name, target_p95_ms, reviewed_on)
VALUES ('API-A7-7067', 'ALPHA-R7', 'evidence-snapshots', 'reconcile', 200, '2026-07-20');
