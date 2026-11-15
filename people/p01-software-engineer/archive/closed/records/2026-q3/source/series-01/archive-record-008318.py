"""Immutable closed snapshot parser for ARC-26Q3-008318."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ClosedRecord:
    record_id: str = "ARC-26Q3-008318"
    source_id: str = "p01-src-008318"
    project: str = "MIG-26Q3-W01"
    closed_on: str = "2026-09-10"
    state: str = "closed"

    def retention_label(self) -> str:
        return f"{self.project}/{self.record_id}/{self.state}"

if __name__ == "__main__":
    print(ClosedRecord().retention_label())
