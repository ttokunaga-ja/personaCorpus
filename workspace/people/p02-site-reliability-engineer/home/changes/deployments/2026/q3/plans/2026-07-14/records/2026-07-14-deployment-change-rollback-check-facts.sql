-- Northstar Transit Cloud | Checkout SVC-17 | 2026-07-13 | error budget 2.4% | INC-260713
BEGIN TRANSACTION;
CREATE TABLE artifact_context (artifact_id TEXT PRIMARY KEY, source_id TEXT NOT NULL, scope_id TEXT NOT NULL, scope_path TEXT NOT NULL, topic TEXT NOT NULL, content_spine_ids TEXT NOT NULL, organization TEXT NOT NULL, service TEXT NOT NULL, incident TEXT NOT NULL, incident_date TEXT NOT NULL, error_budget_text TEXT NOT NULL, fingerprint TEXT NOT NULL);
CREATE TABLE incident_metric (record_id TEXT PRIMARY KEY, metric_name TEXT NOT NULL, numeric_value REAL, text_value TEXT, unit TEXT NOT NULL, note TEXT NOT NULL, CHECK ((numeric_value IS NOT NULL) != (text_value IS NOT NULL)));
INSERT INTO artifact_context VALUES ('p02-full-008514', 'p02-src-008514', 'p02-primary-10', 'changes/deployments', 'rollback', "org.ntc, change.chg-260713-042, rel.change.260713, date.2026-07-14, term.no-kio-claim", 'Northstar Transit Cloud', 'Checkout SVC-17', 'INC-260713', '2026-07-13', 'error budget 2.4%', '53e3726a5de6af4e');
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-008514-01', 'rollback_approval', NULL, "10:02", 'utc', "approved change control");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-008514-02', 'concurrency_increase', NULL, "10:11", 'utc', "consumer concurrency raised");
INSERT INTO incident_metric (record_id, metric_name, numeric_value, text_value, unit, note) VALUES ('p02-full-008514-03', 'impact_end', NULL, "10:31", 'utc', "impact closed");
COMMIT;
