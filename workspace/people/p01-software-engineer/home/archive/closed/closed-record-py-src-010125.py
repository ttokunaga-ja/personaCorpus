"""Immutable closed snapshot parser for ARC-26Q3-010125."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ClosedRecord:
    record_id: str = "ARC-26Q3-010125"
    source_id: str = "p01-src-010125"
    project: str = "MIG-26Q3-W03"
    closed_on: str = "2026-09-09"
    state: str = "closed"

    def retention_label(self) -> str:
        return f"{self.project}/{self.record_id}/{self.state}"

if __name__ == "__main__":
    print(ClosedRecord().retention_label())
