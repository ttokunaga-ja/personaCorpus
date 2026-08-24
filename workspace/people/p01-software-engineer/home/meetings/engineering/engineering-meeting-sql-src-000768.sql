-- Synthetic engineering meeting register: MTG-ENG-20260720-57
CREATE TABLE IF NOT EXISTS meeting_register_768 (meeting_id TEXT PRIMARY KEY, meeting_date TEXT NOT NULL, reference_id TEXT NOT NULL, owner_role TEXT NOT NULL, action_id TEXT NOT NULL, status TEXT NOT NULL);
INSERT INTO meeting_register_768 VALUES ('MTG-ENG-20260720-57', '2026-07-20', 'ADR-042', 'Release Coordinator', 'ACT-ENG-000768', 'planned');
