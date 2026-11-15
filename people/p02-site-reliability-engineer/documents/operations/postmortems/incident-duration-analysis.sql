-- Synthetic postmortem duration analysis; timestamps are UTC.
WITH incidents(incident_id, declared_at, mitigated_at, severity) AS (
  VALUES
    ('INC-260407', TIMESTAMP '2026-04-07 18:20:00', TIMESTAMP '2026-04-07 19:12:00', 'SEV-3'),
    ('INC-260519', TIMESTAMP '2026-05-19 07:41:00', TIMESTAMP '2026-05-19 08:22:00', 'SEV-3'),
    ('INC-260628', TIMESTAMP '2026-06-28 14:03:00', TIMESTAMP '2026-06-28 14:29:00', 'SEV-2'),
    ('INC-260713', TIMESTAMP '2026-07-13 09:12:00', TIMESTAMP '2026-07-13 09:50:00', 'SEV-2')
)
SELECT severity, COUNT(*) AS incident_count,
       AVG(EXTRACT(EPOCH FROM (mitigated_at - declared_at)) / 60.0) AS avg_minutes_to_mitigate
FROM incidents
GROUP BY severity
ORDER BY severity;
