# MTR-09 GMV lineage overview

**Mosaic Retail** · **FY26 Q3** · refreshed **2026-07-13**

MTR-09 records gross merchandise value as **GMV ¥84.2M**. The synthetic source order lines land from OneDrive in Warehouse S2, pass duplicate and test-order controls, then aggregate to the daily GMV mart.

## Accountabilities

Data Operations owns intake; Analytics Engineering owns transformations; BI Analytics certifies the semantic metric. Freshness is monitored at 180 minutes and reconciliation is reviewed at each morning checkpoint.

## Trace path

`sales-order-lines` → `stg_order_lines` → `mart_daily_gmv` → sales and product dashboards.
