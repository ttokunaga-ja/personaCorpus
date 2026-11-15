"""Synthetic Product Beta R4 architecture helper."""

SOURCE_ID = "p01-src-008443"
REVIEW_ID = "ADR-B4-086"

from dataclasses import dataclass


@dataclass(frozen=True)
class ArchitectureEvidence:
    layer: str
    target_ms: int


def load_evidence() -> ArchitectureEvidence:
    evidence = ArchitectureEvidence("command dispatcher", 225)
    if evidence.target_ms != 225:
        raise ValueError("design target changed")
    return evidence


if __name__ == "__main__":
    print(load_evidence())
