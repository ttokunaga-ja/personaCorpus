-- Read-only operational queries for the synthetic Northstar Transit Cloud schema.
-- Assumed tables: service_minute_metrics, incident_events, and capacity_daily_samples.

-- Checkout SVC-17 availability by five-minute bucket during INC-260713.
SELECT
  date_trunc('hour', observed_at)
    + (floor(date_part('minute', observed_at) / 5) * interval '5 minutes') AS bucket_start,
  SUM(request_count) AS requests,
  SUM(error_count) AS errors,
  ROUND(100.0 * (1 - SUM(error_count)::numeric / NULLIF(SUM(request_count), 0)), 3) AS availability_percent
FROM service_minute_metrics
WHERE service_name = 'Checkout SVC-17'
  AND observed_at >= TIMESTAMP '2026-07-13 00:00:00'
  AND observed_at < TIMESTAMP '2026-07-14 00:00:00'
GROUP BY 1
ORDER BY 1;

-- Incident event timeline suitable for a post-incident review.
SELECT occurred_at, event_type, actor_role, summary
FROM incident_events
WHERE incident_id = 'INC-260713'
ORDER BY occurred_at, event_type;

-- Capacity trend with a fourteen-day linear extrapolation for review only.
WITH ordered AS (
  SELECT sample_date, peak_rps,
         row_number() OVER (ORDER BY sample_date) AS sample_number
  FROM capacity_daily_samples
  WHERE service_name = 'Checkout SVC-17'
), trend AS (
  SELECT regr_slope(peak_rps, sample_number) AS daily_growth_rps,
         max(peak_rps) AS observed_peak_rps
  FROM ordered
)
SELECT observed_peak_rps, daily_growth_rps,
       observed_peak_rps + (14 * daily_growth_rps) AS projected_peak_rps,
       (observed_peak_rps + (14 * daily_growth_rps)) * 1.30 AS recommended_capacity_rps
FROM trend;
