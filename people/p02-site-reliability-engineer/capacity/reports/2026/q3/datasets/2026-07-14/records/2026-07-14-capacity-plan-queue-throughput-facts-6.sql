-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-013412', 'p02-src-013412', 'p02-primary-11', 'capacity/reports', 'capacity', "org.ntc, measure.queue.drain-rate, rel.capacity.2026-q3, date.2026-07-14, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', '9361da47cb985695');
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-013412-01', 'headroom', 31, NULL, 'percent', "available capacity");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-013412-02', 'queue_drain', 1460, NULL, 'jobs_per_minute', "drain rate");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-013412-03', 'error_budget_remaining', 2.4, NULL, 'percent', "request-error allowance");
COMMIT;
