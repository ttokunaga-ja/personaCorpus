-- Synthetic Mosaic Retail Analytics analysis; no production connection required.
-- FY26 Q3 | metric MTR-09 | review date 2026-07-13
WITH channel(channel, gmv_yen, share) AS (
  VALUES ('Online',46300000,0.55),('Stores',37900000,0.45)
)
SELECT channel, gmv_yen, ROUND(gmv_yen * 1.0 / SUM(gmv_yen) OVER (), 4) AS actual_share,
       ROUND((share - gmv_yen * 1.0 / SUM(gmv_yen) OVER ()) * 84200000, 0) AS mix_variance_yen
FROM channel ORDER BY gmv_yen DESC;
