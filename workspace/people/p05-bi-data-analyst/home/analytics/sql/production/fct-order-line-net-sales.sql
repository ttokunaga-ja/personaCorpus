-- Grain: one completed synthetic order line; tax excluded and refund netted.
SELECT order_line_id, order_id, completed_at::date AS business_date, channel,
       merchandise_jpy - recognized_refund_jpy AS net_merchandise_jpy
FROM analytics_synthetic.fct_order_line
WHERE order_status = 'completed';
