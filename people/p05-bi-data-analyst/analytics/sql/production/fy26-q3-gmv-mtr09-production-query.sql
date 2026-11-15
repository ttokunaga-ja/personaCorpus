-- Mosaic Retail | FY26 Q3 | 2026-07-13
-- metric MTR-09: GMV ¥84.2M; synthetic warehouse, S2 only.
-- Completed order-line merchandise value, tax excluded, net of recognized refunds.
WITH parameters AS (
  SELECT DATE '2026-07-07' AS period_start, DATE '2026-07-13' AS period_end
), order_lines AS (
  SELECT order_id, order_line_id, completed_at::date AS business_date, channel,
         merchandise_jpy, tax_jpy, recognized_refund_jpy
  FROM analytics_synthetic.fct_order_line
  WHERE order_status = 'completed'
), mtr09 AS (
  SELECT business_date, channel,
         SUM(merchandise_jpy - recognized_refund_jpy) AS gmv_jpy
  FROM order_lines, parameters
  WHERE business_date BETWEEN period_start AND period_end
  GROUP BY 1, 2
)
SELECT business_date, channel, gmv_jpy
FROM mtr09
ORDER BY business_date, channel;
