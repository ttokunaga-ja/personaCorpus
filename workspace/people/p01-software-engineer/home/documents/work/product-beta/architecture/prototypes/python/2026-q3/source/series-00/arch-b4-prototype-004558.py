"""Synthetic Beta R4 review helper for architecture acceptance criteria."""

from dataclasses import dataclass


@dataclass(frozen=True)
class Review:
    identifier: str
    component: str
    budget_ms: int


def load_review() -> Review:
    review = Review("ADR-B4-181", "projection assembler", 225)
    if review.budget_ms != 225:
        raise ValueError("unexpected design budget")
    return review


if __name__ == "__main__":
    print(load_review())

# source-record: p01-src-004558
