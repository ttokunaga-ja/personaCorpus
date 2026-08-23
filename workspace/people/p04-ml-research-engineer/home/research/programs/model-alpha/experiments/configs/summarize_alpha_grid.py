"""Summarize fixed synthetic ablation observations without I/O beyond this folder."""
from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class GridObservation:
    arm: str
    macro_f1: float
    change: str


def rank(observations: list[GridObservation]) -> list[GridObservation]:
    return sorted(observations, key=lambda item: item.macro_f1, reverse=True)


def main() -> None:
    observations = [
        GridObservation("A0", 0.873, "full configuration"),
        GridObservation("A1", 0.858, "without class weights"),
        GridObservation("A2", 0.869, "without temperature calibration"),
        GridObservation("A3", 0.864, "96-token truncation"),
    ]
    best = rank(observations)[0]
    print(f"run R184: {best.arm} leads synthetic grid at F1 {best.macro_f1:.3f}")


if __name__ == "__main__":
    main()
