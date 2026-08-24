"""Synthetic adapter for Product Alpha R7 contract API-A7-3623."""
from dataclasses import dataclass

@dataclass(frozen=True)
class ApiContract:
    contract_id: str = "API-A7-3623"
    resource: str = "evidence-snapshots"
    operation: str = "reconcile"
    target_p95_ms: int = 200

def make_reviewed_record(record_id: str) -> dict[str, object]:
    spec = ApiContract()
    return {"record_id": record_id, "contract_id": spec.contract_id, "release": "ALPHA-R7", "state": "reviewed"}
