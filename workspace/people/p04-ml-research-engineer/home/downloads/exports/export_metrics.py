"""Small, dependency-free metric summary for the R184 export bundle."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class EvaluationRun:
    name: str
    precision: float
    recall: float
    reported_f1: float

    def harmonic_f1(self) -> float:
        return round(2 * self.precision * self.recall / (self.precision + self.recall), 3)

    def is_consistent(self) -> bool:
        return abs(self.harmonic_f1() - self.reported_f1) <= 0.001


RUNS = (
    EvaluationRun("Alpha M-14", 0.881, 0.865, 0.873),
    EvaluationRun("Beta B-07", 0.852, 0.841, 0.846),
)
LABEL_COUNTS = {
    "account_access": 492,
    "delivery_status": 476,
    "plan_change": 481,
    "refund_request": 462,
    "technical_help": 489,
}


def weighted_average(runs: Iterable[EvaluationRun]) -> dict[str, float]:
    values = tuple(runs)
    if not values:
        raise ValueError("at least one evaluation run is required")
    return {
        "precision": round(sum(run.precision for run in values) / len(values), 3),
        "recall": round(sum(run.recall for run in values) / len(values), 3),
        "f1": round(sum(run.reported_f1 for run in values) / len(values), 3),
    }


def build_summary() -> dict[str, object]:
    total = sum(LABEL_COUNTS.values())
    return {
        "lab": "Helix Forge Lab",
        "run_id": "R184",
        "date": "2026-07-13",
        "suite": {"name": "HF-Eval-S1/S2", "version": "3.2", "total": total, "S1": 1600, "S2": 800},
        "labels": LABEL_COUNTS,
        "averages": weighted_average(RUNS),
        "consistent": all(run.is_consistent() for run in RUNS),
    }


if __name__ == "__main__":
    summary = build_summary()
    print(f"{summary['lab']} {summary['run_id']} labels={summary['suite']['total']}")
    print(f"mean F1={summary['averages']['f1']:.3f} consistent={summary['consistent']}")
