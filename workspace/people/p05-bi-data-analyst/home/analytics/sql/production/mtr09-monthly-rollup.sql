SELECT date_trunc('month', completed_at)::date AS month_start,
       SUM(merchandise_jpy-recognized_refund_jpy) AS gmv_jpy
FROM analytics_synthetic.fct_order_line WHERE order_status='completed'
GROUP BY 1 ORDER BY 1;
