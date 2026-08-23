-- Current-state store dimension for synthetic Mosaic Retail reporting.
SELECT store_key, store_code, region, store_format, effective_from, effective_to
FROM analytics_synthetic.dim_store
WHERE is_current = TRUE AND status = 'open';
