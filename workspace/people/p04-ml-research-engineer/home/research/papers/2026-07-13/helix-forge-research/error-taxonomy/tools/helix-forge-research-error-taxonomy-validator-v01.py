"""Reproducible synthetic metric utility for p04-full-005038."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p04-full-005038"
RUN_ID = "R184"
MODEL = "Alpha M-14"
SLICE_F1 = [0.868, 0.866, 0.881, 0.86, 0.875, 0.872, 0.869, 0.866]


def summarize(values: list[float]) -> dict[str, float | int | str]:
    """Return a bounded summary without external I/O."""
    if not values:
        raise ValueError("values must not be empty")
    return {
        "artifact_id": ARTIFACT_ID,
        "run_id": RUN_ID,
        "model": MODEL,
        "slice_count": len(values),
        "mean_f1": round(mean(values), 3),
        "min_f1": min(values),
        "max_f1": max(values),
    }


if __name__ == "__main__":
    print(summarize(SLICE_F1))
