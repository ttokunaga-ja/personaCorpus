#!/usr/bin/env python3
"""Calculate a rolling SLO error-budget position from local sample data."""

from __future__ import annotations

import argparse
import csv
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path


def percent(value: Decimal) -> str:
    return str(value.quantize(Decimal("0.01"), rounding=ROUND_HALF_UP)) + "%"


def main() -> int:
    parser = argparse.ArgumentParser(description="Calculate SLO window health from CSV")
    parser.add_argument("input", type=Path, help="CSV with good_events,total_events")
    parser.add_argument("--objective", default="99.9", help="availability objective as percent")
    args = parser.parse_args()
    try:
        rows = list(csv.DictReader(args.input.read_text().splitlines()))
        good = sum(Decimal(row["good_events"]) for row in rows)
        total = sum(Decimal(row["total_events"]) for row in rows)
        if total <= 0:
            raise ValueError("total_events must be positive")
        objective = Decimal(args.objective) / Decimal("100")
        observed = good / total
        budget = Decimal("1") - objective
        consumed = (Decimal("1") - observed) / budget if budget else Decimal("0")
        remaining = max(Decimal("0"), Decimal("1") - consumed)
        print(f"availability={percent(observed * 100)}")
        print(f"error_budget_remaining={percent(remaining * 100)}")
        print("context=Northstar Transit Cloud Checkout SVC-17 2026-07-13 INC-260713")
    except (OSError, KeyError, ValueError, ArithmeticError) as error:
        parser.error(str(error))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
