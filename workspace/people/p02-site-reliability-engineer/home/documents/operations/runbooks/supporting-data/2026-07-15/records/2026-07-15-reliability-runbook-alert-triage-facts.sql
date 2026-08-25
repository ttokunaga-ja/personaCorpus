-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-003589', 'p02-src-003589', 'p02-primary-01', 'documents/operations/runbooks', 'alert_triage', "org.ntc, svc.identity.svc23, rel.inc260713.timeline, date.2026-07-15, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', '0fc238a5983d769c');
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-003589-01', 'q07_warning', 12000, NULL, 'jobs', "warning threshold at 09:42");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-003589-02', 'q07_critical', 24000, NULL, 'jobs', "critical threshold");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-003589-03', 'burn_one_hour', 14.6, NULL, 'multiplier', "short-window burn");
COMMIT;
