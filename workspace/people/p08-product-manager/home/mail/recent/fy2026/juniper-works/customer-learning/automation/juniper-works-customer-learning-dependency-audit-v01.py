"""Reproducible synthetic metric utility for p08-full-005802."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p08-full-005802"
RUN_ID = "M-27"
MODEL = "Product Alpha"
SLICE_F1 = [0.88, 0.878, 0.874, 0.872, 0.887, 0.866, 0.881, 0.86]


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
