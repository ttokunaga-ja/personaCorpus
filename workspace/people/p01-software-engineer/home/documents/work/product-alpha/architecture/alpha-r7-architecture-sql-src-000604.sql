-- Synthetic Product Alpha R7 architecture evidence p01-src-000604
CREATE TABLE IF NOT EXISTS architecture_observation_604 (record_id TEXT PRIMARY KEY, observed_on TEXT NOT NULL, service TEXT NOT NULL, decision_id TEXT NOT NULL, p95_ms INTEGER NOT NULL CHECK (p95_ms > 0), state TEXT NOT NULL);
INSERT INTO architecture_observation_604 (record_id, observed_on, service, decision_id, p95_ms, state) VALUES ('ARCH-A7-000604', '2026-09-03', 'svc-observability', 'ADR-008', 191, 'closed');
