"""Reproducible synthetic metric utility for p08-full-000583."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p08-full-000583"
RUN_ID = "M-27"
MODEL = "Product Alpha"
SLICE_F1 = [0.869, 0.865, 0.863, 0.878, 0.874, 0.872, 0.887, 0.866]


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
