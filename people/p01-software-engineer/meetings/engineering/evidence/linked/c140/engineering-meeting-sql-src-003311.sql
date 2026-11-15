-- Synthetic engineering meeting register: MTG-ENG-20260824-19
CREATE TABLE IF NOT EXISTS meeting_register_3311 (meeting_id TEXT PRIMARY KEY, meeting_date TEXT NOT NULL, reference_id TEXT NOT NULL, owner_role TEXT NOT NULL, action_id TEXT NOT NULL, status TEXT NOT NULL);
INSERT INTO meeting_register_3311 VALUES ('MTG-ENG-20260824-19', '2026-08-24', 'ADR-B4-031', 'Decision Owner', 'ACT-ENG-003311', 'approved');
