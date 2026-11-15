"""Reproducible synthetic metric utility for p08-full-002934."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p08-full-002934"
RUN_ID = "M-27"
MODEL = "Product Alpha"
SLICE_F1 = [0.874, 0.871, 0.868, 0.883, 0.862, 0.877, 0.875, 0.871]


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
