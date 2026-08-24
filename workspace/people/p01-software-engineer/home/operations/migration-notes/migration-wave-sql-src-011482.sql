BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_11482 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_11482 VALUES ('p01-src-011482', 'MIG-26Q3-W05', 'policy-cache', '2026-09-13T11:00:00+09:00', 139, 2700, '971019e69a1b');
COMMIT;
