"""Synthetic lifecycle policy for decision p01-src-007246."""
from dataclasses import dataclass

@dataclass(frozen=True)
class LifecycleDecision:
    record_id: str
    adr_id: str
    state: str
    product: str

DECISION = LifecycleDecision("p01-src-007246", "ADR-B4-083", "reviewed", "Product Beta R4")

def can_publish(decision: LifecycleDecision, observed_p95_ms: int = 184) -> bool:
    valid_states = {"reviewed", "approved", "released", "closed"}
    return decision.state in valid_states and observed_p95_ms <= 200

if __name__ == "__main__":
    print({"record_id": DECISION.record_id, "adr_id": DECISION.adr_id, "publishable": can_publish(DECISION)})
