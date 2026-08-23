-- Sensitivity excludes refunds posted after the FY26 Q3 reporting cut.
WITH refunds(bucket, refund_yen) AS (
 VALUES ('Controlled refunds',1800000),('Gross GMV',86000000),('Controlled actual GMV',84200000)
)
SELECT bucket, refund_yen,
       ROUND(100.0*refund_yen/86000000,2) AS gross_gmv_reference_pct FROM refunds;
