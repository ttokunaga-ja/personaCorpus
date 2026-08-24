BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_10482 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_10482 VALUES ('p01-src-010482', 'MIG-26Q3-W01', 'identity-sync', '2026-09-09T11:00:00+09:00', 225, 2700, '7c27c4cf716c');
COMMIT;
