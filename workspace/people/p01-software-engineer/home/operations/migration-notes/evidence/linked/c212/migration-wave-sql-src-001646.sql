BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS migration_observation_1646 (record_id TEXT PRIMARY KEY, wave_id TEXT NOT NULL, service_name TEXT NOT NULL, observed_at TEXT NOT NULL, cursor_age_seconds INTEGER NOT NULL CHECK (cursor_age_seconds <= 300), restore_budget_seconds INTEGER NOT NULL CHECK (restore_budget_seconds <= 2700), evidence_hint TEXT NOT NULL);
INSERT INTO migration_observation_1646 VALUES ('p01-src-001646', 'MIG-26Q3-W03', 'billing-ledger', '2026-09-11T15:00:00+09:00', 77, 2700, 'edc82fee3dd0');
COMMIT;
