-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, service TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-007888', 'p02-src-007888', 'p02-primary-01', 'documents/operations/runbooks', 'queue_saturation', "org.ntc, svc.identity.svc23, rel.inc260713.timeline, date.2026-07-14, term.no-kio-claim", 'Northstar Transit Cloud', 'Identity SVC-23', 'INC-260713', '2026-07-13', 'error budget 2.4%', 'c7e8834f8775bb80');
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-007888-01', 'q07_maximum', 'Checkout SVC-17', 38420, NULL, 'jobs', "observed maximum");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-007888-02', 'queue_drain', 'Checkout SVC-17', 1460, NULL, 'jobs_per_minute', "drain rate");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-007888-03', 'recovery_threshold', 'Checkout SVC-17', NULL, "below 8000", 'jobs', "recovery at 10:24");
COMMIT;
