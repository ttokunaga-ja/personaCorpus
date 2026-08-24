-- Synthetic engineering meeting register: MTG-ENG-20260803-56
CREATE TABLE IF NOT EXISTS meeting_register_1568 (meeting_id TEXT PRIMARY KEY, meeting_date TEXT NOT NULL, reference_id TEXT NOT NULL, owner_role TEXT NOT NULL, action_id TEXT NOT NULL, status TEXT NOT NULL);
INSERT INTO meeting_register_1568 VALUES ('MTG-ENG-20260803-56', '2026-08-03', 'ADR-057', 'Observability Lead', 'ACT-ENG-001568', 'approved');
