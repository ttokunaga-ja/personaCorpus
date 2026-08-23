SELECT completed_at::date AS business_date,
       SUM(merchandise_jpy - recognized_refund_jpy) AS mtr09_gmv_jpy
FROM analytics_synthetic.fct_order_line
WHERE order_status = 'completed'
GROUP BY 1 ORDER BY 1;
