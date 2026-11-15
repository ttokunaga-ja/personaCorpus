"""Self-contained offline summary for the synthetic Model Beta B-07 comparison."""
from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ModelMetric:
    name: str
    macro_f1: float
    precision: float
    recall: float


ALPHA = ModelMetric("Model Alpha M-14", 0.873, 0.881, 0.865)
BETA = ModelMetric("Model Beta B-07", 0.846, 0.852, 0.841)


def comparison_note(reference: ModelMetric, candidate: ModelMetric) -> str:
    delta = candidate.macro_f1 - reference.macro_f1
    decision = "retain reference" if delta < 0 else "review candidate"
    return f"run R184: {candidate.name} delta {delta:+.3f}; {decision} ({reference.name} F1 {reference.macro_f1:.3f})"


def main() -> None:
    print(comparison_note(ALPHA, BETA))


if __name__ == "__main__":
    main()
