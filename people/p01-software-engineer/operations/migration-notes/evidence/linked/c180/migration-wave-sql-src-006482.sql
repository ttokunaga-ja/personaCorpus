BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_6482 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_6482 VALUES ('p01-src-006482', 'MIG-26Q3-W03', 'decision-store', '2026-09-11T11:00:00+09:00', 207, 2700, '0a8c8eb78f5b');
COMMIT;
