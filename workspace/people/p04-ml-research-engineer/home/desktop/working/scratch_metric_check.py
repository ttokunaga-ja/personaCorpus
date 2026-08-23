"""Small, dependency-free metric guardrail for Alpha/Beta experiment notes."""

from dataclasses import dataclass


@dataclass(frozen=True)
class RunMetric:
    name: str
    f1: float
    precision: float
    recall: float

    def harmonic_f1(self) -> float:
        return 2 * self.precision * self.recall / (self.precision + self.recall)


RUNS = (
    RunMetric("Alpha", 0.873, 0.881, 0.865),
    RunMetric("Beta", 0.846, 0.852, 0.841),
)


def main() -> None:
    for run in RUNS:
        implied = run.harmonic_f1()
        drift = abs(run.f1 - implied)
        assert drift < 0.002, f"{run.name} reported F1 disagrees by {drift:.4f}"
        print(f"{run.name}: reported={run.f1:.3f} implied={implied:.3f} drift={drift:.4f}")
    print(f"Alpha-Beta F1 delta={RUNS[0].f1 - RUNS[1].f1:+.3f}")


if __name__ == "__main__":
    main()
