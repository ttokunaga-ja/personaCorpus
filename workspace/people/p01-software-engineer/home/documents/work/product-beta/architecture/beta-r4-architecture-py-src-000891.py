"""Synthetic Beta R4 architecture helper: failure classification."""

from dataclasses import dataclass


@dataclass(frozen=True)
class ReviewBudget:
    review_id: str
    target_ms: int
    allocated_ms: int


def validate_budget() -> ReviewBudget:
    budget = ReviewBudget("ADR-B4-096", 225, 151)
    if budget.allocated_ms > budget.target_ms:
        raise ValueError("allocation exceeds the Beta R4 design target")
    return budget


if __name__ == "__main__":
    print(validate_budget())
