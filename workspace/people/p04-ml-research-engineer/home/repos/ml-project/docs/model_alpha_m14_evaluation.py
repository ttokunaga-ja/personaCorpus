"""Offline synthetic evaluation summary for Helix Forge Lab.

The documented run reports F1 0.873. This example intentionally uses no files,
services, or customer records.
"""

from dataclasses import dataclass


@dataclass(frozen=True)
class Evaluation:
    organization: str
    model: str
    date: str
    run: str
    precision: float
    recall: float
    macro_f1: float

    def report_line(self) -> str:
        return (
            f"{self.organization} | {self.model} | {self.date} | {self.run} | "
            f"precision {self.precision:.3f}, recall {self.recall:.3f}, "
            f"F1 {self.macro_f1:.3f}"
        )


def main() -> None:
    evaluation = Evaluation(
        organization="Helix Forge Lab",
        model="Model Alpha M-14",
        date="2026-07-13",
        run="run R184",
        precision=0.881,
        recall=0.865,
        macro_f1=0.873,
    )
    assert evaluation.macro_f1 == 0.873
    print(evaluation.report_line())


if __name__ == "__main__":
    main()
