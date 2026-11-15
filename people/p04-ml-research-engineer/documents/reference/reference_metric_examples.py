"""Small, dependency-free reference calculations for the synthetic R184 evaluation."""
from __future__ import annotations

from collections import Counter

LABEL_COUNTS = {"account_access": 492, "delivery_status": 476, "plan_change": 481, "refund_request": 462, "technical_help": 489}
SPLITS = {"S1": 1600, "S2": 800}
MODEL_MACRO_F1 = {"Alpha": 0.873, "Beta": 0.846}


def f1(true_positive: int, false_positive: int, false_negative: int) -> float:
    """Return class F1; an absent class has zero support in this worked example."""
    denominator = 2 * true_positive + false_positive + false_negative
    return 0.0 if denominator == 0 else 2 * true_positive / denominator


def macro_f1(confusions: dict[str, tuple[int, int, int]]) -> float:
    """Average per-label F1 values and require the agreed five-label taxonomy."""
    if set(confusions) != set(LABEL_COUNTS):
        raise ValueError("expected exactly the R184 support-label taxonomy")
    return sum(f1(*counts) for counts in confusions.values()) / len(confusions)


def support_total() -> int:
    return sum(LABEL_COUNTS.values())


if __name__ == "__main__":
    example = {label: (count - 20, 11, 20) for label, count in LABEL_COUNTS.items()}
    print(f"support={support_total()} split={sum(SPLITS.values())} macro_f1={macro_f1(example):.3f}")
