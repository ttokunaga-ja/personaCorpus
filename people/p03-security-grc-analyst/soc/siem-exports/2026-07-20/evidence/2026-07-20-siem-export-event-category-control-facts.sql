-- Cobalt Harbor Systems Sentinel Export; evidence set 184; EV-184-07; event-category review; 2026-07-20; synthetic minimized defensive SIEM context
-- spine anchors: org.cobalt-harbor, system.sentinel-export, rel.siem.minimization, date.2026-07-20, term.no-kio-claim
CREATE TABLE sentinel_export_001248 (record_sequence INTEGER PRIMARY KEY, review_window TEXT NOT NULL, topic TEXT NOT NULL, event_category TEXT NOT NULL, synthetic_count INTEGER NOT NULL CHECK (synthetic_count >= 0));
INSERT INTO sentinel_export_001248 VALUES (1248, '2026-07-20', 'event-category review', 'access-review', 48);
