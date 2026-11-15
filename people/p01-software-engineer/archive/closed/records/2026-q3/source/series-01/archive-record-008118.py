"""Immutable closed snapshot parser for ARC-26Q3-008118."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ClosedRecord:
    record_id: str = "ARC-26Q3-008118"
    source_id: str = "p01-src-008118"
    project: str = "ALPHA-R7"
    closed_on: str = "2026-07-20"
    state: str = "closed"

    def retention_label(self) -> str:
        return f"{self.project}/{self.record_id}/{self.state}"

if __name__ == "__main__":
    print(ClosedRecord().retention_label())
