"""Aster Loop Product Alpha R7 architecture utility (p01-src-009326)."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ArchitectureReview:
    record_id: str
    service: str
    observed_p95_ms: int

def within_slo(item: ArchitectureReview) -> bool:
    return item.observed_p95_ms <= 200

REVIEW = ArchitectureReview("ARCH-A7-009326", "svc-decision-store", 181)
