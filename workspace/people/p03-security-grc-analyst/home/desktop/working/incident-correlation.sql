-- Local SQLite query for a synthetic incident-analysis table.
WITH ordered_events AS (
  SELECT
    incident_id,
    observed_at,
    event_type,
    LAG(event_type) OVER (PARTITION BY incident_id ORDER BY observed_at) AS prior_event
  FROM synthetic_security_events
  WHERE evidence_set = '184'
)
SELECT incident_id, observed_at, prior_event, event_type
FROM ordered_events
WHERE prior_event IS NOT NULL
  AND prior_event <> event_type
ORDER BY incident_id, observed_at;
