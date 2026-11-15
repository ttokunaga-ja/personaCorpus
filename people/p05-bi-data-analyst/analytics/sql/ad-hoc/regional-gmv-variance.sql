-- Aggregate-only synthetic regional variance for MTR-09.
WITH region(region, actual_yen) AS (
 VALUES ('East',26400000),('West',22800000),('Central',18600000),('North',16400000)
)
SELECT region, actual_yen,
       ROUND(100.0 * actual_yen / 84200000,2) AS actual_share_pct
FROM region ORDER BY actual_yen DESC;
