-- Working-note focus: 2026 q3 working notes timeline annotation control facts sql s007449; source p03-src-007449 is a bounded synthetic analyst record.
-- org.cobalt-harbor | Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.
-- case.case-260713-184 | CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.
-- period.2026-q3 | Governance planning period from 2026-07-01 through 2026-09-30.
CREATE TABLE IF NOT EXISTS trace_p03_src_007449 (spine_id TEXT NOT NULL, canonical_value TEXT NOT NULL);
INSERT INTO trace_p03_src_007449 VALUES ('org.cobalt-harbor', 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.');
INSERT INTO trace_p03_src_007449 VALUES ('case.case-260713-184', 'CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident.');
INSERT INTO trace_p03_src_007449 VALUES ('period.2026-q3', 'Governance planning period from 2026-07-01 through 2026-09-30.');
