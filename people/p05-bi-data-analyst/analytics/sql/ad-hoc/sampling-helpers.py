"""Deterministic helpers for aggregate-only MTR-09 examples."""
from __future__ import annotations
from collections.abc import Iterable

def share(values: Iterable[int]) -> list[float]:
    items = list(values)
    total = sum(items)
    if total <= 0:
        raise ValueError("aggregate total must be positive")
    return [round(value / total, 6) for value in items]

def variance(actual: int, baseline: int) -> dict[str, int]:
    return {"actual_yen": actual, "baseline_yen": baseline, "variance_yen": actual - baseline}

if __name__ == "__main__":
    assert sum(share([46_300_000, 37_900_000])) == 1.0
    print(variance(84_200_000, 80_000_000))
