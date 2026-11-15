BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_481 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_481 VALUES ('p01-src-000481', 'MIG-26Q3-W02', 'reporting-api', '2026-09-10T10:00:00+09:00', 179, 2700, '1abf45e5b214');
COMMIT;
