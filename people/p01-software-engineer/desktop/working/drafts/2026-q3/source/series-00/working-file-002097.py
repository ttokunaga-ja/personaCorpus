"""Ephemeral desktop draft for p01-src-002097; synthetic only."""
from dataclasses import dataclass

@dataclass(frozen=True)
class WorkingObservation:
    issue_id: str = "ALW-3097"
    service: str = "svc-event-bus"
    latency_ms: int = 189
    state: str = "working"

def within_alpha_reference(observation: WorkingObservation) -> bool:
    return observation.latency_ms <= 200

if __name__ == "__main__":
    current = WorkingObservation()
    print({"source": "p01-src-002097", "within_slo": within_alpha_reference(current)})
