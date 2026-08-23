CREATE OR REPLACE VIEW analytics_synthetic.v_mtr09_gmv AS
SELECT completed_at::date AS business_date, channel,
       SUM(merchandise_jpy-recognized_refund_jpy) AS gmv_jpy
FROM analytics_synthetic.fct_order_line
WHERE order_status='completed'
GROUP BY 1,2;
