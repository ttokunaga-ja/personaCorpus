"""Immutable closed snapshot parser for ARC-26Q3-003453."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ClosedRecord:
    record_id: str = "ARC-26Q3-003453"
    source_id: str = "p01-src-003453"
    project: str = "MIG-26Q3-W03"
    closed_on: str = "2026-09-12"
    state: str = "closed"

    def retention_label(self) -> str:
        return f"{self.project}/{self.record_id}/{self.state}"

if __name__ == "__main__":
    print(ClosedRecord().retention_label())
