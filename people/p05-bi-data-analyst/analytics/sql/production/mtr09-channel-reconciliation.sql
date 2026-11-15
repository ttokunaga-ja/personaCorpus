WITH by_channel AS (
 SELECT channel, SUM(merchandise_jpy-recognized_refund_jpy) AS gmv_jpy
 FROM analytics_synthetic.fct_order_line WHERE order_status='completed' GROUP BY 1
)
SELECT channel, gmv_jpy, SUM(gmv_jpy) OVER () AS reconciled_total_jpy FROM by_channel;
