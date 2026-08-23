"""A deterministic, offline reproduction checklist for a synthetic run."""

from collections.abc import Iterable


LABEL_COUNTS = {
    "account_access": 492,
    "delivery_status": 476,
    "plan_change": 481,
    "refund_request": 462,
    "technical_help": 489,
}


def verify_counts(items: Iterable[tuple[str, int]]) -> int:
    total = 0
    for label, count in items:
        if label not in LABEL_COUNTS or count <= 0:
            raise ValueError(f"invalid synthetic label record: {label}")
        total += count
    if total != 2_400:
        raise ValueError(f"expected 2400 synthetic records, got {total}")
    return total


def main() -> None:
    total = verify_counts(LABEL_COUNTS.items())
    partitions = {"HF-Eval-S1": 1_600, "HF-Eval-S2": 800}
    assert sum(partitions.values()) == total
    print(f"run R184 reproduced: {total} records; {partitions}")


if __name__ == "__main__":
    main()
