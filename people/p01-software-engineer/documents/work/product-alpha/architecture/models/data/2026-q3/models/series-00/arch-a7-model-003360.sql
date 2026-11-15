-- Synthetic Product Alpha R7 architecture evidence p01-src-003360
CREATE TABLE IF NOT EXISTS architecture_observation_3360 (record_id TEXT PRIMARY KEY, observed_on TEXT NOT NULL, service TEXT NOT NULL, decision_id TEXT NOT NULL, p95_ms INTEGER NOT NULL CHECK (p95_ms > 0), state TEXT NOT NULL);
INSERT INTO architecture_observation_3360 (record_id, observed_on, service, decision_id, p95_ms, state) VALUES ('ARCH-A7-003360', '2026-08-30', 'svc-edge-api', 'ADR-042', 184, 'planned');
