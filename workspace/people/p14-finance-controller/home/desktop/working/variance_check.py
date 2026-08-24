#!/usr/bin/env python3
"""Validate Cedar Works' synthetic cash-variance bridge without dependencies."""
import csv
import pathlib
import sys

EXPECTED = {"Receipt timing": 7.2, "Vendor timing": 3.1, "Fees": 1.3, "FX": 0.8}
BANK_CASH = 486.2
LEDGER_CASH = 498.6

def load_items(path):
    with open(path, newline="", encoding="utf-8") as handle:
        return {row["workstream"]: float(row["amount_jpy_m"]) for row in csv.DictReader(handle)}

def main():
    path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(__file__).with_name("open-close-items.csv")
    observed = load_items(path)
    assert observed == EXPECTED, f"Unexpected bridge inputs: {observed}"
    component_total = round(sum(observed.values()), 1)
    cash_variance = round(LEDGER_CASH - BANK_CASH, 1)
    assert component_total == 12.4, f"Components total {component_total}, expected 12.4"
    assert cash_variance == 12.4, f"Cash variance {cash_variance}, expected 12.4"
    assert component_total == cash_variance, "Bridge does not reconcile"
    print(f"PASS: JPY {component_total:.1f}m unfavorable bridge reconciles to ledger less bank cash.")

if __name__ == "__main__":
    main()
