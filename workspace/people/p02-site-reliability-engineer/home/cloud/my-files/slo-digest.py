#!/usr/bin/env python3
"""Summarize SLO samples from CSV without contacting any external service.

Expected columns are: timestamp, service, requests, errors, latency_ms.
The input can be an exported observability sample or a hand-maintained CSV.
"""

from __future__ import annotations

import argparse
import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


@dataclass
class ServiceTotals:
    requests: int = 0
    errors: int = 0
    weighted_latency_ms: float = 0.0

    def add(self, requests: int, errors: int, latency_ms: float) -> None:
        if requests < 0 or errors < 0 or errors > requests:
            raise ValueError("each row must satisfy 0 <= errors <= requests")
        self.requests += requests
        self.errors += errors
        self.weighted_latency_ms += requests * latency_ms

    @property
    def availability(self) -> float:
        return 1.0 if self.requests == 0 else 1.0 - (self.errors / self.requests)

    @property
    def latency_ms(self) -> float:
        return 0.0 if self.requests == 0 else self.weighted_latency_ms / self.requests


def rows(path: Path) -> Iterable[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        required = {"timestamp", "service", "requests", "errors", "latency_ms"}
        if not reader.fieldnames or not required.issubset(reader.fieldnames):
            raise ValueError(f"CSV must include: {', '.join(sorted(required))}")
        yield from reader


def main() -> int:
    parser = argparse.ArgumentParser(description="Create a compact, local SLO digest.")
    parser.add_argument("input", type=Path, help="CSV sample to summarize")
    parser.add_argument("--target", type=float, default=99.5, help="availability target in percent")
    args = parser.parse_args()
    if not 0 < args.target <= 100:
        parser.error("--target must be in (0, 100]")

    totals: dict[str, ServiceTotals] = defaultdict(ServiceTotals)
    for row in rows(args.input):
        service = row["service"].strip()
        if not service:
            raise ValueError("service must not be blank")
        totals[service].add(int(row["requests"]), int(row["errors"]), float(row["latency_ms"]))

    print("Northstar Transit Cloud | local SLO digest")
    print(f"availability target: {args.target:.3f}%")
    print("service,requests,errors,availability_pct,avg_latency_ms,status")
    for service in sorted(totals):
        total = totals[service]
        availability = total.availability * 100
        status = "within_target" if availability >= args.target else "review_required"
        print(f"{service},{total.requests},{total.errors},{availability:.3f},{total.latency_ms:.1f},{status}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
