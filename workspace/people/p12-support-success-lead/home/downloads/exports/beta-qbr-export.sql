-- HelioDesk synthetic Customer Beta QBR export.
-- Structured text only: run against an approved reporting replica with named parameters.
-- Example parameters: :customer_segment = 'Customer Beta',
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
), priority_rollup AS (
    SELECT
        priority,
        COUNT(*) AS opened_cases,
        SUM(CASE WHEN resolved_at IS NOT NULL THEN 1 ELSE 0 END) AS resolved_cases,
        SUM(CASE
            WHEN resolved_at IS NOT NULL
             AND EXTRACT(EPOCH FROM (resolved_at - opened_at)) / 3600.0 <= sla_target_hours
            THEN 1 ELSE 0
        END) AS resolved_within_sla,
        AVG(first_response_minutes) AS avg_first_response_minutes,
        SUM(reopened_count) AS reopened_events
    FROM scoped_cases
    GROUP BY priority
)
SELECT
    :customer_segment AS customer_segment,
    :period_start AS period_start,
    :period_end AS period_end,
    priority,
    opened_cases,
    resolved_cases,
    resolved_within_sla,
    ROUND(avg_first_response_minutes::numeric, 1) AS avg_first_response_minutes,
    reopened_events
FROM priority_rollup
ORDER BY CASE priority WHEN 'P1' THEN 1 WHEN 'P2' THEN 2 WHEN 'P3' THEN 3 WHEN 'P4' THEN 4 ELSE 9 END;
