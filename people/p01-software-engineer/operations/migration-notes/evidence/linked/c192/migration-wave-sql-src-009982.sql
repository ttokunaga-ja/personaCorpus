BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_9982 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_9982 VALUES ('p01-src-009982', 'MIG-26Q3-W05', 'audit-journal', '2026-09-13T15:00:00+09:00', 87, 2700, 'b84d43b167ae');
COMMIT;
