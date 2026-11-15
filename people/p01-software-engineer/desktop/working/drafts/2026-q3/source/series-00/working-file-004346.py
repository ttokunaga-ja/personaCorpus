"""Ephemeral desktop draft for p01-src-004346; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-5346"
    service: str = "svc-decision-store"
    latency_ms: int = 181
    state: str = "review-pending"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-004346", "within_slo": within_alpha_reference(current)})
