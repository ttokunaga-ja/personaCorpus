SELECT order_line_id, source_loaded_at, completed_at,
       CASE WHEN source_loaded_at > completed_at + INTERVAL '1 day' THEN TRUE ELSE FALSE END AS requires_backfill
FROM analytics_synthetic.fct_order_line
WHERE completed_at::date BETWEEN DATE '2026-07-07' AND DATE '2026-07-13';
