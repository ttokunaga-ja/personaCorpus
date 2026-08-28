"""Reproducible synthetic metric utility for p04-full-008078."""
from __future__ import annotations

from statistics import mean

ARTIFACT_ID = "p04-full-008078"
RUN_ID = "R184"
MODEL = "Alpha M-14"
SLICE_F1 = [0.864, 0.879, 0.858, 0.873, 0.871, 0.867, 0.882, 0.88]


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
