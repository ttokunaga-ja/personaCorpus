#!/usr/bin/env python3
"""Calculate a conservative capacity projection from local daily samples."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def load_samples(path: Path) -> list[tuple[str, float]]:
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        if not reader.fieldnames or set(("date", "peak_rps")) - set(reader.fieldnames):
            raise ValueError("CSV must include date and peak_rps columns")
        samples = [(row["date"].strip(), float(row["peak_rps"])) for row in reader]
    if len(samples) < 2 or any(value < 0 for _, value in samples):
        raise ValueError("provide at least two non-negative daily samples")
    return samples


def main() -> int:
    parser = argparse.ArgumentParser(description="Project peak request capacity without external calls.")
    parser.add_argument("input", type=Path, help="CSV with date,peak_rps")
    parser.add_argument("--headroom", type=float, default=0.30, help="fractional capacity headroom")
    args = parser.parse_args()
    if args.headroom < 0 or args.headroom > 2:
        parser.error("--headroom must be between 0 and 2")
    samples = load_samples(args.input)
    first, last = samples[0][1], samples[-1][1]
    daily_growth = (last - first) / (len(samples) - 1)
    projected = max(0.0, last + daily_growth * 14)
    required = projected * (1 + args.headroom)
    print("Northstar Transit Cloud capacity projection")
    print(f"observed_peak_rps={last:.1f}")
    print(f"daily_growth_rps={daily_growth:.2f}")
    print(f"fourteen_day_peak_rps={projected:.1f}")
    print(f"recommended_capacity_rps={required:.1f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
