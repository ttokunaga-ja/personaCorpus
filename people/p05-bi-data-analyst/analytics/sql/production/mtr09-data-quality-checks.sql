SELECT
  COUNT(*) FILTER (WHERE merchandise_jpy < 0) AS negative_merchandise_rows,
  COUNT(*) FILTER (WHERE recognized_refund_jpy > merchandise_jpy) AS excessive_refund_rows,
  COUNT(*) FILTER (WHERE completed_at IS NULL) AS missing_completion_rows
FROM analytics_synthetic.fct_order_line;
