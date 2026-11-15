"""Reproducible synthetic metric utility for p04-full-004599."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p04-full-004599"
RUN_ID = "R184"
MODEL = "Alpha M-14"
SLICE_F1 = [0.874, 0.87, 0.868, 0.883, 0.862, 0.877, 0.874, 0.871]


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
