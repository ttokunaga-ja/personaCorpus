"""Ephemeral desktop draft for p01-src-002058; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-3058"
    service: str = "svc-edge-api"
    latency_ms: int = 187
    state: str = "review-pending"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-002058", "within_slo": within_alpha_reference(current)})
