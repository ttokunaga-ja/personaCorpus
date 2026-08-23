"""Local-only validation for the fictional Mosaic Retail Analytics S2 GMV export."""
from __future__ import annotations

import csv
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SNAPSHOT = ROOT / "onedrive-gmv-snapshot.csv"

def main() -> None:
    with SNAPSHOT.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    daily = [row for row in rows if row["record_type"] == "daily"]
    total = next(row for row in rows if row["record_type"] == "period_total")
    gross = sum(int(row["gross_gmv_jpy"]) for row in daily)
    refunds = sum(int(row["recognized_refunds_jpy"]) for row in daily)
    net = sum(int(row["net_gmv_jpy"]) for row in daily)
    channels = sum(int(row["online_net_gmv_jpy"]) + int(row["stores_net_gmv_jpy"]) for row in daily)
    assert (gross, refunds, net) == (86_000_000, 1_800_000, 84_200_000)
    assert gross - refunds == net == channels
    assert int(total["net_gmv_jpy"]) == net
    assert total["metric_id"] == "MTR-09"
    print("PASS: 7 daily rows reconcile to ¥84.2M net GMV; local-only validation.")

if __name__ == "__main__":
    main()
