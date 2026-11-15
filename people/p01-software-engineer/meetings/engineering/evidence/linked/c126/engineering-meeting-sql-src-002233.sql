-- Synthetic engineering meeting register: MTG-ENG-20260904-09
CREATE TABLE IF NOT EXISTS meeting_register_2233 (meeting_id TEXT PRIMARY KEY, meeting_date TEXT NOT NULL, reference_id TEXT NOT NULL, owner_role TEXT NOT NULL, action_id TEXT NOT NULL, status TEXT NOT NULL);
INSERT INTO meeting_register_2233 VALUES ('MTG-ENG-20260904-09', '2026-09-04', 'API-B4-120', 'API Steward', 'ACT-ENG-002233', 'reviewed');
