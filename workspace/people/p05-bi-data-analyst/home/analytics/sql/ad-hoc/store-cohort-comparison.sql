-- Store cohorts are synthetic aggregate groups, never customer-level data.
WITH cohort(cohort, gmv_yen) AS (
 VALUES ('Online cohort',46300000),('Stores cohort',37900000)
)
SELECT cohort,gmv_yen,
       ROUND(100.0*gmv_yen/84200000,2) AS contribution_pct FROM cohort;
