SELECT order_line_id, merchandise_jpy, tax_jpy,
       merchandise_jpy - recognized_refund_jpy AS mtr09_tax_excluded_jpy
FROM analytics_synthetic.fct_order_line
WHERE order_status='completed';
