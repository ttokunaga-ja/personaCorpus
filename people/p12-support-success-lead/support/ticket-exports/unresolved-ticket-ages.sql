-- Age bands are evaluated against the frozen snapshot, not wall-clock time.
CREATE VIEW unresolved_ticket_age_bands AS
SELECT
  CASE
    WHEN unresolved_age_minutes < 60 THEN '00-59m'
    WHEN unresolved_age_minutes < 120 THEN '01-01:59h'
    WHEN unresolved_age_minutes < 240 THEN '02-03:59h'
    ELSE '04h+'
  END AS age_band,
  COUNT(*) AS ticket_count,
  SUM(CASE WHEN sla_state = 'breached' THEN 1 ELSE 0 END) AS breached_count
FROM heliodesk_ticket_export
WHERE status IN ('open', 'pending_customer', 'investigating')
GROUP BY 1;

CREATE VIEW unresolved_ticket_age_detail AS
SELECT ticket_id, customer, priority, unresolved_age_minutes, sla_state
FROM heliodesk_ticket_export
WHERE status IN ('open', 'pending_customer', 'investigating')
ORDER BY unresolved_age_minutes DESC, ticket_id;
