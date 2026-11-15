-- Synthetic Product Alpha R7 architecture evidence p01-src-010235
CREATE TABLE IF NOT EXISTS architecture_observation_10235 (record_id TEXT PRIMARY KEY, observed_on TEXT NOT NULL, service TEXT NOT NULL, decision_id TEXT NOT NULL, p95_ms INTEGER NOT NULL CHECK (p95_ms > 0), state TEXT NOT NULL);
INSERT INTO architecture_observation_10235 (record_id, observed_on, service, decision_id, p95_ms, state) VALUES ('ARCH-A7-010235', '2026-08-05', 'svc-contract-registry', 'ADR-087', 189, 'planned');
