BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_11982 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_11982 VALUES ('p01-src-011982', 'MIG-26Q3-W01', 'media-worker', '2026-09-09T15:00:00+09:00', 96, 2700, '245250a079b0');
COMMIT;
