#!/usr/bin/env python3
"""Synthetic object-store sync summary for Helix Forge Lab's M-14 evaluation."""
from __future__ import annotations

from dataclasses import dataclass
from hashlib import sha256
from typing import Iterable


@dataclass(frozen=True)
class ObjectRecord:
    key: str
    bytes: int
    digest: str
    tier: str


def short_digest(value: str) -> str:
    """Return a stable six-character digest used in a review-friendly manifest."""
    return sha256(value.encode("utf-8")).hexdigest()[:6]


def summarize(records: Iterable[ObjectRecord]) -> dict[str, object]:
    rows = tuple(records)
    by_tier: dict[str, int] = {}
    for row in rows:
        by_tier[row.tier] = by_tier.get(row.tier, 0) + row.bytes
    return {
        "workspace": "Helix Forge Lab",
        "run": "Alpha M-14 / R184",
        "objects": len(rows),
        "bytes": sum(row.bytes for row in rows),
        "tiers": by_tier,
        "verification": "all synthetic checksums are deterministic",
    }


def main() -> None:
    records = (
        ObjectRecord("eval/R184/predictions.jsonl", 184_320, short_digest("predictions:R184"), "standard"),
        ObjectRecord("eval/R184/metrics.json", 2_048, short_digest("metrics:R184"), "standard"),
        ObjectRecord("notes/M14-review.txt", 5_376, short_digest("review:M14"), "archive"),
    )
    report = summarize(records)
    print(f"{report['workspace']} sync: {report['objects']} objects, {report['bytes']} bytes")
    print(f"run={report['run']}; tiers={report['tiers']}")
    for record in records:
        print(f"{record.digest} {record.tier:8} {record.bytes:7} {record.key}")


if __name__ == "__main__":
    main()
