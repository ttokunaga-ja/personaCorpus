-- structured_text
-- Saved view: Personal QBR and support-queue pulse
-- Provenance: synthetic HelioDesk training data, snapshot date 2026-08-04.
-- This query contains no credentials and reads only inline example data.

DROP VIEW IF EXISTS personal_qbr_support_pulse;

CREATE VIEW personal_qbr_support_pulse AS
WITH queue_snapshot AS (
  SELECT
    'Customer Alpha' AS customer_name,
    9 AS unresolved_items,
    3 AS at_risk_items,
    86 AS adoption_index,
    'administrator access and repeat clarification' AS review_focus
  UNION ALL
  SELECT
    'Customer Beta' AS customer_name,
    5 AS unresolved_items,
    2 AS at_risk_items,
    91 AS adoption_index,
    'follow-up ownership review' AS review_focus
  UNION ALL
  SELECT
    'Other managed customers' AS customer_name,
    3 AS unresolved_items,
    1 AS at_risk_items,
    88 AS adoption_index,
    'confirm owners and next-update coverage' AS review_focus
), service_policy AS (
  SELECT '2026-08-04' AS snapshot_date, 4 AS acknowledgement_sla_hours
), summarized AS (
  SELECT
    p.snapshot_date,
    q.customer_name,
    q.unresolved_items,
    q.at_risk_items,
    p.acknowledgement_sla_hours,
    q.adoption_index,
    q.review_focus,
    SUM(q.unresolved_items) OVER () AS portfolio_unresolved_items,
    ROUND(100.0 * q.unresolved_items / SUM(q.unresolved_items) OVER (), 1)
      AS unresolved_share_percent
  FROM queue_snapshot AS q
  CROSS JOIN service_policy AS p
)
SELECT
  snapshot_date,
  customer_name,
  unresolved_items,
  portfolio_unresolved_items,
  unresolved_share_percent,
  at_risk_items,
  acknowledgement_sla_hours,
  adoption_index,
  review_focus,
  CASE
    WHEN at_risk_items > 0 THEN 'Review in next support-success sync'
    ELSE 'Monitor in normal cadence'
  END AS recommended_next_step
FROM summarized
ORDER BY unresolved_items DESC, customer_name;

-- Expected synthetic portfolio total: 17 unresolved items.
