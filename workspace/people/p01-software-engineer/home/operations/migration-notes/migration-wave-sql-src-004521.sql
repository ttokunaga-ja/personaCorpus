BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_4521 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_4521 VALUES ('p01-src-004521', 'MIG-26Q3-W04', 'event-bus', '2026-09-12T10:00:00+09:00', 237, 2700, '04cf9a1fa64c');
COMMIT;
