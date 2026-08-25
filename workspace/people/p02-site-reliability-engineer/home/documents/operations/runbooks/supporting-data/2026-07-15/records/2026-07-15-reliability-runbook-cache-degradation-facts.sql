-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, service TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-013315', 'p02-src-013315', 'p02-primary-01', 'documents/operations/runbooks', 'incident_timeline', "org.ntc, svc.identity.svc23, rel.inc260713.timeline, date.2026-07-15, term.no-kio-claim", 'Northstar Transit Cloud', 'Identity SVC-23', 'INC-260713', '2026-07-13', 'error budget 2.4%', '1bd497b1b41d538c');
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-013315-01', 'checkout_availability_slo', 'Checkout SVC-17', 99.95, NULL, 'percent', "30-day objective");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-013315-02', 'error_budget_remaining', 'Checkout SVC-17', 2.4, NULL, 'percent', "request-error allowance");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-013315-03', 'incident_resolution', 'Checkout SVC-17', NULL, "10:42", 'utc', "resolution declared");
COMMIT;
