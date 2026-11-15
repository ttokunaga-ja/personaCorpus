#!/usr/bin/env python3
"""Validate the synthetic MTR-09 lineage quality contract."""
import json
from pathlib import Path

REQUIRED_CHECKS = {"freshness", "unique_order_line", "nonnegative_gmv", "reconciled_total"}

def validate(path: Path) -> None:
    contract = json.loads(path.read_text())
    assert contract["metric_id"] == "MTR-09"
    assert {item["name"] for item in contract["checks"]} == REQUIRED_CHECKS
    assert contract["checks"][-1]["expected_jpy"] == 84_200_000

if __name__ == "__main__":
    validate(Path(__file__).with_name("lineage-quality-contract.json"))
    print("lineage quality contract: valid")
