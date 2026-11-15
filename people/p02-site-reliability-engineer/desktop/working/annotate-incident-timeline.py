#!/usr/bin/env python3
"""Annotate a synthetic incident timeline with relative elapsed time.

Input is newline-delimited JSON with a ``timestamp`` and ``event`` field.
The tool writes annotated JSON to stdout and never contacts a remote service.
"""

from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def parse_timestamp(value: str) -> datetime:
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("timestamps must include a UTC offset")
    return parsed.astimezone(timezone.utc)


def annotate(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    ordered = sorted(rows, key=lambda row: parse_timestamp(str(row["timestamp"])))
    if not ordered:
        return []
    start = parse_timestamp(str(ordered[0]["timestamp"]))
    for sequence, row in enumerate(ordered, start=1):
        observed = parse_timestamp(str(row["timestamp"]))
        row["sequence"] = sequence
        row["elapsed_seconds"] = int((observed - start).total_seconds())
        row["incident"] = row.get("incident", "INC-260713")
    return ordered


def main() -> int:
    parser = argparse.ArgumentParser(description="Annotate incident timeline JSONL")
    parser.add_argument("input", type=Path, help="JSONL input file")
    args = parser.parse_args()
    try:
        rows = [json.loads(line) for line in args.input.read_text().splitlines() if line.strip()]
        for row in annotate(rows):
            print(json.dumps(row, sort_keys=True))
    except (OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"timeline annotation failed: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
