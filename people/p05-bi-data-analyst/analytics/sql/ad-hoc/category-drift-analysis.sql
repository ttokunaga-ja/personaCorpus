-- Category mix drift from a synthetic plan baseline.
WITH category(category, gmv_yen) AS (
 VALUES ('Home',24800000),('Electronics',22100000),('Apparel',19300000),('Beauty',10700000),('Other',7300000)
)
SELECT category,gmv_yen,ROUND(100.0*gmv_yen/84200000,2) AS actual_pct
FROM category;
