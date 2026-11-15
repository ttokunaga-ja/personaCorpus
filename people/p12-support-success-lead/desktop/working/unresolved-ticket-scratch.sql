-- Canonical frozen HelioDesk queue, 2026-08-04T09:00:00Z; SLA target = 4 hours.
-- Assumption: tickets(ticket_id, customer, priority, status, assignee, created_at, sla_due_at).
WITH open_queue AS (
  SELECT ticket_id, customer, priority, status, assignee, created_at, sla_due_at,
    CASE WHEN sla_due_at < TIMESTAMP '2026-08-04 09:00:00+00' THEN 'breached'
         WHEN sla_due_at <= TIMESTAMP '2026-08-04 10:00:00+00' THEN 'at-risk'
         ELSE 'within' END AS sla_state
  FROM tickets
  WHERE status IN ('open', 'pending_customer', 'investigating')
)
SELECT priority, sla_state, COUNT(*) AS unresolved_count
FROM open_queue
GROUP BY priority, sla_state
ORDER BY CASE priority WHEN 'P1' THEN 1 WHEN 'P2' THEN 2 WHEN 'P3' THEN 3 ELSE 4 END,
         CASE sla_state WHEN 'breached' THEN 1 WHEN 'at-risk' THEN 2 ELSE 3 END;
-- Escalation IDs: HD-260804-2718, HD-260804-1934, HD-260804-2841,
-- HD-260804-2861, HD-260804-2726, HD-260804-2763.
