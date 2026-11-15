"""Offline slice-budget probe for the current intent-classification experiments."""

from collections import defaultdict

LABEL_COUNTS = {
    "account_access": 492,
    "delivery_status": 476,
    "plan_change": 481,
    "refund_request": 462,
    "technical_help": 489,
}
SPLITS = {"S1": 1600, "S2": 800}


def allocate_review_budget(counts: dict[str, int], budget: int = 40) -> dict[str, int]:
    """Allocate a deterministic proportional review budget, retaining every label."""
    total = sum(counts.values())
    allocation = {label: max(1, round(budget * count / total)) for label, count in counts.items()}
    while sum(allocation.values()) > budget:
        label = max(allocation, key=lambda key: (allocation[key], counts[key]))
        if allocation[label] == 1:
            break
        allocation[label] -= 1
    while sum(allocation.values()) < budget:
        label = max(allocation, key=lambda key: (counts[key] / allocation[key], key))
        allocation[label] += 1
    return allocation


def main() -> None:
    review = allocate_review_budget(LABEL_COUNTS)
    checks = defaultdict(int)
    for label, count in LABEL_COUNTS.items():
        checks["labels"] += 1
        print(f"{label:16} corpus={count:3} review={review[label]:2}")
    print(f"total={sum(LABEL_COUNTS.values())}; splits={SPLITS}; labels={checks['labels']}")


if __name__ == "__main__":
    main()
