-- Synthetic Aster Loop release-note record p01-full-011897
CREATE TABLE IF NOT EXISTS release_note_11897 (record_id TEXT PRIMARY KEY, release_date TEXT NOT NULL, status TEXT NOT NULL, observed_p95_ms INTEGER NOT NULL CHECK (observed_p95_ms <= 200));
INSERT INTO release_note_11897 (record_id, release_date, status, observed_p95_ms) VALUES ('REL-A7-20260714-64', '2026-07-13', 'reviewed', 184);
