BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_8482 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_8482 VALUES ('p01-src-008482', 'MIG-26Q3-W05', 'observability', '2026-09-13T11:00:00+09:00', 216, 2700, 'acfd7958ccba');
COMMIT;
