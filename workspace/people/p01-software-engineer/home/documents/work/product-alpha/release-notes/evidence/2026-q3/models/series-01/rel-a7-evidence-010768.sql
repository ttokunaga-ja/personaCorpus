-- Synthetic Aster Loop release-note record p01-full-010768
CREATE TABLE IF NOT EXISTS release_note_10768 (record_id TEXT PRIMARY KEY, release_date TEXT NOT NULL, status TEXT NOT NULL, observed_p95_ms INTEGER NOT NULL CHECK (observed_p95_ms <= 200));
INSERT INTO release_note_10768 (record_id, release_date, status, observed_p95_ms) VALUES ('REL-A7-20260713-02', '2026-07-13', 'released', 184);
