#!/usr/bin/env python3
"""Score a local vendor questionnaire from explicit, defensive answer values."""
from __future__ import annotations

import argparse
import csv
from pathlib import Path


WEIGHTS = {"yes": 0, "partial": 1, "no": 3, "not-applicable": 0}


def main() -> int:
    parser = argparse.ArgumentParser(description="Score CSV questionnaire answers")
    parser.add_argument("input", type=Path)
    args = parser.parse_args()
    total = 0
    with args.input.open(encoding="utf-8", newline="") as handle:
        for row in csv.DictReader(handle):
            answer = row.get("answer", "").strip().lower()
            if answer not in WEIGHTS:
                raise ValueError(f"unsupported answer: {answer!r}")
            total += WEIGHTS[answer]
    band = "review" if total >= 6 else "follow-up" if total >= 2 else "acceptable"
    print(f"risk_points={total}\nrisk_band={band}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
