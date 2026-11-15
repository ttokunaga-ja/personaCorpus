-- Synthetic Cobalt Harbor Systems AC-27 intake record; no operational access.
CREATE TABLE IF NOT EXISTS synthetic_ac27_intake (record_id TEXT, artifact_id TEXT, source_id TEXT, review_date TEXT, control_id TEXT, audit_request TEXT, case_id TEXT, risk_id TEXT, disposition TEXT, marker TEXT);
INSERT INTO synthetic_ac27_intake VALUES ('INB-027', 'p03-full-001949', 'p03-src-001949', '2026-07-20', 'AC-27', 'AR-260713-027', 'CASE-260713-184', 'RISK-031', 'synthetic intake normalized; bounded review retained', 'foundation-b001-027');
-- spine_traceability: org.cobalt-harbor=Cobalt Harbor Systems | date.2026-07-20=2026-07-20 closure-evidence follow-up date
