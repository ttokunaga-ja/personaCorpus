-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-004888', 'p02-src-004888', 'p02-secondary-03', 'downloads/inbox', 'incident_timeline', "org.ntc, dep.payments.q07, rel.archive.closed, date.2026-07-20, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', '1e977c209a9ff2bc');
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-004888-01', 'checkout_availability_slo', 99.95, NULL, 'percent', "30-day objective");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-004888-02', 'error_budget_remaining', 2.4, NULL, 'percent', "request-error allowance");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-004888-03', 'incident_resolution', NULL, "10:42", 'utc', "resolution declared");
COMMIT;
