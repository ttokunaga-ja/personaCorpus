-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, service TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-006938', 'p02-src-006938', 'p02-primary-01', 'documents/operations/runbooks', 'traffic_shift', "org.ntc, term.mitigation, rel.inc260713.timeline, date.2026-07-14, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', '7e6a8b0b40689bb5');
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-006938-01', 'checkout_peak', 'Checkout SVC-17', 2420, NULL, 'requests_per_second', "peak during incident");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-006938-02', 'rate_limit_start', 'Checkout SVC-17', NULL, "09:54", 'utc', "authorizations rate-limited");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-006938-03', 'identity_p95', 'Identity SVC-23', 286, NULL, 'milliseconds', "below 350 ms page threshold");
COMMIT;
