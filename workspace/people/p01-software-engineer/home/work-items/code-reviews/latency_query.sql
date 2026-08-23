-- Terminal-only percentile: pending/retry events are excluded before ranking.
WITH completed AS (
  SELECT latency_ms FROM gate_observations
  WHERE release = 'Product Alpha R7' AND observed_on = '2026-07-13' AND state IN ('accepted','rejected')
), ranked AS (
  SELECT latency_ms, cume_dist() OVER (ORDER BY latency_ms) AS percentile FROM completed
)
SELECT MIN(latency_ms) AS p95_ms FROM ranked WHERE percentile >= 0.95;
