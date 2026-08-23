SELECT recognized_at::date AS recognized_date, SUM(recognized_refund_jpy) AS refund_jpy
FROM analytics_synthetic.fct_refund
WHERE recognition_status='recognized'
GROUP BY 1 ORDER BY 1;
