-- Local analytical query for the synthetic Northstar Transit Cloud exercise.
-- Scope: Checkout SVC-17, 2026-07-13, INC-260713, error budget 2.4%.
WITH alert_windows AS (
    SELECT alert_name, started_at, ended_at
    FROM synthetic_alert_events
    WHERE incident_id = 'INC-260713'
), request_windows AS (
    SELECT request_id, observed_at, latency_ms, status_code
    FROM synthetic_checkout_requests
    WHERE service_name = 'Checkout SVC-17'
      AND observed_at >= TIMESTAMP '2026-07-13 00:00:00'
      AND observed_at < TIMESTAMP '2026-07-14 00:00:00'
)
SELECT
    a.alert_name,
    COUNT(r.request_id) AS correlated_requests,
    ROUND(AVG(r.latency_ms), 2) AS average_latency_ms,
    SUM(CASE WHEN r.status_code >= 500 THEN 1 ELSE 0 END) AS server_errors
FROM alert_windows AS a
LEFT JOIN request_windows AS r
  ON r.observed_at >= a.started_at
 AND r.observed_at < COALESCE(a.ended_at, TIMESTAMP '2026-07-14 00:00:00')
GROUP BY a.alert_name
ORDER BY correlated_requests DESC, a.alert_name;
