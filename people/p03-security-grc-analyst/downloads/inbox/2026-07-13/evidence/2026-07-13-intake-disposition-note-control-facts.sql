-- Synthetic Cobalt Harbor Systems AC-27 intake record; no operational access.
CREATE TABLE IF NOT EXISTS synthetic_ac27_intake (record_id TEXT, artifact_id TEXT, source_id TEXT, review_date TEXT, control_id TEXT, audit_request TEXT, case_id TEXT, risk_id TEXT, disposition TEXT, marker TEXT);
INSERT INTO synthetic_ac27_intake VALUES ('INB-082', 'p03-full-004515', 'p03-src-004515', '2026-07-13', 'AC-27', 'AR-260713-027', 'CASE-260713-184', 'RISK-031', 'synthetic intake normalized; bounded review retained', 'foundation-b001-082');
-- spine_traceability: org.cobalt-harbor=Cobalt Harbor Systems | audit.ar-260713-027=AR-260713-027 fictional audit request for AC-27 review material | date.2026-07-13=2026-07-13 primary review date
