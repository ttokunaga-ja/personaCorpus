"""Synthetic decision lifecycle evaluator for p01-src-001518."""
from dataclasses import dataclass

@dataclass(frozen=True)
class DecisionRecord:
    record_id: str
    adr_id: str
    status: str
    latency_budget_ms: int

RECORD = DecisionRecord("p01-src-001518", "ADR-B4-126", "released", 200)

def is_ready(record: DecisionRecord, observed_p95_ms: int = 184) -> bool:
    """Return whether the synthetic release-gate evidence fits the ceiling."""
    return record.status in {"reviewed", "approved", "released", "closed"} and observed_p95_ms <= record.latency_budget_ms

if __name__ == "__main__":
    print({"record": RECORD.record_id, "adr": RECORD.adr_id, "ready": is_ready(RECORD)})
