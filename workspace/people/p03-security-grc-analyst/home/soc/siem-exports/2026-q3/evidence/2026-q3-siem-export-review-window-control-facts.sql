-- Cobalt Harbor Systems Sentinel Export; evidence set 184; EV-184-07; event-category review; 2026-q3; synthetic minimized defensive SIEM context
-- spine anchors: org.cobalt-harbor, evidence.ev-184-07, rel.siem.minimization, period.2026-q3, term.no-kio-claim
CREATE TABLE sentinel_export_003847 (record_sequence INTEGER PRIMARY KEY, review_window TEXT NOT NULL, topic TEXT NOT NULL, event_category TEXT NOT NULL, synthetic_count INTEGER NOT NULL CHECK (synthetic_count >= 0));
INSERT INTO sentinel_export_003847 VALUES (3847, '2026-q3', 'event-category review', 'approval-record', 6);
