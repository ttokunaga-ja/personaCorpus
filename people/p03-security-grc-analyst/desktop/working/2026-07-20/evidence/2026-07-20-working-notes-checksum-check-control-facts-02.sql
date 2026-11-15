-- Working-note focus: 2026 07 20 working notes checksum check control facts sql s004746; source p03-src-004746 is a bounded synthetic analyst record.
-- org.cobalt-harbor | Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.
-- case.case-260713-184 | CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.
-- date.2026-07-20 | Seven-day follow-up validates closure evidence and residual-risk status.
CREATE TABLE IF NOT EXISTS trace_p03_src_004746 (spine_id TEXT NOT NULL, canonical_value TEXT NOT NULL);
INSERT INTO trace_p03_src_004746 VALUES ('org.cobalt-harbor', 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.');
INSERT INTO trace_p03_src_004746 VALUES ('case.case-260713-184', 'CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.');
INSERT INTO trace_p03_src_004746 VALUES ('date.2026-07-20', 'Seven-day follow-up validates closure evidence and residual-risk status.');
