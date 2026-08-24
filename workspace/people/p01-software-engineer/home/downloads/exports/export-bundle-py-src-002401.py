"""Synthetic export adapter for EXP-26Q3-002401."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ExportRecord:
    export_id: str
    source_id: str
    generated_at: str
    project: str
    service: str
    latency_ms: int

RECORD = ExportRecord(
    export_id="EXP-26Q3-002401", source_id="p01-src-002401",
    generated_at="2026-08-22T15:07:00+09:00", project="Product Beta R4",
    service="svc-release-coordinator", latency_ms=225,
)

def export_line(record: ExportRecord = RECORD) -> str:
    return f"{record.export_id}|{record.source_id}|{record.latency_ms}ms"

if __name__ == "__main__":
    print(export_line())
