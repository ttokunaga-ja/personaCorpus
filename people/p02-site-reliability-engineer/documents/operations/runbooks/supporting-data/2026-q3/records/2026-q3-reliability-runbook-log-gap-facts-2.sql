-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, service TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-011686', 'p02-src-011686', 'p02-primary-01', 'documents/operations/runbooks', 'log_gap', "org.ntc, svc.identity.svc23, rel.inc260713.timeline, period.2026-q3, term.no-kio-claim", 'Northstar Transit Cloud', 'Identity SVC-23', 'INC-260713', '2026-07-13', 'error budget 2.4%', 'db696c16f0c5c8b6');
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-011686-01', 'telemetry_export_gap', 'Telemetry SVC-31', 7, NULL, 'minutes', "reconciled by 11:06 UTC");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-011686-02', 'telemetry_delivery_slo', 'Telemetry SVC-31', 99.9, NULL, 'percent_within_10_minutes', "delivery objective");
INSERT INTO incident_metric (record_id, metric_name, service, numeric_value, text_value, unit, note) VALUES ('p02-full-011686-03', 'evidence_chain', 'Checkout SVC-17', 1, NULL, 'complete', "scope-local evidence retained");
COMMIT;
