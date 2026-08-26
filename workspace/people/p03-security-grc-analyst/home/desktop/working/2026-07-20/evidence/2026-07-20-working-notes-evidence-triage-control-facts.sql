-- Working-note focus: 2026 07 20 working notes evidence triage control facts sql s004016; source p03-src-004016 is a bounded synthetic analyst record.
-- org.cobalt-harbor | Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.
-- team.grc | Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination.
-- date.2026-07-20 | Seven-day follow-up validates closure evidence and residual-risk status.
CREATE TABLE IF NOT EXISTS trace_p03_src_004016 (spine_id TEXT NOT NULL, canonical_value TEXT NOT NULL);
INSERT INTO trace_p03_src_004016 VALUES ('org.cobalt-harbor', 'Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform.');
INSERT INTO trace_p03_src_004016 VALUES ('team.grc', 'Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination.');
INSERT INTO trace_p03_src_004016 VALUES ('date.2026-07-20', 'Seven-day follow-up validates closure evidence and residual-risk status.');
