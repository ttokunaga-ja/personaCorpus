-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-006988', 'p02-src-006988', 'p02-primary-01', 'documents/operations/runbooks', 'node_pressure', "org.ntc, svc.identity.svc23, rel.inc260713.timeline, date.2026-07-20, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', 'c98e6688ddf25b8b');
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-006988-01', 'consumer_headroom', 31, NULL, 'percent', "capacity headroom after mitigation");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-006988-02', 'checkout_baseline', 1850, NULL, 'requests_per_second', "baseline demand");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-006988-03', 'checkout_peak', 2420, NULL, 'requests_per_second', "incident peak");
COMMIT;
