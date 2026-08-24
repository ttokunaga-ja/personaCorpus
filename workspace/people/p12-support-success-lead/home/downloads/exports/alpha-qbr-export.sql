-- HelioDesk synthetic Customer Alpha QBR export.
-- Structured text only: run against an approved reporting replica with named parameters.
-- Example parameters: :customer_segment = 'Customer Alpha',
-- :period_start = DATE '2026-07-01', :period_end = DATE '2026-08-01'.
-- Intended for an approved reporting replica; bindings are supplied separately.

WITH scoped_cases AS (
    SELECT
        c.case_id,
        c.opened_at,
        c.resolved_at,
        c.priority,
        c.sla_target_hours,
        c.first_response_minutes,
        c.reopened_count
    FROM reporting.support_cases AS c
    WHERE c.customer_segment = :customer_segment
      AND c.opened_at >= :period_start
      AND c.opened_at < :period_end
), case_durations AS (
    SELECT
        case_id,
        priority,
        sla_target_hours,
        first_response_minutes,
        reopened_count,
        EXTRACT(EPOCH FROM (resolved_at - opened_at)) / 3600.0 AS resolution_hours
    FROM scoped_cases
    WHERE resolved_at IS NOT NULL
)
SELECT
    :customer_segment AS customer_segment,
    :period_start AS period_start,
    :period_end AS period_end,
    COUNT(*) AS resolved_cases,
    ROUND(AVG(resolution_hours)::numeric, 2) AS avg_resolution_hours,
    ROUND(PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY resolution_hours)::numeric, 2) AS median_resolution_hours,
    ROUND(AVG(first_response_minutes)::numeric, 1) AS avg_first_response_minutes,
    SUM(CASE WHEN resolution_hours <= sla_target_hours THEN 1 ELSE 0 END) AS resolved_within_sla,
    SUM(reopened_count) AS reopened_events
FROM case_durations;
