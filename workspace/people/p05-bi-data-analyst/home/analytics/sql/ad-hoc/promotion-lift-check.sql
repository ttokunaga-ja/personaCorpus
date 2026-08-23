-- Compare synthetic promoted and baseline cohorts, holding channel constant.
WITH review_period(period, gmv_yen) AS (
 VALUES ('FY26 Q3 actual',84200000),('Plan',80000000),('Prior period',78600000)
)
SELECT period, gmv_yen, gmv_yen - 80000000 AS variance_to_plan_yen,
       ROUND(100.0*(gmv_yen-80000000)/80000000,2) AS variance_to_plan_pct
FROM review_period;
