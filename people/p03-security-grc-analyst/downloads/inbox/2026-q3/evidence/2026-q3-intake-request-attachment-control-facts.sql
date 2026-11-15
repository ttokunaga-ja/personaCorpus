-- Synthetic Cobalt Harbor Systems AC-27 intake record; no operational access.
CREATE TABLE IF NOT EXISTS synthetic_ac27_intake (record_id TEXT, artifact_id TEXT, source_id TEXT, review_date TEXT, review_period TEXT, control_id TEXT, audit_request TEXT, case_id TEXT, risk_id TEXT, disposition TEXT, marker TEXT);
INSERT INTO synthetic_ac27_intake VALUES ('INB-025', 'p03-full-001816', 'p03-src-001816', '2026-07-13', '2026-Q3', 'AC-27', 'AR-260713-027', 'CASE-260713-184', 'RISK-031', 'synthetic intake normalized; bounded review retained', 'foundation-b001-025');
-- spine_traceability: org.cobalt-harbor=Cobalt Harbor Systems | evidence.ev-184-19=EV-184-19 evidence set 184 checksum reconciliation record | period.2026-q3=2026-Q3 governance planning period 2026-07-01 through 2026-09-30
