"""Build a compact synthetic ForgeLine measurement summary from TSV input."""
from __future__ import annotations

import csv
from pathlib import Path


def summarize(source: Path) -> dict[str, int]:
    """Count open working actions by product without changing source records."""
    counts: dict[str, int] = {}
    with source.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle, delimiter="\t"):
            counts[row["product"]] = counts.get(row["product"], 0) + 1
    return counts


if __name__ == "__main__":
    print(summarize(Path("capa-data-working-copy.tsv")))
