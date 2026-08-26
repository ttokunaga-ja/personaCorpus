-- Synthetic Cobalt Harbor Systems AC-27 intake record; no operational access.
CREATE TABLE IF NOT EXISTS synthetic_ac27_intake (record_id TEXT, artifact_id TEXT, source_id TEXT, review_date TEXT, review_period TEXT, control_id TEXT, audit_request TEXT, case_id TEXT, risk_id TEXT, disposition TEXT, marker TEXT);
INSERT INTO synthetic_ac27_intake VALUES ('INB-033', 'p03-full-002583', 'p03-src-002583', '2026-07-13', '2026-Q3', 'AC-27', 'AR-260713-027', 'CASE-260713-184', 'RISK-031', 'synthetic intake normalized; bounded review retained', 'foundation-b001-033');
-- spine_traceability: org.cobalt-harbor=Cobalt Harbor Systems | audit.ar-260713-027=AR-260713-027 fictional audit request for AC-27 review material | period.2026-q3=2026-Q3 governance planning period 2026-07-01 through 2026-09-30
