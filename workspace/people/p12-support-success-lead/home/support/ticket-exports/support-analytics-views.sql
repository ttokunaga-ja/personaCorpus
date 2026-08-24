-- HelioDesk synthetic analytics views; source relation: heliodesk_ticket_export.
-- Snapshot is intentionally frozen at 2026-08-04T09:00:00Z.
CREATE VIEW unresolved_ticket_summary AS
SELECT customer, priority, sla_state, channel, COUNT(*) AS unresolved_tickets
FROM heliodesk_ticket_export
WHERE status IN ('open', 'pending_customer', 'investigating')
GROUP BY customer, priority, sla_state, channel;

CREATE VIEW sla_watch_queue AS
SELECT ticket_id, customer, priority, sla_state, sla_due_at, unresolved_age_minutes
FROM heliodesk_ticket_export
WHERE sla_state IN ('at-risk', 'breached')
ORDER BY CASE sla_state WHEN 'breached' THEN 0 ELSE 1 END, sla_due_at;

CREATE VIEW priority_distribution AS
SELECT priority, COUNT(*) AS ticket_count
FROM heliodesk_ticket_export
WHERE status IN ('open', 'pending_customer', 'investigating')
GROUP BY priority;
