-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, service TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-008113', 'p02-src-008113', 'p02-primary-06', 'services/identity/prod/oncall/operations', 'identity', "org.ntc, inc.inc-260713, rel.oncall.2026-07-13, period.2026-q3, term.no-kio-claim", 'Northstar Transit Cloud', 'Identity SVC-23', 'INC-260713', '2026-07-13', 'error budget 2.4%', 'ca0a3732c2526318');
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-008113-01', 'identity_availability_slo', 'Identity SVC-23', 99.99, NULL, 'percent', "availability objective");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-008113-02', 'identity_p95', 'Identity SVC-23', 286, NULL, 'milliseconds', "below 350 ms page threshold");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-008113-03', 'identity_cause', 'Identity SVC-23', NULL, "load_degraded_only", 'classification', "never initiating cause");
COMMIT;
