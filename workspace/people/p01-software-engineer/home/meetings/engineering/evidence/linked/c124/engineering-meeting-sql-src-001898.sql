-- Synthetic engineering meeting register: MTG-ENG-20260803-30
CREATE TABLE IF NOT EXISTS meeting_register_1898 (meeting_id TEXT PRIMARY KEY, meeting_date TEXT NOT NULL, reference_id TEXT NOT NULL, owner_role TEXT NOT NULL, action_id TEXT NOT NULL, status TEXT NOT NULL);
INSERT INTO meeting_register_1898 VALUES ('MTG-ENG-20260803-30', '2026-08-03', 'ADR-057', 'Observability Lead', 'ACT-ENG-001898', 'approved');
